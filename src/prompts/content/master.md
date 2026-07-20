# Confluence Data Center workflows

Start here. Match the user's goal (or the "Context already provided" `goal`
field above, if supplied) to one sub-workflow below, then fetch that
prompt by name (`prompts/get`).

**If your environment provides a way to run a sub-task/agent in an isolated
context, delegate the entire matched sub-workflow to it**: hand the
sub-task the prompt name and whatever parameters are already known, let it
fetch the prompt itself and carry out every step -- including all of its
own `search`/`get`/`call` traffic -- entirely within its own context, and
have it report back only a short summary (what was accomplished/confirmed,
and anything still needed from the user). Only run the sub-workflow's
steps directly in this conversation if no such delegation mechanism is
available.

Every operation reference in every sub-workflow is phrased as a capability
to search for (e.g. "search for how to update a page"), never a hardcoded
operationId -- this server supports 7 different Confluence API versions,
and operationIds/schemas genuinely differ across them. Always read the
schema `get` returns before relying on any field name.

## Sub-workflows

- `confluence-spaces` -- create, update, delete, archive, restore, or browse a space.
- `confluence-content` -- create, update, delete, or browse pages/blog posts, their hierarchy and history.
- `confluence-attachments` -- upload, list, update, delete, or move attachments on a page.
- `confluence-labels` -- add, remove, or browse labels on content or spaces.
- `confluence-properties` -- read or write arbitrary key/value metadata on content or spaces.
- `confluence-permissions-restrictions` -- restrict a specific page/blog post, or grant a broader space/site permission.
- `confluence-users-groups` -- manage users, groups, and group membership.
- `confluence-search-cql` -- find content via CQL or general search.
- `confluence-watches` -- check, add, or remove watch subscriptions on content or spaces.
- `confluence-backup-restore` -- back up or restore a site or space's definitions.
- `confluence-webhooks` -- create, test, or inspect webhooks.
- `confluence-admin-diagnostics` -- reindexing, cluster status, audit records, and other operational signals.
- `confluence-space-provisioning` -- set up a brand-new team space end-to-end: create, permission, categorize, seed content.
- `confluence-user-lifecycle` -- onboard or offboard a user across accounts, groups, permissions, and watches.

The last two are composite workflows that sequence several of the
domain ones above in a specific order -- fetch them directly for a
whole end-to-end task instead of stitching the domain prompts together
yourself.

If the user's goal doesn't clearly match one sub-workflow, ask a short
clarifying question rather than guessing which one to fetch.
