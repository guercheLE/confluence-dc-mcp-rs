//! MCP prompts: guided, multi-step Confluence Data Center workflows layered
//! on top of the `search`/`get`/`call` tool surface (see
//! `docs/mcp-prompts-workflow-plan.md`). Kept entirely separate from
//! `crate::tools`, which holds `search`/`get`/`call`'s own business logic.

pub mod router;

use rmcp::schemars;
use serde::Deserialize;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct MasterWorkflowArgs {
    /// What the user is trying to accomplish, in their own words
    pub goal: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PermissionsRestrictionsArgs {
    /// The id of the specific page or blog post to restrict, if that's the target
    pub content_id: Option<String>,
    /// The space key, if granting a space-level or global permission instead
    pub space_key: Option<String>,
    /// Whether the principal is a "user" or a "group"
    pub principal_type: Option<String>,
    /// The username or group name to grant or restrict access for
    pub principal_name: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct BackupRestoreArgs {
    /// Whether this is a "site" or "space" scoped backup/restore job
    pub scope: Option<String>,
    /// The space key, required when scope is "space"
    pub space_key: Option<String>,
}

/// Renders a short "Context already provided" / "Still missing" header from
/// whichever optional prompt arguments the caller already supplied, so a
/// sub-workflow's numbered steps don't have to re-ask for values already
/// known. Prepended to the static markdown body pulled in via
/// `include_str!` — no template-substitution engine needed.
pub(crate) fn render_context_header(fields: &[(&str, Option<&str>)]) -> String {
    let mut known = Vec::new();
    let mut missing = Vec::new();
    for (name, value) in fields {
        match value {
            Some(v) => known.push(format!("- `{name}`: {v}")),
            None => missing.push(format!("- `{name}`")),
        }
    }

    let mut header = String::from("## Context already provided\n");
    if known.is_empty() {
        header.push_str("(none)\n");
    } else {
        header.push_str(&known.join("\n"));
        header.push('\n');
    }

    if !missing.is_empty() {
        header.push_str("\n## Still missing -- ask the user for these before proceeding\n");
        header.push_str(&missing.join("\n"));
        header.push('\n');
    }

    header
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_slice_renders_only_the_known_header_with_none() {
        let header = render_context_header(&[]);
        assert!(header.contains("Context already provided"));
        assert!(header.contains("(none)"));
        assert!(!header.contains("Still missing"));
    }

    #[test]
    fn all_supplied_lists_every_value_and_skips_the_missing_section() {
        let header = render_context_header(&[
            ("goal", Some("restrict a page to one team")),
            ("content_id", Some("123456")),
        ]);
        assert!(header.contains("`goal`: restrict a page to one team"));
        assert!(header.contains("`content_id`: 123456"));
        assert!(!header.contains("Still missing"));
    }

    #[test]
    fn all_missing_lists_every_field_under_the_missing_section() {
        let header = render_context_header(&[("content_id", None), ("space_key", None)]);
        assert!(header.contains("(none)"));
        assert!(header.contains("Still missing"));
        assert!(header.contains("`content_id`"));
        assert!(header.contains("`space_key`"));
    }

    #[test]
    fn mixed_supplied_and_missing_splits_correctly() {
        let header = render_context_header(&[
            ("content_id", Some("123456")),
            ("space_key", None),
        ]);
        assert!(header.contains("`content_id`: 123456"));
        assert!(header.contains("Still missing"));
        assert!(header.contains("`space_key`"));
        assert!(!header.contains("`space_key`: "));
    }
}
