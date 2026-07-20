# Confluence workflow: attachments

Covers uploading, listing, updating, deleting, and moving attachments on a
piece of content, plus reading an attachment's extracted text and removing
a specific version of it.

Search for the capability in plain language (e.g. "search for how to
upload an attachment to a page" or "search for how to move an attachment
to a different piece of content") and read the schema `get` returns before
calling -- attachment endpoints take the parent content's id as a path
parameter, so confirm that id first (see `confluence_workflow_content` if
it isn't already known).

**Gotchas worth checking before you call:**

- Uploading a new version of an attachment with the same filename is
  usually how you "update" one, rather than a distinct update-in-place
  call for the file bytes themselves -- confirm which the target operation
  actually expects from its schema.
- Removing a single version is different from removing the attachment
  entirely -- confirm which the user wants.

After uploading, verify by re-fetching the attachment (or its new version)
rather than trusting a non-error response alone.
