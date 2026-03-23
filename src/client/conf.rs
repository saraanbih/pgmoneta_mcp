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

use super::PgmonetaClient;
use crate::constant::Command;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
struct EmptyRequest {}

#[derive(Serialize, Clone, Debug)]
struct ConfSetRequest {
    #[serde(rename = "ConfigKey")]
    config_key: String,
    #[serde(rename = "ConfigValue")]
    config_value: String,
}

impl PgmonetaClient {
    pub async fn request_conf_reload(username: &str) -> anyhow::Result<String> {
        let request = EmptyRequest {};
        Self::forward_request(username, Command::RELOAD, request).await
    }

    pub async fn request_conf_ls(username: &str) -> anyhow::Result<String> {
        let request = EmptyRequest {};
        Self::forward_request(username, Command::CONF_LS, request).await
    }

    pub async fn request_conf_get(username: &str) -> anyhow::Result<String> {
        let request = EmptyRequest {};
        Self::forward_request(username, Command::CONF_GET, request).await
    }

    pub async fn request_conf_set(
        username: &str,
        config_key: &str,
        config_value: &str,
    ) -> anyhow::Result<String> {
        let request = ConfSetRequest {
            config_key: config_key.to_string(),
            config_value: config_value.to_string(),
        };
        Self::forward_request(username, Command::CONF_SET, request).await
    }
}
