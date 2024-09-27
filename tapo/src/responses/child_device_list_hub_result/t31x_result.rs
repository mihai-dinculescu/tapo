use chrono::{DateTime, Duration, Timelike, Utc};
use itertools::izip;
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::responses::{decode_value, DecodableResultExt, Status, TapoResponseExt};

/// Temperature unit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all, eq, eq_int))]
#[allow(missing_docs)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

/// Device info of Tapo T310 and T315 temperature and humidity sensors.
///
/// Specific properties: `current_temperature`, `temperature_unit`,
/// `current_temperature_exception`, `current_humidity`, `current_humidity_exception`,
/// `report_interval`, `last_onboarding_timestamp`, `status_follow_edge`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
#[allow(missing_docs)]
pub struct T31XResult {
    // Common properties to all Hub child devices.
    pub at_low_battery: bool,
    pub avatar: String,
    pub bind_count: u32,
    pub category: String,
    pub device_id: String,
    pub fw_ver: String,
    pub hw_id: String,
    pub hw_ver: String,
    pub jamming_rssi: i16,
    pub jamming_signal_level: u8,
    pub mac: String,
    pub nickname: String,
    pub oem_id: String,
    pub parent_device_id: String,
    pub region: String,
    pub rssi: i16,
    pub signal_level: u8,
    pub specs: String,
    pub status: Status,
    pub r#type: String,
    // Specific properties to this device.
    /// This value will be `0` when the current humidity is within the comfort zone.
    /// When the current humidity value falls outside the comfort zone, this value
    /// will be the difference between the current humidity and the lower or upper bound of the comfort zone.
    pub current_humidity_exception: i8,
    pub current_humidity: u8,
    /// This value will be `0.0` when the current temperature is within the comfort zone.
    /// When the current temperature value falls outside the comfort zone, this value
    /// will be the difference between the current temperature and the lower or upper bound of the comfort zone.
    #[serde(rename = "current_temp_exception")]
    pub current_temperature_exception: f32,
    #[serde(rename = "current_temp")]
    pub current_temperature: f32,
    #[serde(rename = "lastOnboardingTimestamp")]
    pub last_onboarding_timestamp: u64,
    /// The time in seconds between each report.
    pub report_interval: u32,
    pub status_follow_edge: bool,
    #[serde(rename = "temp_unit")]
    pub temperature_unit: TemperatureUnit,
}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl T31XResult {
    /// Gets all the properties of this result as a dictionary.
    pub fn to_dict(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyDict>> {
        let value = serde_json::to_value(self)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;

        crate::python::serde_object_to_py_dict(py, &value)
    }
}

impl TapoResponseExt for T31XResult {}

impl DecodableResultExt for T31XResult {
    fn decode(mut self) -> Result<Self, Error> {
        self.nickname = decode_value(&self.nickname)?;
        Ok(self)
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct TemperatureHumidityRecordsRaw {
    pub local_time: i64,
    pub past24h_humidity_exception: Vec<i16>,
    pub past24h_humidity: Vec<i16>,
    pub past24h_temp_exception: Vec<i16>,
    pub past24h_temp: Vec<i16>,
    pub temp_unit: TemperatureUnit,
}

impl TapoResponseExt for TemperatureHumidityRecordsRaw {}

/// Temperature and Humidity record as an average over a 15 minute interval.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
#[allow(missing_docs)]
pub struct TemperatureHumidityRecord {
    /// Record's DateTime in UTC.
    pub datetime: DateTime<Utc>,
    /// This value will be `0` when the current humidity is within the comfort zone.
    /// When the current humidity value falls outside the comfort zone, this value
    /// will be the difference between the current humidity and the lower or upper bound of the comfort zone.
    pub humidity_exception: i8,
    pub humidity: u8,
    /// This value will be `0.0` when the current temperature is within the comfort zone.
    /// When the current temperature value falls outside the comfort zone, this value
    /// will be the difference between the current temperature and the lower or upper bound of the comfort zone.
    pub temperature_exception: f32,
    pub temperature: f32,
}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl TemperatureHumidityRecord {
    /// Gets all the properties of this result as a dictionary.
    pub fn to_dict(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyDict>> {
        let value = serde_json::to_value(self)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;

        crate::python::serde_object_to_py_dict(py, &value)
    }
}

/// Temperature and Humidity records for the last 24 hours at 15 minute intervals.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all))]
#[allow(missing_docs)]
pub struct TemperatureHumidityRecords {
    /// The datetime in UTC of when this response was generated.
    pub datetime: DateTime<Utc>,
    pub records: Vec<TemperatureHumidityRecord>,
    pub temperature_unit: TemperatureUnit,
}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl TemperatureHumidityRecords {
    /// Gets all the properties of this result as a dictionary.
    pub fn to_dict(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyDict>> {
        let value = serde_json::to_value(self)
            .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;

        crate::python::serde_object_to_py_dict(py, &value)
    }
}

impl TryFrom<TemperatureHumidityRecordsRaw> for TemperatureHumidityRecords {
    type Error = anyhow::Error;

    fn try_from(raw: TemperatureHumidityRecordsRaw) -> Result<Self, Self::Error> {
        let datetime = DateTime::from_timestamp(raw.local_time, 0).unwrap_or_default();

        let interval_minute = if datetime.minute() >= 45 {
            45
        } else if datetime.minute() >= 30 {
            30
        } else if datetime.minute() >= 15 {
            15
        } else {
            0
        };

        let mut interval_time = datetime
            .with_minute(interval_minute)
            .unwrap_or_default()
            .with_second(0)
            .unwrap_or_default();

        let mut records = Vec::with_capacity(raw.past24h_temp.len());

        let iter = izip!(
            raw.past24h_humidity_exception.into_iter(),
            raw.past24h_humidity.into_iter(),
            raw.past24h_temp_exception.into_iter(),
            raw.past24h_temp.into_iter(),
        )
        .rev();

        for (humidity_exception, humidity, temperature_exception, temperature) in iter {
            if temperature != -1000
                && temperature_exception != -1000
                && humidity != -1000
                && humidity_exception != -1000
            {
                records.push(TemperatureHumidityRecord {
                    humidity_exception: humidity_exception as i8,
                    humidity: humidity as u8,
                    datetime: interval_time,
                    temperature_exception: temperature_exception as f32 / 10.0,
                    temperature: temperature as f32 / 10.0,
                });
            }

            interval_time = interval_time
                .checked_sub_signed(Duration::try_minutes(15).unwrap())
                .ok_or_else(|| anyhow::anyhow!("Failed to subtract from interval"))?;
        }

        records.reverse();

        Ok(Self {
            datetime,
            temperature_unit: raw.temp_unit,
            records,
        })
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;

    use super::*;

    #[test]
    fn test_temperature_humidity_records_parse() {
        let raw = TemperatureHumidityRecordsRaw {
            local_time: 1685371944,
            past24h_humidity_exception: vec![0, 0, 0, 0, 0, 0],
            past24h_humidity: vec![49, 50, 50, 55, 53, 52],
            past24h_temp_exception: vec![0, 0, 0, 0, 0, 0],
            past24h_temp: vec![196, 195, 194, 162, 164, 165],
            temp_unit: TemperatureUnit::Celsius,
        };

        let parsed = TemperatureHumidityRecords::try_from(raw).unwrap();

        assert_eq!(
            parsed.datetime,
            NaiveDateTime::parse_from_str("2023-05-29 14:52:24", "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .and_utc()
        );
        assert_eq!(parsed.temperature_unit, TemperatureUnit::Celsius);
        assert_eq!(parsed.records.len(), 6);
        assert_eq!(
            parsed.records[0],
            TemperatureHumidityRecord {
                humidity_exception: 0,
                humidity: 49,
                datetime: NaiveDateTime::parse_from_str("2023-05-29 13:30:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap()
                    .and_utc(),
                temperature_exception: 0.0,
                temperature: 19.6,
            }
        );
        assert_eq!(
            parsed.records[1],
            TemperatureHumidityRecord {
                humidity_exception: 0,
                humidity: 50,
                datetime: NaiveDateTime::parse_from_str("2023-05-29 13:45:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap()
                    .and_utc(),
                temperature_exception: 0.0,
                temperature: 19.5,
            }
        );
        assert_eq!(
            parsed.records[2],
            TemperatureHumidityRecord {
                humidity_exception: 0,
                humidity: 50,
                datetime: NaiveDateTime::parse_from_str("2023-05-29 14:00:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap()
                    .and_utc(),
                temperature_exception: 0.0,
                temperature: 19.4,
            }
        );
        assert_eq!(
            parsed.records[3],
            TemperatureHumidityRecord {
                humidity_exception: 0,
                humidity: 55,
                datetime: NaiveDateTime::parse_from_str("2023-05-29 14:15:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap()
                    .and_utc(),
                temperature_exception: 0.0,
                temperature: 16.2,
            }
        );
        assert_eq!(
            parsed.records[4],
            TemperatureHumidityRecord {
                humidity_exception: 0,
                humidity: 53,
                datetime: NaiveDateTime::parse_from_str("2023-05-29 14:30:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap()
                    .and_utc(),
                temperature_exception: 0.0,
                temperature: 16.4,
            }
        );
        assert_eq!(
            parsed.records[5],
            TemperatureHumidityRecord {
                humidity_exception: 0,
                humidity: 52,
                datetime: NaiveDateTime::parse_from_str("2023-05-29 14:45:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap()
                    .and_utc(),
                temperature_exception: 0.0,
                temperature: 16.5,
            }
        );
    }

    #[test]
    fn test_temperature_humidity_records_parse_in_progress() {
        let raw = TemperatureHumidityRecordsRaw {
            local_time: 1685371944,
            past24h_humidity_exception: vec![0, 0, 0, 0, 0, -1000],
            past24h_humidity: vec![49, 50, 50, 55, 53, -1000],
            past24h_temp_exception: vec![0, 0, 0, 0, 0, -1000],
            past24h_temp: vec![196, 195, 194, 162, 164, -1000],
            temp_unit: TemperatureUnit::Celsius,
        };

        let parsed = TemperatureHumidityRecords::try_from(raw).unwrap();

        assert_eq!(
            parsed.datetime,
            NaiveDateTime::parse_from_str("2023-05-29 14:52:24", "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .and_utc()
        );
        assert_eq!(parsed.temperature_unit, TemperatureUnit::Celsius);
        assert_eq!(parsed.records.len(), 5);
        assert_eq!(
            parsed.records[0],
            TemperatureHumidityRecord {
                humidity_exception: 0,
                humidity: 49,
                datetime: NaiveDateTime::parse_from_str("2023-05-29 13:30:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap()
                    .and_utc(),
                temperature_exception: 0.0,
                temperature: 19.6,
            }
        );
        assert_eq!(
            parsed.records[1],
            TemperatureHumidityRecord {
                humidity_exception: 0,
                humidity: 50,
                datetime: NaiveDateTime::parse_from_str("2023-05-29 13:45:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap()
                    .and_utc(),
                temperature_exception: 0.0,
                temperature: 19.5,
            }
        );
        assert_eq!(
            parsed.records[2],
            TemperatureHumidityRecord {
                humidity_exception: 0,
                humidity: 50,
                datetime: NaiveDateTime::parse_from_str("2023-05-29 14:00:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap()
                    .and_utc(),
                temperature_exception: 0.0,
                temperature: 19.4,
            }
        );
        assert_eq!(
            parsed.records[3],
            TemperatureHumidityRecord {
                humidity_exception: 0,
                humidity: 55,
                datetime: NaiveDateTime::parse_from_str("2023-05-29 14:15:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap()
                    .and_utc(),
                temperature_exception: 0.0,
                temperature: 16.2,
            }
        );
        assert_eq!(
            parsed.records[4],
            TemperatureHumidityRecord {
                humidity_exception: 0,
                humidity: 53,
                datetime: NaiveDateTime::parse_from_str("2023-05-29 14:30:00", "%Y-%m-%d %H:%M:%S")
                    .unwrap()
                    .and_utc(),
                temperature_exception: 0.0,
                temperature: 16.4,
            }
        );
    }
}
