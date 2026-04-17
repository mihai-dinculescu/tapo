//! Tapo response objects.

#[cfg(feature = "debug")]
mod child_device_component_list_result;
mod child_device_list_hub_result;
mod child_device_list_power_strip_result;
#[cfg(feature = "debug")]
mod component_list_result;
mod control_child_result;
mod current_power_result;
mod decodable_result_ext;
mod device_info_result;
mod device_usage_energy_monitoring_result;
mod device_usage_result;
mod energy_data_result;
mod energy_usage_result;
mod power_data_result;
mod preset;
mod rtsp_stream_url;
mod supported_alarm_type_list_result;
mod tapo_response;
mod tapo_result;
mod token_result;
mod trigger_logs_result;

pub use crate::requests::{LightingEffect, LightingEffectType};

#[cfg(feature = "debug")]
pub use child_device_component_list_result::*;
pub use child_device_list_hub_result::*;
pub use child_device_list_power_strip_result::*;
#[cfg(feature = "debug")]
pub use component_list_result::*;
pub use current_power_result::*;
pub use device_info_result::*;
pub use device_usage_energy_monitoring_result::*;
pub use device_usage_result::*;
pub use energy_data_result::*;
pub use energy_usage_result::*;
pub use power_data_result::*;
pub use preset::*;
pub use rtsp_stream_url::*;
pub use trigger_logs_result::*;

pub(crate) use control_child_result::*;
pub(crate) use decodable_result_ext::*;
#[cfg(feature = "debug")]
pub(crate) use supported_alarm_type_list_result::*;
pub(crate) use tapo_response::*;
pub(crate) use tapo_result::*;
pub(crate) use token_result::*;
