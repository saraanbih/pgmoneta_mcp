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
use crate::configuration::CONFIG;

impl PgmonetaClient {
    /// Runs pgmoneta-walinfo for the WAL directory of the given server.
    pub async fn request_walinfo(_username: &str, server: &str) -> anyhow::Result<String> {
        let config = CONFIG.get().expect("Configuration should be initialized");
        let base_dir = &config.pgmoneta.base_dir;

        // WAL files are stored under <base_dir>/<server>/wal/
        let wal_dir = format!("{}/{}/wal", base_dir, server);

        let output = tokio::process::Command::new("pgmoneta-walinfo")
            .args([&wal_dir, "--format", "json", "--quiet", "--summary"])
            .output()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to execute pgmoneta-walinfo: {}", e))?;

        if output.status.success() {
            let wal_output: serde_json::Value = serde_json::from_slice(&output.stdout)
                .map_err(|e| anyhow::anyhow!("Invalid JSON from pgmoneta-walinfo output: {}", e))?;

            let wrapped = serde_json::json!({
                "Outcome": {
                    "Status": true,
                    "Command": "walinfo"
                },
                "Response": wal_output
            });

            Ok(wrapped.to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            Err(anyhow::anyhow!(
                "pgmoneta-walinfo failed for server '{}' (WAL dir: {}).\nstderr: {}\nstdout: {}",
                server,
                wal_dir,
                stderr,
                stdout
            ))
        }
    }
}
