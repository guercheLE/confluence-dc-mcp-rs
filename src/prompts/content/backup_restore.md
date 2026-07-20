# Confluence workflow: backup & restore

This sub-workflow is designed to be run as an isolated sub-task where
possible -- if delegated here from `confluence`'s routing, or your
environment otherwise supports it, everything you need is in this
prompt's own text plus the parameters already listed above; report back
only a short summary when done.

## Step 0 -- gather required parameters

Check the "Context already provided" header first. Only ask the user for
what's still missing: is this a **site-wide** backup/restore, or scoped to
one **space** (in which case, which space key)?

## Step 1 -- the scope fork

- **Site** -- backs up or restores the whole instance's definitions.
- **Space** -- scoped to one space's content; needs the space key.

These use different operations entirely -- ask which scope applies rather
than guessing, since running a site-wide job when only one space was
intended is expensive and hard to undo.

## Step 2 -- fire the job

Search for how to create a backup (or restore) job at the confirmed scope,
then call it. This starts an **asynchronous** job -- the response
confirms the job was *accepted*, not that it finished.

## Step 3 -- poll until terminal, don't guess "done"

Search for how to check a long-running job's status and poll it
periodically. Do not report the backup/restore as complete just because
the create call didn't error -- that only means the job was queued. Only
consider it done once the job's own status field reports a terminal state
(succeeded/failed), and report the terminal state's actual result, not an
assumption. If polling would take many round-trips, consider delegating
this whole step to a sub-task if your environment supports one, and have
it report back only the final status.

## Step 4 -- retrieve the result

Once the job is confirmed successful, search for how to download the
resulting backup file (or, for a restore, confirm the target space/site
now reflects the restored content) before telling the user it's done.

## Step 5 -- cancellation

If the user wants to abort a queued or running job, search for how to
cancel a specific job (or clear all queued jobs, for a bulk cancellation)
-- confirm which one before calling, since clearing all queued jobs
affects every pending job, not just the user's own.

## Composing with other workflows

Restoring into a specific space may interact with that space's existing
content -- see `confluence-spaces` and `confluence-content`
for how to inspect what's there beforehand.
