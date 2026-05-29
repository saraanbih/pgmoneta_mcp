// Copyright (C) 2026 The pgmoneta community
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use std::borrow::Cow;
use std::sync::Arc;

use super::PgmonetaHandler;
use crate::client::PgmonetaClient;
use rmcp::ErrorData as McpError;
use rmcp::handler::server::router::tool::{AsyncTool, ToolBase};
use rmcp::model::JsonObject;
use rmcp::schemars;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct WalInfoRequest {
    pub username: String,
    pub server: String,
}

/// Tool for retrieving WAL information for a given server.
pub struct WalInfoTool;

impl ToolBase for WalInfoTool {
    type Parameter = WalInfoRequest;
    type Output = String;
    type Error = McpError;

    fn name() -> Cow<'static, str> {
        "walinfo".into()
    }

    fn description() -> Option<Cow<'static, str>> {
        Some(
            "Get Write-Ahead Log (WAL) information for a PostgreSQL server. \
            Returns details about WAL segments, timeline history, and archiving status. \
            The username has to be one of the pgmoneta admins to be able to access pgmoneta"
                .into(),
        )
    }

    // input_schema is NOT overridden — the default generates the correct JSON schema
    // automatically from `type Parameter = WalInfoRequest` via its JsonSchema derive.

    // output_schema must be overridden to return None because our Output type is String
    // (dynamically-translated JSON), and the MCP spec requires output schema root type
    // to be 'object', which String does not satisfy.
    fn output_schema() -> Option<Arc<JsonObject>> {
        None
    }
}

impl AsyncTool<PgmonetaHandler> for WalInfoTool {
    async fn invoke(
        _service: &PgmonetaHandler,
        request: WalInfoRequest,
    ) -> Result<String, McpError> {
        let result: String = PgmonetaClient::request_walinfo(&request.username, &request.server)
            .await
            .map_err(|e| {
                McpError::internal_error(
                    format!("Failed to retrieve WAL information: {:?}", e),
                    None,
                )
            })?;
        PgmonetaHandler::generate_call_tool_result_string(&result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handler::PgmonetaHandler;
    use rmcp::handler::server::router::tool::ToolBase;

    #[test]
    fn test_walinfo_tool_metadata() {
        assert_eq!(WalInfoTool::name(), "walinfo");
        let desc = WalInfoTool::description();
        assert!(desc.is_some());
        let desc = desc.unwrap();
        assert!(desc.contains("WAL"));
        assert!(desc.contains("pgmoneta admins"));
    }

    #[test]
    fn test_handler_has_walinfo_tool() {
        let tools = PgmonetaHandler::tool_router().list_all();
        let tool_names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
        assert!(
            tool_names.contains(&"walinfo"),
            "walinfo tool should be registered, found: {:?}",
            tool_names
        );
    }

    #[test]
    fn test_walinfo_request_deserializes() {
        let request: WalInfoRequest = serde_json::from_value(serde_json::json!({
            "username": "alice",
            "server": "primary"
        }))
        .expect("Failed to deserialize WalInfoRequest");
        assert_eq!(request.username, "alice");
        assert_eq!(request.server, "primary");
    }

    #[test]
    fn test_walinfo_tool_output_schema_is_none() {
        assert!(WalInfoTool::output_schema().is_none());
    }
}
