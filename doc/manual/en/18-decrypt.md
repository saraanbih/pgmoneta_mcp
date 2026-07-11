\newpage

# Decrypt

**Natural language description**

Decrypt a file on the pgmoneta host.

**Example**

```text
Decrypt /tmp/base.tar.aes on the server
```

## Tool: /decrypt

**Tool description**

Decrypt a file.

**Arguments**

- `file_path`: Path of the file to decrypt on the pgmoneta host.

**Behavior**

- Decryption uses the encryption setup configured on the pgmoneta side.
- `username` is required by the MCP API and is typically injected by `pgmoneta-mcp-client`.

**Examples**

```text
decrypt {"file_path":"/tmp/base.tar.aes"}
```

