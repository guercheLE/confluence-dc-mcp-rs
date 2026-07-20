# Confluence workflow: spaces

Covers a space's own lifecycle: creating a space (regular, private, or
personal), fetching/updating/deleting one, archiving and restoring, moving
it to and out of the trash, managing categories, and its per-space color
scheme.

For each of these, search for the capability in plain language (e.g.
"search for how to create a private space" or "search for how to archive
a space") and read the schema `get` returns before calling -- never assume
a field name, since the exact shape can differ across the API versions
this server supports.

**Gotchas worth checking before you call:**

- Archiving and trashing are different states -- confirm which one the
  user actually wants (archived spaces are still listed differently than
  trashed ones) rather than assuming they're interchangeable.
- Deleting a space is typically irreversible in a way trashing is not --
  confirm with the user before calling a delete rather than a trash/archive
  operation if there's any ambiguity.
- Before archiving, trashing, or deleting a space that might still matter,
  consider a backup first -- see `confluence-backup-restore`.

After any create/update/delete, verify by re-fetching the space rather
than trusting a non-error response alone.

Content within a space (pages, labels, permissions, properties) is covered
by its own dedicated sub-workflow -- fetch `confluence-content`,
`confluence-labels`, `confluence-permissions-restrictions`,
or `confluence-properties` by name rather than duplicating that
here. For setting up a brand-new team space end-to-end (create, permission,
categorize, seed content in one guided flow), fetch
`confluence-space-provisioning` instead of doing all of that
manually.
