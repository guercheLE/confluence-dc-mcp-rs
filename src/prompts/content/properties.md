# Confluence workflow: content & space properties

Covers generic key/value metadata storage -- reading, creating, updating,
and deleting a named property -- on either a piece of content or a whole
space. Both resource types expose the same parallel shape (list all
properties, get/create/update/delete by key), so first confirm which
resource (a specific content id, or a space key) the property belongs to.

Search for the capability in plain language (e.g. "search for how to set
a property on a page" or "search for how to read a space's properties")
and read the schema `get` returns before calling -- property *values* are
arbitrary caller-defined JSON, so don't assume a shape for the value
itself beyond what the caller actually wants stored.

**Gotcha:** updating an existing property's value typically requires
knowing its current version number (similar to content updates) -- read
the property back first if the target update operation's schema expects
one, rather than guessing.

Verify by re-reading the property by key after a write rather than
trusting a non-error response alone.
