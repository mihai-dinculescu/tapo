use std::fmt::Display;
use std::ops::RangeInclusive;

use chrono::{DateTime, Datelike, Days, Duration, NaiveDate, Utc};
use rmcp::model::{CallToolResult, ContentBlock};
use tapo::DiscoveryResult;
use tapo::requests::{EnergyDataInterval, PowerDataInterval};
use tapo::responses::ChildDeviceHubIrResult;
use tracing::field::Empty;

use crate::config::AppConfig;
use crate::device_locks::DeviceLocks;
use crate::errors::TapoMcpError;
use crate::models::{
    DATA_YEARS, DETECTIONS_WINDOW_HOURS, DetectionsResult, ENERGY_HOURLY_DAYS,
    GetCapabilityRequest, GetDeviceStateParams, POWER_5_MINUTE_HOURS, POWER_HOURLY_DAYS, QUARTERS,
};
use crate::requests;
use crate::requests::{CheckedDevice, HubParent};
use crate::telemetry::run_in_span;

pub async fn get_device_state(
    config: &AppConfig,
    locks: &DeviceLocks,
    params: GetDeviceStateParams,
) -> Result<CallToolResult, TapoMcpError> {
    let capability = params.capability.name();
    // Checked before the device is contacted, so that a bad parameter fails fast.
    let read = read_request(params.capability)?;

    let value = requests::with_device(config, locks, &params.id, &params.ip, async |checked| {
        let span = tracing::info_span!(
            "get_capability",
            tapo.capability = capability,
            otel.status_code = Empty,
            "error.type" = Empty,
        );
        match read {
            ReadRequest::Detections {
                start_date_time,
                end_date_time,
            } => {
                run_in_span(
                    span,
                    get_detections(
                        &params.id,
                        capability,
                        checked,
                        start_date_time,
                        end_date_time,
                    ),
                )
                .await
            }
            ReadRequest::DeviceInfo => run_in_span(span, get_device_info(checked)).await,
            ReadRequest::EnergyData(interval) => {
                run_in_span(
                    span,
                    get_energy_data(&params.id, capability, checked, interval),
                )
                .await
            }
            ReadRequest::EnergyMeterData => {
                run_in_span(span, get_energy_meter_data(&params.id, capability, checked)).await
            }
            ReadRequest::PowerData(interval) => {
                run_in_span(
                    span,
                    get_power_data(&params.id, capability, checked, interval),
                )
                .await
            }
            ReadRequest::TriggerLogs {
                page_size,
                start_id,
            } => {
                run_in_span(
                    span,
                    get_trigger_logs(&params.id, capability, checked, page_size, start_id),
                )
                .await
            }
            ReadRequest::TemperatureHumidityRecords => {
                run_in_span(
                    span,
                    get_temperature_humidity_records(&params.id, capability, checked),
                )
                .await
            }
        }
    })
    .await?;

    let content = vec![ContentBlock::text(serde_json::to_string(&value)?)];
    Ok(CallToolResult::success(content))
}

/// A capability to read, with its parameters checked against what the device accepts.
enum ReadRequest {
    Detections {
        start_date_time: DateTime<Utc>,
        end_date_time: DateTime<Utc>,
    },
    DeviceInfo,
    EnergyData(EnergyDataInterval),
    EnergyMeterData,
    PowerData(PowerDataInterval),
    TemperatureHumidityRecords,
    TriggerLogs {
        page_size: u64,
        start_id: u64,
    },
}

/// Checks the parameters of `capability`.
fn read_request(capability: GetCapabilityRequest) -> Result<ReadRequest, TapoMcpError> {
    Ok(match capability {
        GetCapabilityRequest::Detections {
            start_date_time,
            end_date_time,
        } => {
            detections_window(start_date_time, end_date_time)?;
            ReadRequest::Detections {
                start_date_time,
                end_date_time,
            }
        }
        GetCapabilityRequest::DeviceInfo => ReadRequest::DeviceInfo,
        GetCapabilityRequest::EnergyDataHourly { start_date, days } => {
            ReadRequest::EnergyData(energy_data_hourly(start_date, days)?)
        }
        GetCapabilityRequest::EnergyDataDaily { year, quarter } => {
            ReadRequest::EnergyData(energy_data_daily(year, quarter)?)
        }
        GetCapabilityRequest::EnergyDataMonthly { year } => {
            ReadRequest::EnergyData(energy_data_monthly(year)?)
        }
        GetCapabilityRequest::EnergyMeterData => ReadRequest::EnergyMeterData,
        GetCapabilityRequest::PowerDataEvery5Minutes {
            start_date_time,
            hours,
        } => ReadRequest::PowerData(power_data_every_5_minutes(start_date_time, hours)?),
        GetCapabilityRequest::PowerDataHourly {
            start_date_time,
            days,
        } => ReadRequest::PowerData(power_data_hourly(start_date_time, days)?),
        GetCapabilityRequest::TemperatureHumidityRecords => ReadRequest::TemperatureHumidityRecords,
        GetCapabilityRequest::TriggerLogs {
            page_size,
            start_id,
        } => ReadRequest::TriggerLogs {
            page_size,
            start_id,
        },
    })
}

async fn get_detections(
    id: &str,
    capability: &str,
    checked: CheckedDevice,
    start_date_time: DateTime<Utc>,
    end_date_time: DateTime<Utc>,
) -> Result<serde_json::Value, TapoMcpError> {
    let detections = match checked {
        CheckedDevice::Parent(DiscoveryResult::Camera { handler, .. }) => {
            handler
                .get_detections(start_date_time, end_date_time)
                .await?
        }
        CheckedDevice::Parent(DiscoveryResult::CameraPtz { handler, .. }) => {
            handler
                .get_detections(start_date_time, end_date_time)
                .await?
        }
        _ => {
            return Err(TapoMcpError::WrongDeviceType {
                id: id.to_string(),
                capability: capability.to_string(),
                expected: "a camera".to_string(),
            });
        }
    };
    Ok(serde_json::to_value(&DetectionsResult { detections })?)
}

/// Checks that the detections window runs forward, within [`DATA_YEARS`], for
/// at most [`DETECTIONS_WINDOW_HOURS`].
fn detections_window(
    start_date_time: DateTime<Utc>,
    end_date_time: DateTime<Utc>,
) -> Result<(), TapoMcpError> {
    if !DATA_YEARS.contains(&start_date_time.year()) {
        return Err(year_out_of_range("start_date_time"));
    }
    if !DATA_YEARS.contains(&end_date_time.year()) {
        return Err(year_out_of_range("end_date_time"));
    }
    if end_date_time < start_date_time {
        return Err(invalid_parameter(
            "end_date_time",
            "must not be before start_date_time",
        ));
    }
    if end_date_time - start_date_time > Duration::hours(DETECTIONS_WINDOW_HOURS) {
        return Err(invalid_parameter(
            "end_date_time",
            &format!("must be at most {DETECTIONS_WINDOW_HOURS} hours after start_date_time"),
        ));
    }
    Ok(())
}

async fn get_device_info(checked: CheckedDevice) -> Result<serde_json::Value, TapoMcpError> {
    match checked {
        CheckedDevice::Parent(device) => match device {
            DiscoveryResult::Light { device_info, .. } => Ok(serde_json::to_value(&*device_info)?),
            DiscoveryResult::ColorLight { device_info, .. } => {
                Ok(serde_json::to_value(&*device_info)?)
            }
            DiscoveryResult::RgbLightStrip { device_info, .. } => {
                Ok(serde_json::to_value(&*device_info)?)
            }
            DiscoveryResult::RgbicLightStrip { device_info, .. } => {
                Ok(serde_json::to_value(&*device_info)?)
            }
            DiscoveryResult::Plug { device_info, .. } => Ok(serde_json::to_value(&*device_info)?),
            DiscoveryResult::PlugEnergyMonitoring { device_info, .. } => {
                Ok(serde_json::to_value(&*device_info)?)
            }
            DiscoveryResult::PowerStrip { device_info, .. } => {
                Ok(serde_json::to_value(&*device_info)?)
            }
            DiscoveryResult::PowerStripEnergyMonitoring { device_info, .. } => {
                Ok(serde_json::to_value(&*device_info)?)
            }
            DiscoveryResult::Hub { device_info, .. } => Ok(serde_json::to_value(&*device_info)?),
            DiscoveryResult::HubIr { device_info, .. } => Ok(serde_json::to_value(&*device_info)?),
            DiscoveryResult::CameraHub { device_info, .. } => {
                Ok(serde_json::to_value(&*device_info)?)
            }
            DiscoveryResult::Camera { device_info, .. }
            | DiscoveryResult::CameraPtz { device_info, .. } => {
                Ok(serde_json::to_value(&*device_info)?)
            }
            DiscoveryResult::Other { device_info, .. } => Ok(serde_json::to_value(&*device_info)?),
            // `DiscoveryResult` is non-exhaustive. Every type it has today is handled above.
            _ => unreachable!(),
        },
        CheckedDevice::PowerStripChild { handler, child_id } => {
            let plug = handler.plug_unchecked(child_id);
            let info = plug.get_device_info().await?;
            Ok(serde_json::to_value(&info)?)
        }
        CheckedDevice::PowerStripEnergyMonitoringChild { handler, child_id } => {
            let plug = handler.plug_unchecked(child_id);
            let info = plug.get_device_info().await?;
            Ok(serde_json::to_value(&info)?)
        }
        CheckedDevice::HubChild { child, .. } => Ok(serde_json::to_value(&child)?),
    }
}

async fn get_energy_data(
    id: &str,
    capability: &str,
    checked: CheckedDevice,
    interval: EnergyDataInterval,
) -> Result<serde_json::Value, TapoMcpError> {
    match checked {
        CheckedDevice::Parent(DiscoveryResult::PlugEnergyMonitoring { handler, .. }) => Ok(
            serde_json::to_value(&handler.get_energy_data(interval, None).await?)?,
        ),
        CheckedDevice::PowerStripEnergyMonitoringChild { handler, child_id } => {
            let plug = handler.plug_unchecked(child_id);
            Ok(serde_json::to_value(
                &plug.get_energy_data(interval, None).await?,
            )?)
        }
        _ => Err(TapoMcpError::WrongDeviceType {
            id: id.to_string(),
            capability: capability.to_string(),
            expected: ENERGY_MONITORING_DEVICES.to_string(),
        }),
    }
}

async fn get_energy_meter_data(
    id: &str,
    capability: &str,
    checked: CheckedDevice,
) -> Result<serde_json::Value, TapoMcpError> {
    match checked {
        CheckedDevice::Parent(DiscoveryResult::PlugEnergyMonitoring { handler, .. }) => Ok(
            serde_json::to_value(&handler.get_energy_meter_data().await?)?,
        ),
        CheckedDevice::PowerStripEnergyMonitoringChild { handler, child_id } => {
            let plug = handler.plug_unchecked(child_id);
            Ok(serde_json::to_value(&plug.get_energy_meter_data().await?)?)
        }
        _ => Err(TapoMcpError::WrongDeviceType {
            id: id.to_string(),
            capability: capability.to_string(),
            expected: ENERGY_MONITORING_DEVICES.to_string(),
        }),
    }
}

async fn get_power_data(
    id: &str,
    capability: &str,
    checked: CheckedDevice,
    interval: PowerDataInterval,
) -> Result<serde_json::Value, TapoMcpError> {
    match checked {
        CheckedDevice::Parent(DiscoveryResult::PlugEnergyMonitoring { handler, .. }) => Ok(
            serde_json::to_value(&handler.get_power_data(interval).await?)?,
        ),
        CheckedDevice::PowerStripEnergyMonitoringChild { handler, child_id } => {
            let plug = handler.plug_unchecked(child_id);
            Ok(serde_json::to_value(&plug.get_power_data(interval).await?)?)
        }
        _ => Err(TapoMcpError::WrongDeviceType {
            id: id.to_string(),
            capability: capability.to_string(),
            expected: ENERGY_MONITORING_DEVICES.to_string(),
        }),
    }
}

const ENERGY_MONITORING_DEVICES: &str =
    "an energy-monitoring plug (P110, P110M, P115) or power strip child plug (P304M, P316M)";

/// Reads hourly energy data for `days` whole days, from `start_date`.
fn energy_data_hourly(start_date: NaiveDate, days: u8) -> Result<EnergyDataInterval, TapoMcpError> {
    check_range("days", days, ENERGY_HOURLY_DAYS)?;
    let end_date = start_date
        .checked_add_days(Days::new(u64::from(days) - 1))
        .filter(|end_date| DATA_YEARS.contains(&end_date.year()));
    let (true, Some(end_date)) = (DATA_YEARS.contains(&start_date.year()), end_date) else {
        return Err(year_out_of_range("start_date"));
    };
    Ok(EnergyDataInterval::Hourly {
        start_date,
        end_date,
    })
}

/// Reads daily energy data for a quarter, which the device starts on its first day.
fn energy_data_daily(year: i32, quarter: u8) -> Result<EnergyDataInterval, TapoMcpError> {
    check_range("quarter", quarter, QUARTERS)?;
    let month = u32::from(quarter - 1) * 3 + 1;
    Ok(EnergyDataInterval::Daily {
        start_date: first_day_of_month(year, month)?,
    })
}

/// Reads monthly energy data for a year, which the device starts on its first day.
fn energy_data_monthly(year: i32) -> Result<EnergyDataInterval, TapoMcpError> {
    Ok(EnergyDataInterval::Monthly {
        start_date: first_day_of_month(year, 1)?,
    })
}

/// Reads power data every 5 minutes for `hours` hours, a range the device returns in full.
fn power_data_every_5_minutes(
    start_date_time: DateTime<Utc>,
    hours: u8,
) -> Result<PowerDataInterval, TapoMcpError> {
    check_range("hours", hours, POWER_5_MINUTE_HOURS)?;
    let end_date_time = power_data_end(start_date_time, Duration::hours(i64::from(hours)))?;
    Ok(PowerDataInterval::Every5Minutes {
        start_date_time,
        end_date_time,
    })
}

/// Reads hourly power data for `days` days, a range the device returns in full.
fn power_data_hourly(
    start_date_time: DateTime<Utc>,
    days: u8,
) -> Result<PowerDataInterval, TapoMcpError> {
    check_range("days", days, POWER_HOURLY_DAYS)?;
    let end_date_time = power_data_end(start_date_time, Duration::days(i64::from(days)))?;
    Ok(PowerDataInterval::Hourly {
        start_date_time,
        end_date_time,
    })
}

/// Adds `duration` to `start_date_time`, if both ends fall within [`DATA_YEARS`].
fn power_data_end(
    start_date_time: DateTime<Utc>,
    duration: Duration,
) -> Result<DateTime<Utc>, TapoMcpError> {
    start_date_time
        .checked_add_signed(duration)
        .filter(|end_date_time| {
            DATA_YEARS.contains(&start_date_time.year())
                && DATA_YEARS.contains(&end_date_time.year())
        })
        .ok_or_else(|| year_out_of_range("start_date_time"))
}

fn first_day_of_month(year: i32, month: u32) -> Result<NaiveDate, TapoMcpError> {
    check_range("year", year, DATA_YEARS)?;
    NaiveDate::from_ymd_opt(year, month, 1).ok_or_else(|| year_out_of_range("year"))
}

fn check_range<T: PartialOrd + Display>(
    name: &str,
    value: T,
    range: RangeInclusive<T>,
) -> Result<(), TapoMcpError> {
    if range.contains(&value) {
        Ok(())
    } else {
        Err(invalid_parameter(
            name,
            &format!(
                "must be between {} and {}, got {value}",
                range.start(),
                range.end()
            ),
        ))
    }
}

fn year_out_of_range(name: &str) -> TapoMcpError {
    invalid_parameter(
        name,
        &format!(
            "must fall in the years {} to {}",
            DATA_YEARS.start(),
            DATA_YEARS.end()
        ),
    )
}

fn invalid_parameter(name: &str, message: &str) -> TapoMcpError {
    TapoMcpError::InvalidParameter {
        name: name.to_string(),
        message: message.to_string(),
    }
}

async fn get_trigger_logs(
    id: &str,
    capability: &str,
    checked: CheckedDevice,
    page_size: u64,
    start_id: u64,
) -> Result<serde_json::Value, TapoMcpError> {
    let CheckedDevice::HubChild {
        hub,
        child_id,
        child,
    } = checked
    else {
        return Err(TapoMcpError::WrongDeviceType {
            id: id.to_string(),
            capability: capability.to_string(),
            expected: "a hub child device".to_string(),
        });
    };

    macro_rules! trigger_logs {
        ($constructor:ident) => {{
            let h = match hub {
                HubParent::Hub(hub) => hub.$constructor(child_id),
                HubParent::HubIr(hub) => hub.$constructor(child_id),
                HubParent::CameraHub(hub) => hub.$constructor(child_id),
            };
            Ok(serde_json::to_value(
                &h.get_trigger_logs(page_size, start_id).await?,
            )?)
        }};
    }

    match child {
        ChildDeviceHubIrResult::S200(_) => trigger_logs!(s200_unchecked),
        ChildDeviceHubIrResult::T100(_) => trigger_logs!(t100_unchecked),
        ChildDeviceHubIrResult::T110(_) => trigger_logs!(t110_unchecked),
        ChildDeviceHubIrResult::T300(_) => trigger_logs!(t300_unchecked),
        _ => Err(TapoMcpError::WrongDeviceType {
            id: id.to_string(),
            capability: capability.to_string(),
            expected: "a trigger-based sensor (S200B/D, T100, T110, T300)".to_string(),
        }),
    }
}

async fn get_temperature_humidity_records(
    id: &str,
    capability: &str,
    checked: CheckedDevice,
) -> Result<serde_json::Value, TapoMcpError> {
    let CheckedDevice::HubChild {
        hub,
        child_id,
        child,
    } = checked
    else {
        return Err(TapoMcpError::WrongDeviceType {
            id: id.to_string(),
            capability: capability.to_string(),
            expected: "a hub child device".to_string(),
        });
    };

    match child {
        ChildDeviceHubIrResult::T31X(_) => {
            let h = match hub {
                HubParent::Hub(hub) => hub.t31x_unchecked(child_id),
                HubParent::HubIr(hub) => hub.t31x_unchecked(child_id),
                HubParent::CameraHub(hub) => hub.t31x_unchecked(child_id),
            };
            Ok(serde_json::to_value(
                &h.get_temperature_humidity_records().await?,
            )?)
        }
        _ => Err(TapoMcpError::WrongDeviceType {
            id: id.to_string(),
            capability: capability.to_string(),
            expected: "a T310 or T315 sensor".to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn detections_window_accepts_up_to_a_day() {
        let start = "2026-10-17T08:00:00Z".parse::<DateTime<Utc>>().unwrap();

        assert!(detections_window(start, start).is_ok());
        assert!(detections_window(start, start + Duration::minutes(30)).is_ok());
        assert!(detections_window(start, start + Duration::hours(24)).is_ok());
    }

    #[test]
    fn detections_window_rejects_out_of_range_parameters() {
        let start = "2026-10-17T08:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let results = [
            detections_window(start, start - Duration::seconds(1)),
            detections_window(start, start + Duration::hours(24) + Duration::seconds(1)),
            detections_window("1999-12-31T23:00:00Z".parse().unwrap(), start),
            detections_window(start, DateTime::<Utc>::MAX_UTC),
        ];
        for result in results {
            assert!(matches!(result, Err(TapoMcpError::InvalidParameter { .. })));
        }
    }

    #[test]
    fn energy_data_hourly_ends_on_the_last_day() {
        let interval = energy_data_hourly(date(2026, 10, 10), 8).unwrap();

        assert!(matches!(
            interval,
            EnergyDataInterval::Hourly { start_date, end_date }
                if start_date == date(2026, 10, 10) && end_date == date(2026, 10, 17)
        ));
    }

    #[test]
    fn energy_data_daily_starts_on_the_first_day_of_the_quarter() {
        for (quarter, month) in [(1, 1), (2, 4), (3, 7), (4, 10)] {
            let interval = energy_data_daily(2026, quarter).unwrap();

            assert!(matches!(
                interval,
                EnergyDataInterval::Daily { start_date } if start_date == date(2026, month, 1)
            ));
        }
    }

    #[test]
    fn energy_data_rejects_out_of_range_parameters() {
        let results = [
            energy_data_hourly(date(2026, 10, 10), 0),
            energy_data_hourly(date(2026, 10, 10), 9),
            energy_data_daily(2026, 0),
            energy_data_daily(2026, 5),
            energy_data_hourly(date(1999, 12, 31), 1),
            energy_data_hourly(date(9999, 12, 31), 2),
            energy_data_daily(1969, 4),
            energy_data_monthly(10000),
            energy_data_monthly(i32::MAX),
        ];
        for result in results {
            assert!(matches!(result, Err(TapoMcpError::InvalidParameter { .. })));
        }
    }

    #[test]
    fn power_data_ends_after_the_requested_duration() {
        let start = "2026-10-17T08:00:00Z".parse::<DateTime<Utc>>().unwrap();

        let interval = power_data_every_5_minutes(start, 12).unwrap();
        assert!(matches!(
            interval,
            PowerDataInterval::Every5Minutes { end_date_time, .. }
                if end_date_time == start + Duration::hours(12)
        ));

        let interval = power_data_hourly(start, 6).unwrap();
        assert!(matches!(
            interval,
            PowerDataInterval::Hourly { end_date_time, .. }
                if end_date_time == start + Duration::days(6)
        ));
    }

    #[test]
    fn power_data_rejects_out_of_range_parameters() {
        let start = "2026-10-17T08:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let results = [
            power_data_every_5_minutes(start, 0),
            power_data_every_5_minutes(start, 13),
            power_data_hourly(start, 0),
            power_data_hourly(start, 7),
            power_data_every_5_minutes("1960-01-01T00:00:00Z".parse().unwrap(), 1),
            power_data_every_5_minutes("9999-12-31T23:00:00Z".parse().unwrap(), 2),
            power_data_hourly(DateTime::<Utc>::MAX_UTC - Duration::hours(1), 1),
        ];
        for result in results {
            assert!(matches!(result, Err(TapoMcpError::InvalidParameter { .. })));
        }
    }
}
