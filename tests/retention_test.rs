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

use pgmoneta_mcp::handler::PgmonetaHandler;
use pgmoneta_mcp::handler::retention::{
    ExpungeBackupTool, ExpungeRequest, RetainBackupTool, RetainRequest,
};
use rmcp::handler::server::router::tool::AsyncTool;
use serde_json::Value;

mod common;

fn assert_command_and_status(response: &str, json: &Value, expected_command: &str) {
    let header = json
        .get("Header")
        .unwrap_or_else(|| panic!("Header field missing in response: {response}"));
    let command = header
        .get("Command")
        .unwrap_or_else(|| panic!("Command field missing in Header: {response}"));
    assert_eq!(
        command, expected_command,
        "unexpected command in response: {response}"
    );

    let outcome = json
        .get("Outcome")
        .unwrap_or_else(|| panic!("Outcome field missing in response: {response}"));
    let status = outcome
        .get("Status")
        .unwrap_or_else(|| panic!("Status field missing in Outcome: {response}"));
    assert_eq!(status, true, "unexpected status in response: {response}");
}

#[tokio::test]
#[ignore = "requires pgmoneta stack (see test/check.sh and full-test CI job)"]
async fn retain_backup_test() {
    common::init_config();
    let _guard = common::backup_fixture_lock().await;
    let backup_id = common::ensure_backup("primary")
        .await
        .expect("backup fixture should be created");

    let handler = PgmonetaHandler::new();
    let request = RetainRequest {
        username: "backup_user".to_string(),
        server: "primary".to_string(),
        backup_id,
    };

    let response = RetainBackupTool::invoke(&handler, request)
        .await
        .expect("retain_backup should succeed");

    let json: Value = serde_json::from_str(&response).expect("response should be valid json");
    assert_command_and_status(&response, &json, "retain");
}

#[tokio::test]
#[ignore = "requires pgmoneta stack (see test/check.sh and full-test CI job)"]
async fn expunge_backup_test() {
    common::init_config();
    let _guard = common::backup_fixture_lock().await;
    let backup_id = common::ensure_backup("primary")
        .await
        .expect("backup fixture should be created");

    let handler = PgmonetaHandler::new();
    let retain_request = RetainRequest {
        username: "backup_user".to_string(),
        server: "primary".to_string(),
        backup_id: backup_id.clone(),
    };
    let retain_response = RetainBackupTool::invoke(&handler, retain_request)
        .await
        .expect("retain_backup should succeed before expunge_backup");
    let retain_json: Value =
        serde_json::from_str(&retain_response).expect("retain response should be valid json");
    assert_command_and_status(&retain_response, &retain_json, "retain");

    let request = ExpungeRequest {
        username: "backup_user".to_string(),
        server: "primary".to_string(),
        backup_id,
    };

    let response = ExpungeBackupTool::invoke(&handler, request)
        .await
        .expect("expunge_backup should succeed");

    let json: Value = serde_json::from_str(&response).expect("response should be valid json");
    assert_command_and_status(&response, &json, "expunge");
}
