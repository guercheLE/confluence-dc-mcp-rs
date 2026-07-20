# Confluence workflow: space provisioning

This sub-workflow is designed to be run as an isolated sub-task where
possible -- if delegated here from `confluence`'s routing, or
your environment otherwise supports it, everything you need is in this
prompt's own text plus the parameters already listed above; report back
only a short summary when done.

## Step 0 -- gather required parameters

Check the "Context already provided" header first. Only ask the user for
what's still missing: the new space's key, and its type (regular,
private, or personal).

## Step 1 -- an explicit fork on space type

Confluence creates regular, private, and personal spaces through entirely
different operations -- this choice is effectively permanent once made,
so confirm it rather than guessing:

- **Regular** -- visible per normal space permissions, the common case
  for a team space.
- **Private** -- created visible only to its creator until permissions
  are explicitly widened.
- **Personal** -- tied to a specific user's account.

Search for how to create the matching kind of space and read the schema
`get` returns before calling.

## Step 2 -- create and verify

Call the create operation, then verify by re-fetching the space by key
rather than trusting a non-error response alone. Don't proceed to Step 3
until the space is confirmed to exist.

## Step 3 -- initial permissions

Delegate to `confluence-permissions-restrictions` for granting
the right groups/users access to the new space -- don't duplicate that
read-merge-write-verify logic here. Gate: don't consider the space ready
for its team until those permissions are confirmed granted by re-reading
them, not just because the grant call didn't error.

## Step 4 -- categorize (optional)

If the user wants the space discoverable under a category, delegate to
`confluence-labels` for adding a space category/label.

## Step 5 -- seed initial content (optional)

If the user wants a starter home page or initial structure, delegate to
`confluence-content` for creating that content -- the new
space's home page already exists once the space is created, so this step
is about adding *additional* structure, not replacing the auto-created
home page unless the user explicitly asks to edit it.

## Step 6 -- summarize

Report which of steps 3-5 were actually completed and which were
skipped or still need the user's input, rather than assuming every
optional step was wanted.
