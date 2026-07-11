\newpage

# Ping

**Natural language description**

Check pgmoneta server reachability.

**Example**

```text
Check whether pgmoneta is reachable
```

## Tool: /ping

**Tool description**

Check server reachability.

**Arguments**

- No tool-specific arguments beyond `username`.

**Behavior**

- Use this as the simplest health check before backup or restore operations.
- `username` is required by the MCP API and is typically injected by `pgmoneta-mcp-client`.

**Examples**

```text
ping {}
```

