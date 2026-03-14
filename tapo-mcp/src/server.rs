use std::sync::Arc;

use anyhow::Result;
use rmcp::handler::server::{tool::ToolRouter, wrapper::Parameters};
use rmcp::model::{
    CallToolResult, ListResourcesResult, PaginatedRequestParams, ReadResourceRequestParams,
    ReadResourceResult, ServerCapabilities, ServerInfo,
};
use rmcp::service::{RequestContext, RoleServer};
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp::transport::streamable_http_server::{StreamableHttpServerConfig, StreamableHttpService};
use rmcp::{ErrorData as McpError, ServerHandler, tool, tool_handler, tool_router};

use crate::config::AppConfig;
use crate::models::{CheckDeviceParams, GetDeviceStateParams, SetDeviceStateParams};
use crate::resources;
use crate::tools;

#[derive(Clone)]
pub struct TapoMcp {
    pub(crate) tool_router: ToolRouter<Self>,
    config: Arc<AppConfig>,
}

#[tool_router]
impl TapoMcp {
    pub(crate) fn new(config: Arc<AppConfig>) -> Self {
        Self {
            tool_router: Self::tool_router(),
            config,
        }
    }

    #[tool(
        description = "Verify a Tapo device ID matches at a given IP.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn check_device(
        &self,
        Parameters(params): Parameters<CheckDeviceParams>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::check_device(&self.config, params).await?)
    }

    #[tool(
        description = "Get a Tapo device's current state. Runs check_device first to verify the device ID matches at the given IP.",
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
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::get_device_state(&self.config, params).await?)
    }

    #[tool(
        description = "Set a Tapo device's state. Runs check_device first to verify the device ID matches at the given IP.",
        annotations(
            read_only_hint = false,
            destructive_hint = true,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn set_device_state(
        &self,
        Parameters(params): Parameters<SetDeviceStateParams>,
    ) -> Result<CallToolResult, McpError> {
        Ok(tools::set_device_state(&self.config, params).await?)
    }

    #[tool(
        description = "List available Tapo devices. Prefer reading the `tapo://devices` resource instead if your client supports resources.",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = true
        )
    )]
    async fn list_devices(&self) -> Result<CallToolResult, McpError> {
        Ok(tools::list_devices(&self.config).await?)
    }
}

#[tool_handler]
impl ServerHandler for TapoMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .build(),
        )
        .with_instructions("Tapo MCP server with device control tools and device list resources.")
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
    ) -> Result<ReadResourceResult, McpError> {
        match request.uri.as_str() {
            resources::DEVICES_RESOURCE_URI => Ok(resources::read_devices(&self.config).await?),
            _ => Err(McpError::resource_not_found(
                "Unknown resource URI",
                Some(serde_json::json!({ "uri": request.uri })),
            )),
        }
    }
}

pub fn new_service(
    app_config: Arc<AppConfig>,
) -> StreamableHttpService<TapoMcp, LocalSessionManager> {
    let session_manager = Arc::new(LocalSessionManager::default());
    let server_config = StreamableHttpServerConfig::default();
    StreamableHttpService::new(
        move || Ok(TapoMcp::new(Arc::clone(&app_config))),
        session_manager,
        server_config,
    )
}
