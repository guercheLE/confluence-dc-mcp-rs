# Confluence workflow: permissions & restrictions

This sub-workflow is designed to be run as an isolated sub-task where possible —
if you were delegated here from `confluence_workflow`'s routing, or your
environment otherwise supports running this as its own sub-task, everything
you need is in this prompt's own text plus the parameters already listed
above; report back only a short summary when done rather than the full
step-by-step trace.

## Step 0 -- gather required parameters

Check the "Context already provided" header above first; only ask the user
for whatever is still listed as missing:

- Is the target a single page/blog post, or a whole space (or site-wide)?
- Who is the principal -- a specific user or a group?

Don't proceed to Step 1 until both are known.

## Step 1 -- an explicit fork

Confluence models these two things completely differently, so ask rather
than guess:

- **(A) Content-level restriction** -- a specific page/blog post's view
  and/or update access. Setting this **replaces the entire restriction set
  for that operation** (view or update); anything not included in the write
  is silently revoked.
- **(B) Space-level or global permission grant** -- broader, per
  user/group/anonymous/unlicensed grant-or-revoke. Additive by default:
  a principal not mentioned in the request keeps their existing
  permissions, but an explicitly empty list for a mentioned principal
  revokes all of theirs.

Ask: "are you restricting one specific page/blog post, or granting a
broader space/site-wide permission?" The two are not interchangeable.

## Step 2 (path A) -- read before writing

Because the content-restriction write is replace-all per operation, search
for and call the operation that reads the content's *current* restrictions
(by operation) before changing anything, so the new principal can be
merged into the existing list rather than overwriting it. If you skip this
read, you will silently revoke every other user/group's existing access
for that operation. Don't proceed to Step 3 until the current restriction
set has actually been read back, not assumed empty.

## Step 3 (path A) -- apply the change

Search for how to update a piece of content's restrictions, then call the
matching operation with the merged list (existing principals plus the new
one). Never assume a field name from this prose -- read the schema `get`
returns for whatever operationId `search` resolves to, since the exact
shape can differ across the API versions this server supports.

## Step 4 (path A) -- verify

Re-read the content's restrictions (or the relevant/effective-view
equivalent) to confirm the intended principal now actually has access. A
non-error response from Step 3 does not by itself prove this -- the
follow-up read is the real gate.

## Step 5 (path B) -- space/global permissions

Same read-merge-write-verify shape, but against the space-level or global
permission endpoints instead: read the current grants, merge in the new
principal's permissions (don't drop any principal you didn't intend to
touch), apply, then re-read to confirm. Keep in mind the "absent principal
keeps their permissions, empty list revokes all of theirs" semantics noted
in Step 1 -- it is its own must-not-guess gotcha, independent from path A's.

## Composing with other workflows

Identifying the target page or space overlaps with `confluence_workflow_content`
and `confluence_workflow_spaces` -- fetch those prompts by name for more
detail on locating or creating the resource itself rather than duplicating
that here.
