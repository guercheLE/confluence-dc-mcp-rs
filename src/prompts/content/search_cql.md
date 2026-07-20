# Confluence workflow: search & CQL

Two distinct search capabilities exist -- confirm which one fits before
searching for it:

- **Content search (CQL)** -- structured queries over content
  (type, space, title, label, ancestor, contributor, dates, text, etc.),
  combined with `AND`/`OR`/`NOT` and comparison operators. Use this
  whenever the user's need can be expressed as filters over content
  fields, e.g. "pages in space ENG labeled 'runbook' updated in the last
  30 days".
- **General entity search** -- a broader search across more than just
  content (e.g. spaces, users), for looser "find anything matching this
  text" needs.

Search for the capability in plain language (e.g. "search for how to
search content using CQL") and read the schema `get` returns for the exact
query parameter name and any pagination fields (typically a limit and a
start offset) before calling -- don't assume the parameter names from this
prose.

**Example CQL fragments** (illustrative only -- verify actual field/operator
support against the live schema before relying on one):
`type=page AND space=ENG`, `label="runbook"`, `title~"onboarding"`,
`lastmodified >= now("-30d")`.

If a query is likely to return many results, consider delegating the
search-and-filter step to a sub-task if your environment supports one, and
bring back only the distilled matches (e.g. "3 pages matched: ..."), not
the full raw listing.
