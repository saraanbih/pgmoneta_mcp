\newpage

# Get Metrics

**Natural language description**

Return all Prometheus metrics from pgmoneta.

**Example**

```text
Show all pgmoneta metrics
```

## Tool: /get_metrics

**Tool description**

Return all metrics.

**Arguments**

- No tool-specific arguments beyond `username`.

**Behavior**

- Returns the full Prometheus exposition from the configured pgmoneta metrics endpoint.
- Use `metric` instead when you only want one metric name or one filtered sample.
- `username` is required by the MCP API and is typically injected by `pgmoneta-mcp-client`.

**Examples**

```text
get_metrics {}
```

