use std::fmt::Display;
use std::ops::RangeInclusive;

use chrono::{DateTime, Datelike, Days, Duration, NaiveDate, Utc};
use rmcp::ErrorData as McpError;
use rmcp::model::{CallToolResult, ContentBlock};
use tapo::DiscoveryResult;
use tapo::requests::{EnergyDataInterval, PowerDataInterval};
use tapo::responses::ChildDeviceHubIrResult;
use tracing::field::Empty;

use crate::config::AppConfig;
use crate::errors::TapoMcpError;
use crate::models::{
    DATA_YEARS, ENERGY_HOURLY_DAYS, EnergyDataIntervalRequest, GetCapabilityRequest,
    GetDeviceStateParams, POWER_5_MINUTE_HOURS, POWER_HOURLY_DAYS, PowerDataIntervalRequest,
    QUARTERS,
};
use crate::requests;
use crate::requests::{CheckedDevice, HubParent};
use crate::telemetry::run_in_span;

pub async fn get_device_state(
    config: &AppConfig,
    params: GetDeviceStateParams,
) -> Result<CallToolResult, McpError> {
    let checked = requests::check_device(config, &params.id, &params.ip).await?;

    let capability = params.capability.name();
    let span = || {
        tracing::info_span!(
            "get_capability",
            tapo.capability = capability,
            otel.status_code = Empty,
            "error.type" = Empty,
        )
    };
    let value = match params.capability {
        GetCapabilityRequest::Snapshot => {
            return Err(TapoMcpError::WrongTool {
                capability: capability.to_string(),
                tool: "take_snapshot".to_string(),
            }
            .into());
        }
        GetCapabilityRequest::DeviceInfo => run_in_span(span(), get_device_info(checked)).await?,
        GetCapabilityRequest::EnergyData { interval } => {
            run_in_span(span(), get_energy_data(&params.id, checked, interval)).await?
        }
        GetCapabilityRequest::PowerData { interval } => {
            run_in_span(span(), get_power_data(&params.id, checked, interval)).await?
        }
        GetCapabilityRequest::TriggerLogs {
            page_size,
            start_id,
        } => {
            run_in_span(
                span(),
                get_trigger_logs(&params.id, checked, page_size, start_id),
            )
            .await?
        }
        GetCapabilityRequest::TemperatureHumidityRecords => {
            run_in_span(
                span(),
                get_temperature_humidity_records(&params.id, checked),
            )
            .await?
        }
    };

    let content = vec![ContentBlock::json(value)?];
    Ok(CallToolResult::success(content))
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
    checked: CheckedDevice,
    interval: EnergyDataIntervalRequest,
) -> Result<serde_json::Value, TapoMcpError> {
    let interval = energy_data_interval(interval)?;

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
            capability: "EnergyData".to_string(),
            expected: ENERGY_MONITORING_DEVICES.to_string(),
        }),
    }
}

async fn get_power_data(
    id: &str,
    checked: CheckedDevice,
    interval: PowerDataIntervalRequest,
) -> Result<serde_json::Value, TapoMcpError> {
    let interval = power_data_interval(interval)?;

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
            capability: "PowerData".to_string(),
            expected: ENERGY_MONITORING_DEVICES.to_string(),
        }),
    }
}

const ENERGY_MONITORING_DEVICES: &str =
    "an energy-monitoring plug (P110, P110M, P115) or power strip outlet (P304M, P316M)";

/// Turns the request into one of the date ranges the device accepts.
fn energy_data_interval(
    interval: EnergyDataIntervalRequest,
) -> Result<EnergyDataInterval, TapoMcpError> {
    match interval {
        EnergyDataIntervalRequest::Hourly { start_date, days } => {
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
        EnergyDataIntervalRequest::Daily { year, quarter } => {
            check_range("quarter", quarter, QUARTERS)?;
            let month = u32::from(quarter - 1) * 3 + 1;
            Ok(EnergyDataInterval::Daily {
                start_date: first_day_of_month(year, month)?,
            })
        }
        EnergyDataIntervalRequest::Monthly { year } => Ok(EnergyDataInterval::Monthly {
            start_date: first_day_of_month(year, 1)?,
        }),
    }
}

/// Turns the request into a range the device returns in full.
fn power_data_interval(
    interval: PowerDataIntervalRequest,
) -> Result<PowerDataInterval, TapoMcpError> {
    match interval {
        PowerDataIntervalRequest::Every5Minutes {
            start_date_time,
            hours,
        } => {
            check_range("hours", hours, POWER_5_MINUTE_HOURS)?;
            let end_date_time = power_data_end(start_date_time, Duration::hours(i64::from(hours)))?;
            Ok(PowerDataInterval::Every5Minutes {
                start_date_time,
                end_date_time,
            })
        }
        PowerDataIntervalRequest::Hourly {
            start_date_time,
            days,
        } => {
            check_range("days", days, POWER_HOURLY_DAYS)?;
            let end_date_time = power_data_end(start_date_time, Duration::days(i64::from(days)))?;
            Ok(PowerDataInterval::Hourly {
                start_date_time,
                end_date_time,
            })
        }
    }
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
            capability: "TriggerLogs".to_string(),
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
            capability: "TriggerLogs".to_string(),
            expected: "a trigger-based sensor (S200B/D, T100, T110, T300)".to_string(),
        }),
    }
}

async fn get_temperature_humidity_records(
    id: &str,
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
            capability: "TemperatureHumidityRecords".to_string(),
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
            capability: "TemperatureHumidityRecords".to_string(),
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
    fn energy_data_hourly_ends_on_the_last_day() {
        let interval = energy_data_interval(EnergyDataIntervalRequest::Hourly {
            start_date: date(2026, 10, 10),
            days: 8,
        })
        .unwrap();

        assert!(matches!(
            interval,
            EnergyDataInterval::Hourly { start_date, end_date }
                if start_date == date(2026, 10, 10) && end_date == date(2026, 10, 17)
        ));
    }

    #[test]
    fn energy_data_daily_starts_on_the_first_day_of_the_quarter() {
        for (quarter, month) in [(1, 1), (2, 4), (3, 7), (4, 10)] {
            let interval = energy_data_interval(EnergyDataIntervalRequest::Daily {
                year: 2026,
                quarter,
            })
            .unwrap();

            assert!(matches!(
                interval,
                EnergyDataInterval::Daily { start_date } if start_date == date(2026, month, 1)
            ));
        }
    }

    #[test]
    fn energy_data_rejects_out_of_range_parameters() {
        let requests = [
            EnergyDataIntervalRequest::Hourly {
                start_date: date(2026, 10, 10),
                days: 0,
            },
            EnergyDataIntervalRequest::Hourly {
                start_date: date(2026, 10, 10),
                days: 9,
            },
            EnergyDataIntervalRequest::Daily {
                year: 2026,
                quarter: 0,
            },
            EnergyDataIntervalRequest::Daily {
                year: 2026,
                quarter: 5,
            },
            EnergyDataIntervalRequest::Hourly {
                start_date: date(1999, 12, 31),
                days: 1,
            },
            EnergyDataIntervalRequest::Hourly {
                start_date: date(9999, 12, 31),
                days: 2,
            },
            EnergyDataIntervalRequest::Daily {
                year: 1969,
                quarter: 4,
            },
            EnergyDataIntervalRequest::Monthly { year: 10000 },
            EnergyDataIntervalRequest::Monthly { year: i32::MAX },
        ];
        for request in requests {
            assert!(matches!(
                energy_data_interval(request),
                Err(TapoMcpError::InvalidParameter { .. })
            ));
        }
    }

    #[test]
    fn power_data_ends_after_the_requested_duration() {
        let start = "2026-10-17T08:00:00Z".parse::<DateTime<Utc>>().unwrap();

        let interval = power_data_interval(PowerDataIntervalRequest::Every5Minutes {
            start_date_time: start,
            hours: 12,
        })
        .unwrap();
        assert!(matches!(
            interval,
            PowerDataInterval::Every5Minutes { end_date_time, .. }
                if end_date_time == start + Duration::hours(12)
        ));

        let interval = power_data_interval(PowerDataIntervalRequest::Hourly {
            start_date_time: start,
            days: 6,
        })
        .unwrap();
        assert!(matches!(
            interval,
            PowerDataInterval::Hourly { end_date_time, .. }
                if end_date_time == start + Duration::days(6)
        ));
    }

    #[test]
    fn power_data_rejects_out_of_range_parameters() {
        let start = "2026-10-17T08:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let requests = [
            PowerDataIntervalRequest::Every5Minutes {
                start_date_time: start,
                hours: 0,
            },
            PowerDataIntervalRequest::Every5Minutes {
                start_date_time: start,
                hours: 13,
            },
            PowerDataIntervalRequest::Hourly {
                start_date_time: start,
                days: 0,
            },
            PowerDataIntervalRequest::Hourly {
                start_date_time: start,
                days: 7,
            },
            PowerDataIntervalRequest::Every5Minutes {
                start_date_time: "1960-01-01T00:00:00Z".parse().unwrap(),
                hours: 1,
            },
            PowerDataIntervalRequest::Every5Minutes {
                start_date_time: "9999-12-31T23:00:00Z".parse().unwrap(),
                hours: 2,
            },
            PowerDataIntervalRequest::Hourly {
                start_date_time: DateTime::<Utc>::MAX_UTC - Duration::hours(1),
                days: 1,
            },
        ];
        for request in requests {
            assert!(matches!(
                power_data_interval(request),
                Err(TapoMcpError::InvalidParameter { .. })
            ));
        }
    }
}
