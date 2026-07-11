\newpage

# Conf List

**Natural language description**

List available runtime configuration entries.

**Example**

```text
List available pgmoneta configuration entries
```

## Tool: /conf_ls

**Tool description**

List available configuration entries.

**Arguments**

- No tool-specific arguments beyond `username`.

**Behavior**

- Use this when you want an overview of configuration entries before calling `conf_get` or `conf_set`.
- `username` is required by the MCP API and is typically injected by `pgmoneta-mcp-client`.

**Examples**

```text
conf_ls {}
```

