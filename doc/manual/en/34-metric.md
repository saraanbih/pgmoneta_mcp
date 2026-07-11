\newpage

# Metric

**Natural language description**

Return one metric by name, with optional label filters.

**Example**

```text
Show the retention metric for server primary
```

## Tool: /metric

**Tool description**

Return one metric by name, optionally filtered by labels.

**Arguments**

- `name`: Prometheus metric name.
- Optional `attributes`: Exact label filters.
- Optional `labels`: Alias for `attributes`.

**Behavior**

- Use either `attributes` or `labels`, not both.
- Filter values may be strings, numbers, or booleans.
- If one sample matches, the tool returns that sample line.
- If multiple samples match, the tool returns all matching lines.
- If no metric name matches, the tool fails.
- `username` is required by the MCP API and is typically injected by `pgmoneta-mcp-client`.

**Examples**

```text
metric {"name":"pgmoneta_version"}
metric {"name":"pgmoneta_retention_server","attributes":{"server":"primary"}}
metric {"name":"pgmoneta_retention_server","labels":{"server":"primary"}}
```

