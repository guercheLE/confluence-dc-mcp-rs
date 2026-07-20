# Confluence workflow: webhooks

Covers a webhook's lifecycle: creating one, testing it, inspecting its
latest invocation and statistics, updating it, and deleting it.

Search for the capability in plain language (e.g. "search for how to
create a webhook" or "search for how to inspect a webhook's invocation
statistics") and read the schema `get` returns before calling -- confirm
the target URL and the events it should fire on with the user before
creating one, since a misconfigured webhook can fire unwanted requests
against a real external endpoint.

After creating a webhook, use the test operation (if the resolved schema
offers one) to confirm it actually reaches the target before considering
the setup complete, rather than trusting the create call's success alone.
