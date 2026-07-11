\newpage

# Decompress

**Natural language description**

Decompress a file on the pgmoneta host.

**Example**

```text
Decompress /tmp/base.tar.zst on the server
```

## Tool: /decompress

**Tool description**

Decompress a file.

**Arguments**

- `file_path`: Path of the file to decompress on the pgmoneta host.

**Behavior**

- The decompression method follows the file and configured server behavior.
- `username` is required by the MCP API and is typically injected by `pgmoneta-mcp-client`.

**Examples**

```text
decompress {"file_path":"/tmp/base.tar.zst"}
```

