# Confluence workflow: labels

Covers listing, adding, and removing labels on a piece of content or on a
space, plus browsing popular/recent labels and labels related to a given
one. The same shape of operations exists twice -- once scoped to content,
once scoped to a space -- so confirm which resource the user means before
searching.

Search for the capability in plain language (e.g. "search for how to add
a label to a page" or "search for how to list popular labels in a space")
and read the schema `get` returns before calling.

**Gotcha:** adding a label is typically additive (existing labels stay),
but confirm from the schema whether the specific remove operation you find
takes a single label name or a list -- removing by the wrong shape can
silently no-op or affect more labels than intended.

Verify by re-listing the content's or space's labels after a change rather
than trusting a non-error response alone.
