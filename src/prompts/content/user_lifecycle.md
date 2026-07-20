# Confluence workflow: user onboarding & offboarding

This sub-workflow is designed to be run as an isolated sub-task where
possible -- if delegated here from `confluence_workflow`'s routing, or
your environment otherwise supports it, everything you need is in this
prompt's own text plus the parameters already listed above; report back
only a short summary when done.

## Step 0 -- gather required parameters

Check the "Context already provided" header first. Only ask the user for
what's still missing: the username, whether this is **onboarding** or
**offboarding**, and (for onboarding, if permissions will be granted) the
relevant space key.

## Step 1 -- the direction fork

Ask which direction applies rather than guessing -- the two paths share
no steps and getting this wrong is disruptive to a real account.

## Step 2 (onboarding) -- check before creating

Search for how to look up the user by username first. Don't blindly
create a new account -- if one already exists (e.g. previously
disabled), re-enabling it is usually the right move instead of creating a
duplicate. Only call the create operation if the lookup confirms no
account exists.

## Step 3 (onboarding) -- group membership

Delegate to `confluence_workflow_users_groups` for adding the user to
whichever group(s) the user's role requires. Verify by re-reading the
group's membership afterward.

## Step 4 (onboarding) -- permissions and watches

Delegate to `confluence_workflow_permissions_restrictions` for granting
space-level access, and optionally to `confluence_workflow_watches` for
subscribing the new user to a relevant space. Gate: don't report
onboarding as complete until the permission grant is confirmed by
re-reading it, not just because the grant call didn't error.

## Step 5 (offboarding) -- disable, don't delete, unless confirmed

Search for how to disable the user's account. Do not call a delete
operation instead unless the user has explicitly confirmed they want the
account permanently removed, not just deactivated -- disabling is
reversible, deletion typically is not.

## Step 6 (offboarding) -- remove memberships and revoke access

Delegate to `confluence_workflow_users_groups` for removing the user from
their groups, then to `confluence_workflow_permissions_restrictions` for
explicitly revoking any space/content permissions granted directly to
this user (not just relying on group removal, since directly-granted
permissions can outlive group membership). Remember that revoking
permissions there can be a replace-all write for content restrictions --
read the current state before writing, per that workflow's own gate.

## Step 7 -- summarize

Report exactly which steps were completed and confirmed, and anything
still outstanding (e.g. a permission grant the user asked to defer)
rather than a blanket "done."
