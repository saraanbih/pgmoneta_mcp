\newpage

# Encrypt

**Natural language description**

Encrypt a file on the pgmoneta host.

**Example**

```text
Encrypt /tmp/base.tar on the server
```

## Tool: /encrypt

**Tool description**

Encrypt a file.

**Arguments**

- `file_path`: Path of the file to encrypt on the pgmoneta host.

**Behavior**

- Encryption uses the configured pgmoneta encryption settings.
- `username` is required by the MCP API and is typically injected by `pgmoneta-mcp-client`.

**Examples**

```text
encrypt {"file_path":"/tmp/base.tar"}
```

