# Confluence workflow: users & groups

Covers user lifecycle (including site-admin provisioning: create, update,
enable/disable, change password, list active users), group lifecycle
(create/delete a group), and group membership (direct members, nested
members, and ancestor/parent groups) plus assigning or removing a user
from a group.

First confirm which of these the user actually wants: **provisioning** a
user or group (an admin-level operation, typically restricted to site
admins) versus **inspecting** membership (read-only, more widely
available). Search for the capability in plain language (e.g. "search for
how to create a user" or "search for how to list a group's nested
members") and read the schema `get` returns before calling.

**Gotcha:** disabling a user is not the same as deleting one -- confirm
which the user actually wants, since deletion is typically harder to
reverse.

Verify a provisioning change by re-fetching the user or group afterward
rather than trusting a non-error response alone. If a membership listing
could be large (e.g. a big group's nested members), consider delegating
that one step to a sub-task if your environment supports it, and bring
back only the distilled answer.
