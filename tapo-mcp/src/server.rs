use std::sync::Arc;

use anyhow::Result;
use rmcp::handler::server::tool::ToolCallContext;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    CallToolRequestParams, CallToolResponse, CallToolResult, ListResourcesResult,
    PaginatedRequestParams, ReadResourceRequestParams, ReadResourceResponse, ServerCapabilities,
    ServerConfig,
};
use rmcp::service::{RequestContext, RoleServer};
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp::transport::streamable_http_server::{StreamableHttpServerConfig, StreamableHttpService};
use rmcp::{ErrorData as McpError, ServerHandler, tool, tool_handler, tool_router};
use tracing::Instrument as _;

use crate::config::AppConfig;
use crate::device_locks::DeviceLocks;
use crate::discovery::DeviceCache;
use crate::errors::TapoMcpError;
use crate::models::{
    CONTROL_CAPABILITIES, ControlDeviceParams, GetDeviceStateParams, TakeSnapshotParams,
};
use crate::resources;
use crate::snapshots::{SNAPSHOT_TTL, SnapshotStore};
use crate::telemetry;
use crate::tools;

/// What every client needs to know to use the tools well, sent to the client
/// when it connects.
const INSTRUCTIONS: &str = "Control and monitor Tapo smart home devices (plugs, lights, light strips, power strips, hubs and their child devices, cameras).

- Start with list_devices, or the tapo://devices resource, unless you have a recent list; it answers immediately from the server's background discovery. Use each device's `id` and `ip` exactly as listed. If a device the user expects is missing, check `errors`, `unsupported`, `discovered_at` and `refresh_error`: a device that joined after `discovered_at` appears after the next refresh, so do not call list_devices in a loop.
- A child device (a power strip's child plug, or a device paired with a hub) is addressed with its own `id` and its parent's `ip`, which its list entry carries.
- get_device_state reads the device's `get_capabilities`, except `Snapshot`, which take_snapshot reads; control_device applies its `set_capabilities`.
- DeviceInfo: whether a device is on, its brightness and color, a sensor's latest reading. EnergyMeterData: what a plug draws now. EnergyDataHourly, EnergyDataDaily, EnergyDataMonthly: energy used over hours, days or months. PowerDataEvery5Minutes, PowerDataHourly: power drawn over time. TemperatureHumidityRecords, TriggerLogs: a sensor's history. Detections: what a camera detected. Snapshot: what a camera sees now. control_device returns the device's state afterwards, so there is no need to read it again.
- Date-times are in UTC, except that energy data covers whole days and months in the plug's time zone, as the Tapo app does; its entry times are still UTC and its `local_time` is the plug's clock.
- EnergyMeterData gives mV, mA and mW, and the energy since the last restart as one total in both Wh and mWh, not to be added. Energy data is in Wh, power data in W.
- Calls that reach a device run one at a time per device, children sharing their parent's turn, and wait for a running background discovery, so one can take a minute or more. Send several changes to one device as one control_device call.";
// Some clients cut server instructions at 2048 characters. `len` counts bytes,
// which is never fewer, so the check errs on the safe side.
const _: () = assert!(INSTRUCTIONS.len() <= 2048);

#[derive(Clone)]
pub struct TapoMcp {
    config: Arc<AppConfig>,
    devices: Arc<DeviceCache>,
    locks: Arc<DeviceLocks>,
    snapshots: Arc<SnapshotStore>,
}

#[tool_router]
impl TapoMcp {
    pub(crate) fn new(
        config: Arc<AppConfig>,
        devices: Arc<DeviceCache>,
        locks: Arc<DeviceLocks>,
        snapshots: Arc<SnapshotStore>,
    ) -> Self {
        Self {
            config,
            devices,
            locks,
            snapshots,
        }
    }

    #[tool(
        description = "Get a Tapo device's current state. Checks the capability's parameters, then verifies the device ID matches at the given IP. A camera's `Snapshot` capability is read with take_snapshot, not this tool.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn get_device_state(
        &self,
        Parameters(params): Parameters<GetDeviceStateParams>,
    ) -> Result<CallToolResult, TapoMcpError> {
        tools::get_device_state(&self.config, &self.locks, params).await
    }

    #[tool(
        description = format!(
            "Control a Tapo device by applying {} to {} set capabilities, in order. Checks the count and every value before contacting the device, then verifies the device ID matches at the given IP. A capability the device does not support fails the call before any capability is applied. When the device returns an error partway through, the error says how many capabilities were applied and which one failed. Returns the device's state afterwards, read back from the device as the DeviceInfo capability returns it.",
            CONTROL_CAPABILITIES.start(),
            CONTROL_CAPABILITIES.end()
        ),
        annotations(
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn control_device(
        &self,
        Parameters(params): Parameters<ControlDeviceParams>,
    ) -> Result<CallToolResult, TapoMcpError> {
        tools::control_device(&self.config, &self.locks, params).await
    }

    #[tool(
        description = "List available Tapo devices. The list is refreshed periodically in the background. If a device the user expects is missing, it may have joined after `discovered_at` and will appear after the next refresh. Prefer reading the `tapo://devices` resource instead if your client supports resources.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn list_devices(&self) -> Result<CallToolResult, TapoMcpError> {
        tools::list_devices(&self.devices)
    }

    #[tool(
        description = format!(
            "Capture a still JPEG snapshot from a Tapo camera (~640x360). Verifies the device ID matches at the given IP first. Returns the device id, capture time and size as text alongside the image. The text may also include a link to the image that is valid for {} minutes; share it with the user or fetch it to save the file.",
            SNAPSHOT_TTL.as_secs() / 60
        ),
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn take_snapshot(
        &self,
        Parameters(params): Parameters<TakeSnapshotParams>,
    ) -> Result<CallToolResult, TapoMcpError> {
        tools::take_snapshot(&self.config, &self.locks, &self.snapshots, params).await
    }
}

#[tool_handler]
impl ServerHandler for TapoMcp {
    fn get_info(&self) -> ServerConfig {
        let mut info = ServerConfig::new(
            ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .build(),
        )
        .with_instructions(INSTRUCTIONS);
        info.server_info.name = env!("CARGO_PKG_NAME").to_owned();
        info.server_info.version = env!("CARGO_PKG_VERSION").to_owned();
        info.server_info.title = Some("Tapo MCP Server".to_owned());
        info
    }

    // Replaces the `call_tool` that `#[tool_handler]` would generate, so every
    // tool call runs inside one server span.
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, McpError> {
        let router = Self::tool_router();
        let known_tool = router.has_route(&request.name);
        let span = telemetry::tool_call_span(&request, known_tool, &context);
        let tcc = ToolCallContext::new(self, request, context);
        let result = router.call(tcc).instrument(span.clone()).await;
        telemetry::record_tool_outcome(&span, &result);
        result
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        let resource = resources::build_devices_resource();
        Ok(ListResourcesResult::with_all_items(vec![resource]))
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResponse, McpError> {
        match request.uri.as_str() {
            resources::DEVICES_RESOURCE_URI => Ok(resources::read_devices(&self.devices)?.into()),
            _ => Err(McpError::resource_not_found(
                "Unknown resource URI",
                Some(serde_json::json!({ "uri": request.uri })),
            )),
        }
    }
}

pub fn new_service(
    app_config: Arc<AppConfig>,
    devices: Arc<DeviceCache>,
    locks: Arc<DeviceLocks>,
    snapshots: Arc<SnapshotStore>,
) -> StreamableHttpService<TapoMcp, LocalSessionManager> {
    let session_manager = Arc::new(LocalSessionManager::default());
    // Keep rmcp's DNS-rebinding protection. By default only loopback `Host`
    // headers are accepted; LAN/remote deployments opt in explicit hostnames
    // via `TAPO_MCP_ALLOWED_HOSTS`.
    let mut server_config = StreamableHttpServerConfig::default();
    if !app_config.allowed_hosts.is_empty() {
        server_config = server_config.with_allowed_hosts(app_config.allowed_hosts.clone());
    }
    StreamableHttpService::new(
        move || {
            Ok(TapoMcp::new(
                Arc::clone(&app_config),
                Arc::clone(&devices),
                Arc::clone(&locks),
                Arc::clone(&snapshots),
            ))
        },
        session_manager,
        server_config,
    )
}
