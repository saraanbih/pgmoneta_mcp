\newpage

# Clear

**Natural language description**

Reset Prometheus-related statistics.

**Example**

```text
Clear pgmoneta metrics statistics
```

## Tool: /clear

**Tool description**

Clear Prometheus-related statistics.

**Arguments**

- No tool-specific arguments beyond `username`.

**Behavior**

- This tool resets the pgmoneta statistics exposed through the metrics endpoint.
- `username` is required by the MCP API and is typically injected by `pgmoneta-mcp-client`.

**Examples**

```text
clear {}
```

