/// Generates common handler boilerplate for Tapo device handlers.
///
/// # Usage
///
/// ```ignore
/// tapo_handler! {
///     /// Doc comment for the handler.
///     Handler(DeviceInfoResult),
///     on_off,
///     device_usage = DeviceUsageResult,
///     device_management,
/// }
/// ```
///
/// All options (`on_off`, `device_usage`, `device_management`) are independently optional.
///
/// # Generated code
///
/// * `#[derive(Debug)]` struct with `client: Arc<RwLock<ApiClient>>` field
/// * `new(client)` constructor
/// * `refresh_session()` method
/// * `get_device_info()` method (typed)
/// * `get_device_info_json()` method
/// * `on()` and `off()` methods (if `on_off` specified)
/// * `get_device_usage()` method (if `device_usage = Type` specified)
/// * `device_reboot()` and `device_reset()` methods (if `device_management` specified)
/// * `impl HandlerExt` with `get_client()`
macro_rules! tapo_handler {
    // With on_off + device_usage + device_management
    (
        $(#[$meta:meta])*
        $name:ident($device_info:ty),
        on_off,
        device_usage = $device_usage:ty,
        device_management,
    ) => {
        tapo_handler!(@base $(#[$meta])* $name($device_info));
        tapo_handler!(@on_off $name);
        tapo_handler!(@device_usage $name, $device_usage);
        tapo_handler!(@device_management $name);
    };

    // With on_off + device_usage only
    (
        $(#[$meta:meta])*
        $name:ident($device_info:ty),
        on_off,
        device_usage = $device_usage:ty,
    ) => {
        tapo_handler!(@base $(#[$meta])* $name($device_info));
        tapo_handler!(@on_off $name);
        tapo_handler!(@device_usage $name, $device_usage);
    };

    // With on_off + device_management only
    (
        $(#[$meta:meta])*
        $name:ident($device_info:ty),
        on_off,
        device_management,
    ) => {
        tapo_handler!(@base $(#[$meta])* $name($device_info));
        tapo_handler!(@on_off $name);
        tapo_handler!(@device_management $name);
    };

    // With on_off only
    (
        $(#[$meta:meta])*
        $name:ident($device_info:ty),
        on_off,
    ) => {
        tapo_handler!(@base $(#[$meta])* $name($device_info));
        tapo_handler!(@on_off $name);
    };

    // With device_usage + device_management
    (
        $(#[$meta:meta])*
        $name:ident($device_info:ty),
        device_usage = $device_usage:ty,
        device_management,
    ) => {
        tapo_handler!(@base $(#[$meta])* $name($device_info));
        tapo_handler!(@device_usage $name, $device_usage);
        tapo_handler!(@device_management $name);
    };

    // With device_usage only
    (
        $(#[$meta:meta])*
        $name:ident($device_info:ty),
        device_usage = $device_usage:ty,
    ) => {
        tapo_handler!(@base $(#[$meta])* $name($device_info));
        tapo_handler!(@device_usage $name, $device_usage);
    };

    // With device_management only
    (
        $(#[$meta:meta])*
        $name:ident($device_info:ty),
        device_management,
    ) => {
        tapo_handler!(@base $(#[$meta])* $name($device_info));
        tapo_handler!(@device_management $name);
    };

    // No options
    (
        $(#[$meta:meta])*
        $name:ident($device_info:ty),
    ) => {
        tapo_handler!(@base $(#[$meta])* $name($device_info));
    };

    // Internal: base struct + core methods + HandlerExt
    (@base $(#[$meta:meta])* $name:ident($device_info:ty)) => {
        $(#[$meta])*
        #[derive(Debug)]
        pub struct $name {
            client: std::sync::Arc<tokio::sync::RwLock<crate::api::ApiClient>>,
        }

        impl $name {
            pub(crate) fn new(
                client: std::sync::Arc<tokio::sync::RwLock<crate::api::ApiClient>>,
            ) -> Self {
                Self { client }
            }

            /// Refreshes the authentication session.
            pub async fn refresh_session(&mut self) -> Result<&mut Self, crate::error::Error> {
                self.client.write().await.refresh_session().await?;
                Ok(self)
            }

            #[doc = concat!(
                "Returns *device info* as [`", stringify!($device_info), "`].\n",
                "It is not guaranteed to contain all the properties returned from the Tapo API.\n",
                "If the deserialization fails, or if a property that you care about it's not present, ",
                "try [`", stringify!($name), "::get_device_info_json`].",
            )]
            pub async fn get_device_info(&self) -> Result<$device_info, crate::error::Error> {
                self.client.read().await.get_device_info().await
            }

            /// Returns *device info* as [`serde_json::Value`].
            /// It contains all the properties returned from the Tapo API.
            #[cfg(feature = "debug")]
            pub async fn get_device_info_json(
                &self,
            ) -> Result<serde_json::Value, crate::error::Error> {
                self.client.read().await.get_device_info().await
            }

            /// Returns the *component list* of the device.
            #[cfg(feature = "debug")]
            pub async fn get_component_list(
                &self,
            ) -> Result<Vec<crate::responses::Component>, crate::error::Error> {
                self.client.read().await.get_component_list().await
            }
        }

        #[async_trait::async_trait]
        impl crate::api::HandlerExt for $name {
            async fn get_client(
                &self,
            ) -> tokio::sync::RwLockReadGuard<'_, dyn crate::api::ApiClientExt> {
                tokio::sync::RwLockReadGuard::map(
                    self.client.read().await,
                    |client: &crate::api::ApiClient| -> &dyn crate::api::ApiClientExt { client },
                )
            }
        }
    };

    // Internal: on_off
    (@on_off $name:ident) => {
        impl $name {
            /// Turns *on* the device.
            pub async fn on(&self) -> Result<(), crate::error::Error> {
                let json = serde_json::to_value(
                    crate::requests::GenericSetDeviceInfoParams::device_on(true)?,
                )?;
                crate::api::ApiClientExt::set_device_info(&*self.client.read().await, json).await
            }

            /// Turns *off* the device.
            pub async fn off(&self) -> Result<(), crate::error::Error> {
                let json = serde_json::to_value(
                    crate::requests::GenericSetDeviceInfoParams::device_on(false)?,
                )?;
                crate::api::ApiClientExt::set_device_info(&*self.client.read().await, json).await
            }
        }
    };

    // Internal: device_usage
    (@device_usage $name:ident, $device_usage:ty) => {
        impl $name {
            #[doc = concat!("Returns *device usage* as [`", stringify!($device_usage), "`].")]
            pub async fn get_device_usage(&self) -> Result<$device_usage, crate::error::Error> {
                self.client.read().await.get_device_usage().await
            }
        }
    };

    // Internal: device_management
    (@device_management $name:ident) => {
        impl $name {
            /// *Reboots* the device.
            ///
            /// Notes:
            /// * Using a very small delay (e.g. 0 seconds) may cause a `ConnectionReset` or `TimedOut` error as the device reboots immediately.
            /// * Using a larger delay (e.g. 2-3 seconds) allows the device to respond before rebooting, reducing the chance of errors.
            /// * With larger delays, the method completes successfully before the device reboots.
            ///   However, subsequent commands may fail if sent during the reboot process or before the device reconnects to the network.
            ///
            /// # Arguments
            ///
            /// * `delay_s` - The delay in seconds before the device is rebooted.
            pub async fn device_reboot(&self, delay_s: u16) -> Result<(), crate::error::Error> {
                crate::api::ApiClientExt::device_reboot(&*self.client.read().await, delay_s).await
            }

            /// *Hardware resets* the device.
            ///
            /// **Warning**: This action will reset the device to its factory settings.
            /// The connection to the Wi-Fi network and the Tapo app will be lost,
            /// and the device will need to be reconfigured.
            ///
            /// This feature is especially useful when the device is difficult to access
            /// and requires reconfiguration.
            pub async fn device_reset(&self) -> Result<(), crate::error::Error> {
                crate::api::ApiClientExt::device_reset(&*self.client.read().await).await
            }
        }
    };
}

/// Generates common handler boilerplate for Tapo child device handlers (hub sensors,
/// power strip plugs, etc.).
///
/// # Usage
///
/// ```ignore
/// tapo_child_handler! {
///     /// Doc comment for the handler.
///     ChildHandler(DeviceInfoResult),
///     on_off,
/// }
/// ```
///
/// The `on_off` option is optional.
///
/// # Generated code
///
/// * Struct with `client: Arc<RwLock<ApiClient>>` and `device_id: String` fields
/// * `new(client, device_id)` constructor
/// * `get_device_info()` method (typed)
/// * `get_device_info_json()` method
/// * `on()` and `off()` methods (if `on_off` specified)
macro_rules! tapo_child_handler {
    // With on_off
    (
        $(#[$meta:meta])*
        $name:ident($device_info:ty),
        on_off,
    ) => {
        tapo_child_handler!(@base $(#[$meta])* $name($device_info));
        tapo_child_handler!(@on_off $name);
    };

    // No options
    (
        $(#[$meta:meta])*
        $name:ident($device_info:ty),
    ) => {
        tapo_child_handler!(@base $(#[$meta])* $name($device_info));
    };

    // Internal: base struct + core methods
    (@base $(#[$meta:meta])* $name:ident($device_info:ty)) => {
        $(#[$meta])*
        pub struct $name {
            client: std::sync::Arc<tokio::sync::RwLock<crate::api::ApiClient>>,
            device_id: String,
        }

        impl $name {
            pub(crate) fn new(
                client: std::sync::Arc<tokio::sync::RwLock<crate::api::ApiClient>>,
                device_id: String,
            ) -> Self {
                Self { client, device_id }
            }

            #[doc = concat!(
                "Returns *device info* as [`", stringify!($device_info), "`].\n",
                "It is not guaranteed to contain all the properties returned from the Tapo API.\n",
                "If the deserialization fails, or if a property that you care about it's not present, ",
                "try [`", stringify!($name), "::get_device_info_json`].",
            )]
            pub async fn get_device_info(&self) -> Result<$device_info, crate::error::Error> {
                let request = crate::requests::TapoRequest::GetDeviceInfo(
                    crate::requests::TapoParams::new(crate::requests::EmptyParams),
                );

                self.client
                    .read()
                    .await
                    .control_child::<$device_info>(self.device_id.clone(), request)
                    .await?
                    .ok_or_else(|| {
                        crate::error::Error::Tapo(crate::error::TapoResponseError::EmptyResult)
                    })
                    .map(|result| crate::responses::DecodableResultExt::decode(result))?
            }

            /// Returns *device info* as [`serde_json::Value`].
            /// It contains all the properties returned from the Tapo API.
            #[cfg(feature = "debug")]
            pub async fn get_device_info_json(
                &self,
            ) -> Result<serde_json::Value, crate::error::Error> {
                let request = crate::requests::TapoRequest::GetDeviceInfo(
                    crate::requests::TapoParams::new(crate::requests::EmptyParams),
                );

                self.client
                    .read()
                    .await
                    .control_child::<serde_json::Value>(self.device_id.clone(), request)
                    .await?
                    .ok_or_else(|| {
                        crate::error::Error::Tapo(crate::error::TapoResponseError::EmptyResult)
                    })
            }

            /// Returns the *component list* of the device.
            #[cfg(feature = "debug")]
            pub async fn get_component_list(
                &self,
            ) -> Result<Vec<crate::responses::Component>, crate::error::Error> {
                let request = crate::requests::TapoRequest::ComponentNegotiation(
                    crate::requests::TapoParams::new(crate::requests::EmptyParams),
                );

                let result: crate::responses::ComponentListResult = self
                    .client
                    .read()
                    .await
                    .control_child(self.device_id.clone(), request)
                    .await?
                    .ok_or_else(|| {
                        crate::error::Error::Tapo(crate::error::TapoResponseError::EmptyResult)
                    })?;

                Ok(result.component_list)
            }
        }
    };

    // Internal: on_off for child devices
    (@on_off $name:ident) => {
        impl $name {
            /// Turns *on* the device.
            pub async fn on(&self) -> Result<(), crate::error::Error> {
                let json = serde_json::to_value(
                    crate::requests::GenericSetDeviceInfoParams::device_on(true)?,
                )?;
                let request = crate::requests::TapoRequest::SetDeviceInfo(
                    Box::new(crate::requests::TapoParams::new(json)),
                );

                self.client
                    .read()
                    .await
                    .control_child::<serde_json::Value>(self.device_id.clone(), request)
                    .await?;

                Ok(())
            }

            /// Turns *off* the device.
            pub async fn off(&self) -> Result<(), crate::error::Error> {
                let json = serde_json::to_value(
                    crate::requests::GenericSetDeviceInfoParams::device_on(false)?,
                )?;
                let request = crate::requests::TapoRequest::SetDeviceInfo(
                    Box::new(crate::requests::TapoParams::new(json)),
                );

                self.client
                    .read()
                    .await
                    .control_child::<serde_json::Value>(self.device_id.clone(), request)
                    .await?;

                Ok(())
            }
        }
    };
}
