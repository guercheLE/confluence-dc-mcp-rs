// Protocol-level `prompts/list`/`prompts/get` integration tests for the
// guided Confluence Data Center workflow prompts (see
// docs/mcp-prompts-workflow-plan.md). Kept out of
// `src/core/mcp_server.rs`'s existing tool-focused test module entirely.

use std::sync::Arc;

use confluence_dc_mcp::auth::auth_manager::AuthManager;
use confluence_dc_mcp::core::config_schema::{AuthMethod, Config};
use confluence_dc_mcp::core::mcp_server::McpifyServer;
use rmcp::ServiceExt;
use rmcp::model::{ContentBlock, GetPromptRequestParams};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Default)]
struct TestClient;

impl rmcp::ClientHandler for TestClient {}

fn server() -> McpifyServer {
    let config: Config = serde_json::from_value(serde_json::json!({
        "url": "https://api.example.test",
        "auth_method": "basic"
    }))
    .unwrap();
    McpifyServer::new(
        "10.2.14".to_string(),
        config,
        Arc::new(Mutex::new(AuthManager::new(AuthMethod::Basic))),
    )
}

fn message_text(message: &rmcp::model::PromptMessage) -> &str {
    match &message.content {
        ContentBlock::Text(text_content) => text_content.text.as_str(),
        other => panic!("expected text content, got {other:?}"),
    }
}

#[tokio::test]
async fn server_advertises_prompts_capability() {
    use rmcp::ServerHandler;

    let info = server().get_info();
    assert!(info.capabilities.prompts.is_some());
}

#[tokio::test]
async fn prompts_list_and_get_round_trip_over_the_wire() -> anyhow::Result<()> {
    let (server_transport, client_transport) = tokio::io::duplex(64 * 1024);
    let server_task = tokio::spawn(async move {
        server().serve(server_transport).await?.waiting().await?;
        anyhow::Ok(())
    });
    let client = TestClient.serve(client_transport).await?;

    // `prompts/list` shape: all 13 confluence_workflow* prompts are advertised.
    let prompts = client.list_all_prompts().await?;
    let mut names: Vec<&str> = prompts.iter().map(|p| p.name.as_ref()).collect();
    names.sort_unstable();
    let expected = {
        let mut expected = vec![
            "confluence_workflow",
            "confluence_workflow_admin_diagnostics",
            "confluence_workflow_attachments",
            "confluence_workflow_backup_restore",
            "confluence_workflow_content",
            "confluence_workflow_labels",
            "confluence_workflow_permissions_restrictions",
            "confluence_workflow_properties",
            "confluence_workflow_search_cql",
            "confluence_workflow_spaces",
            "confluence_workflow_users_groups",
            "confluence_workflow_watches",
            "confluence_workflow_webhooks",
        ];
        expected.sort_unstable();
        expected
    };
    assert_eq!(names, expected);
    assert!(
        names
            .iter()
            .all(|name| name.starts_with("confluence_workflow")),
        "every advertised prompt should share the confluence_workflow* prefix, got {names:?}"
    );

    let restrictions_prompt = prompts
        .iter()
        .find(|p| p.name == "confluence_workflow_permissions_restrictions")
        .expect("confluence_workflow_permissions_restrictions should be advertised");
    let arguments = restrictions_prompt
        .arguments
        .as_ref()
        .expect("should advertise arguments");
    for expected in [
        "content_id",
        "space_key",
        "principal_type",
        "principal_name",
    ] {
        let arg = arguments
            .iter()
            .find(|a| a.name == expected)
            .unwrap_or_else(|| panic!("expected argument `{expected}` to be advertised"));
        assert_eq!(
            arg.required,
            Some(false),
            "prompt arguments must never be required (see plan's rationale)"
        );
    }

    // `prompts/get` round-trip: the master menu links to the worked-example sub-workflow.
    let master = client
        .get_prompt(GetPromptRequestParams::new("confluence_workflow"))
        .await?;
    let master_text = message_text(&master.messages[0]);
    assert!(master_text.contains("confluence_workflow_permissions_restrictions"));

    // `prompts/get` round-trip with partial arguments: the rendered header
    // echoes what was supplied and lists what's still missing.
    let restrictions = client
        .get_prompt(
            GetPromptRequestParams::new("confluence_workflow_permissions_restrictions")
                .with_arguments(
                    serde_json::json!({ "content_id": "123456" })
                        .as_object()
                        .unwrap()
                        .clone(),
                ),
        )
        .await?;
    let restrictions_text = message_text(&restrictions.messages[0]);
    assert!(restrictions_text.contains("`content_id`: 123456"));
    assert!(restrictions_text.contains("Still missing"));
    assert!(restrictions_text.contains("`space_key`"));
    assert!(restrictions_text.contains("`principal_type`"));
    assert!(restrictions_text.contains("`principal_name`"));

    drop(client);
    tokio::time::timeout(std::time::Duration::from_secs(2), server_task).await???;
    Ok(())
}
