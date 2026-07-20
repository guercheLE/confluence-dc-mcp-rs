# Confluence workflow: pages, blog posts & content

This sub-workflow is designed to be run as an isolated sub-task where
possible -- if delegated here from `confluence`'s routing, or your
environment otherwise supports it, everything you need is in this prompt's
own text; report back only a short summary when done.

## Step 0 -- gather required parameters

Ask the user (if not already obvious from context): is this a **page**, a
**blog post**, or a **comment** (comments are just content of a different
type, attached to a parent via a container reference)? Which space? For a
page, does it need a specific parent (a child page), or should it sit at
the space's top level?

## Step 1 -- an explicit fork on hierarchy

- **Top-level page** -- no ancestor needed, just the target space.
- **Child page** -- needs the parent's content id as an ancestor reference.
- **Blog post** -- belongs to a space but has no page hierarchy.
- **Comment** -- needs a container reference to the content it comments on.

Ask which of these applies rather than guessing; search for how to create
content of the right type and read the schema `get` returns for the exact
shape (body representation, ancestors, container) before calling.

## Step 2 -- creating

Search for how to create content, then call it with the type, space, body
(commonly the `storage` representation -- Confluence's storage-format
XHTML), and, per the Step 1 fork, ancestors or a container reference.
Verify by re-fetching the new content by id rather than trusting a
non-error response alone.

## Step 3 -- updating: the version-conflict gate

Confluence's content update uses optimistic locking: every update requires
the content's **current version number incremented by one**, not an
arbitrary value. Before any update, search for and call the operation that
reads the content's current state (or history) to get its live version
number -- don't assume a version number from an earlier step, since
another edit may have happened since. If the update is rejected, re-read
the current version and retry with the corrected number rather than
guessing.

## Step 4 -- hierarchy and history (read paths)

For browsing rather than mutating: search for how to list a piece of
content's children/descendants (optionally filtered by type), or its
edit history. If a listing looks like it could return many results,
consider delegating that one step to a sub-task if your environment
supports it, and bring back only the distilled answer (e.g. "12 child
pages, here are their ids/titles") rather than the full listing.

## Step 5 -- body-format conversion

If the caller needs a different representation of the same content (e.g.
converting `storage` format to `view` or `export_view` for display),
search for the content-body conversion operation rather than trying to
transform the body yourself -- Confluence's storage format has its own
macro/markup rules that are easy to get subtly wrong by hand.

## Composing with other workflows

Labels, comments-listing, properties, and restrictions on a piece of
content are covered by `confluence-labels`,
`confluence-properties`, and
`confluence-permissions-restrictions` respectively -- fetch those
prompts by name rather than duplicating them here. Attachments are
covered by `confluence-attachments`.
