# MCP prompts: composite, cross-domain workflows

## Context

`docs/mcp-prompts-workflow-plan.md` shipped 12 domain sub-workflows plus a
master menu, each deliberately scoped to one Confluence resource type
(spaces, content, attachments, labels, properties, permissions &
restrictions, users & groups, CQL search, watches, backup/restore,
webhooks, admin diagnostics). Re-verified directly against the default
embedded store (`mcp_store.db`, 176 operations, decompressed and queried
fresh for this plan): every one of the 18 top-level path segments in the
catalog (`space` 42, `content` 41, `user` 16, `permissions` 14,
`backup-restore` 12, `group` 11, `admin` 10, `webhooks` 9, `label` 4,
`index` 4, `color-scheme` 4, `longtask` 2, and 6 single-operation segments
-- `server-information`, `search`, `instance-metrics`, `contentbody`,
`cluster`, `audit`, `accessmode`) already has a home in one of the 12
existing prompts. There is no uncovered *domain*.

The gap is a different one: real admin/end-user tasks routinely span more
than one of those domains in a specific order, and that sequencing/gating
knowledge -- exactly the thing the original plan's Context argued was
missing from the flat 3-tool surface -- is precisely what a
single-domain prompt does not capture. Two concrete, common tasks fit
this shape and are worth their own guided prompt rather than leaving the
calling LLM to stitch several domain prompts together unaided each time:

1. **Provisioning a new team space end-to-end** -- create the space (a
   real fork: regular vs. private vs. personal, decided once at creation
   and not cleanly changed after), then grant its initial permissions,
   categorize it, and optionally seed a home page -- spanning
   `confluence-spaces`, `confluence-permissions-restrictions`,
   `confluence-labels`, and `confluence-content`.
2. **Onboarding or offboarding a user** -- a real fork (which direction),
   each with its own ordered, gated steps: onboarding is check-then-create
   the account, add to the right group(s), grant space permissions, and
   optionally add a relevant watch; offboarding is disable (not delete,
   unless the user explicitly confirms otherwise) the account, remove
   group memberships, and explicitly revoke permissions (mindful of
   `confluence-permissions-restrictions`' replace-all semantics)
   -- spanning `confluence-users-groups`,
   `confluence-permissions-restrictions`, and
   `confluence-watches`.

A third candidate considered and rejected as a standalone prompt: safely
decommissioning a space (back up, then archive/trash). It doesn't have
enough of its own sequencing logic beyond "back up first" to justify a
whole new prompt -- instead, `confluence-spaces`' existing
gotcha list gets one added line cross-referencing
`confluence-backup-restore` before a destructive space
operation, consistent with how every prompt already cross-references
others rather than duplicating them.

Every new prompt follows the same agnostic-phrasing rule and version-drift
rationale already established in `docs/mcp-prompts-workflow-plan.md` --
this server still embeds and serves 7 Confluence API versions
(`10.2.14` default through `9.2.21`), so nothing here hardcodes an
`operationId` or an assumed response field name.

Separately, `README.md` never documented the prompts capability at all
(it predates that feature and only covers the `search`/`get`/`call`
tools) -- this plan also adds a "Workflows (MCP prompts)" section.

## Approach

### New prompts

| name | description | arguments |
|---|---|---|
| `confluence-space-provisioning` | Guided end-to-end setup of a new team space: create (regular/private/personal fork), grant initial permissions, categorize, optionally seed a home page. | `space_key`, `space_type` |
| `confluence-user-lifecycle` | Onboard (check-then-create, group membership, permissions, watch) or offboard (disable, remove from groups, revoke permissions) a user -- an explicit fork on direction. | `username`, `lifecycle_stage`, `space_key` |

Both are worked examples in the same 60-120 line band as
`permissions_restrictions.md`/`backup_restore.md`: numbered steps, an
explicit fork up front, a "don't proceed until confirmed" gate per step,
and a "composing with other workflows" section that cross-references the
domain prompts they build on rather than repeating their content.

Argument structs (`src/prompts/mod.rs`, same `Option<String>`-everywhere
shape as the existing ones):

```rust
pub struct SpaceProvisioningArgs {
    pub space_key: Option<String>,
    pub space_type: Option<String>,
}

pub struct UserLifecycleArgs {
    pub username: Option<String>,
    pub lifecycle_stage: Option<String>,
    pub space_key: Option<String>,
}
```

### Files touched

- `src/prompts/mod.rs` -- the two new arg structs above.
- `src/prompts/router.rs` -- two new `#[prompt(...)]` methods, same shape
  as the existing ones (`Parameters<T>` + `render_context_header` +
  `include_str!`).
- `src/prompts/content/space_provisioning.md` (new)
- `src/prompts/content/user_lifecycle.md` (new)
- `src/prompts/content/master.md` -- add both to the sub-workflow menu.
- `src/prompts/content/spaces.md` -- one added gotcha line cross-referencing
  `confluence-backup-restore` before archiving/trashing/deleting.
- `tests/prompts_workflow.rs` -- extend the expected 13-name list to 15;
  the existing per-prompt fetch-and-assert loop automatically covers both
  new prompts once they're in that list.
- `README.md` -- new "### Workflows (MCP prompts)" section under `## Usage`,
  documenting the master prompt, the full sub-workflow list (14 names),
  and the delegation/agnostic-phrasing design principles in brief.

No changes to `src/core/mcp_server.rs` or the `McpifyServer` struct --
the prompt-router wiring from the prior plan already covers any number of
`#[prompt]` methods.

## Verification

- `cargo fmt --check`, `cargo clippy --locked --all-targets -- -D warnings`,
  `cargo test --locked` (all must pass, matching the main CI workflow
  exactly -- the prior feature's CI failures were fmt drift and a
  production-coverage gap, so re-run `bash scripts/coverage.sh` locally
  before pushing this time, not just `cargo test`).
- `tests/prompts_workflow.rs`'s existing per-prompt loop confirms both new
  prompts are fetchable with no arguments and return non-empty content,
  which also keeps production coverage from regressing the way it did
  last time.
- Manual smoke check: `cargo run -- start` and `prompts/get` for both new
  prompts with partial arguments, confirming the rendered header and
  content read naturally.

## Release

Same tag-driven convention as before. This is a `feat` (two new prompts),
so per this repo's actual history (confirmed again: `feat: adopt current
mcpify Rust parity` bumped `0.2.12` -> `0.3.0`, a minor bump, while
`fix`/`chore`/`test`/`style` commits only bump patch), this bumps the
**minor** version: `0.6.0` -> `0.7.0`.

1. `git commit` the implementation (`feat(prompts): ...`).
2. `git commit` this plan doc separately (`docs: ...`).
3. Bump `Cargo.toml`/`Cargo.lock` to `0.7.0`, commit as
   `chore(release): bump version to 0.7.0`.
4. `git tag v0.7.0`.
5. `git push` branch, then push the tag.
