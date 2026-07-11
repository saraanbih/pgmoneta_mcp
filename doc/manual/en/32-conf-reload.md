\newpage

# Conf Reload

**Natural language description**

Reload configuration from server files.

**Example**

```text
Reload pgmoneta configuration files now
```

## Tool: /conf_reload

**Tool description**

Reload configuration from server files.

**Arguments**

- No tool-specific arguments beyond `username`.

**Behavior**

- Reloads the active pgmoneta configuration without changing tool input.
- `username` is required by the MCP API and is typically injected by `pgmoneta-mcp-client`.

**Examples**

```text
conf_reload {}
```

