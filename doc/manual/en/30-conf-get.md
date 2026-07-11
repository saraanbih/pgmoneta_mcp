\newpage

# Conf Get

**Natural language description**

Read full runtime configuration.

**Example**

```text
Show the full pgmoneta runtime configuration
```

## Tool: /conf_get

**Tool description**

Read full runtime configuration.

**Arguments**

- No tool-specific arguments beyond `username`.

**Behavior**

- Returns detailed configuration content rather than only configuration keys.
- `username` is required by the MCP API and is typically injected by `pgmoneta-mcp-client`.

**Examples**

```text
conf_get {}
```

