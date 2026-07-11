\newpage

# Shutdown

**Natural language description**

Shutdown pgmoneta.

**Example**

```text
Shutdown pgmoneta now
```

## Tool: /shutdown

**Tool description**

Shutdown pgmoneta.

**Arguments**

- No tool-specific arguments beyond `username`.

**Behavior**

- Use this carefully, because subsequent backup-related operations fail until pgmoneta is started again.
- `username` is required by the MCP API and is typically injected by `pgmoneta-mcp-client`.

**Examples**

```text
shutdown {}
```

