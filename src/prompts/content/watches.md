# Confluence workflow: watches

Covers checking whether a user is watching a piece of content or a space,
and adding or removing that watch. Confirm which resource (a specific
content id, or a space key) and, if relevant, which user (the current
user, or another one, if the caller has permission to manage others'
watches).

Search for the capability in plain language (e.g. "search for how to check
if a user is watching a page" or "search for how to add a space watch")
and read the schema `get` returns before calling.

Verify by re-checking the watch status after adding or removing one,
rather than trusting a non-error response alone.
