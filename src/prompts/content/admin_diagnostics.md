# Confluence workflow: admin & diagnostics

A thin pointer, not a guided multi-step flow: for reindexing/unindexing
the search index (and checking reindex status), cluster node status,
audit records, the instance's access mode, instance metrics, server
information, or the global color scheme, search for the specific
capability in plain language (e.g. "search for how to trigger a reindex"
or "search for how to check cluster node status") and read the schema
`get` returns before calling. These are mostly read-only or rarely-run
maintenance operations, each simple enough on its own that it doesn't
need a numbered-step scaffold -- if a listing this surfaces is large,
consider delegating that one lookup to a sub-task if your environment
supports one, and bring back only the distilled answer.
