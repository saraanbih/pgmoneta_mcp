\newpage

# Compress

**Natural language description**

Compress a file using the configured pgmoneta algorithm.

**Example**

```text
Compress /tmp/base.tar on the server
```

## Tool: /compress

**Tool description**

Compress a file using configured compression.

**Arguments**

- `file_path`: Path of the file to compress on the pgmoneta host.

**Behavior**

- The compression algorithm comes from the server-side pgmoneta configuration.
- `username` is required by the MCP API and is typically injected by `pgmoneta-mcp-client`.

**Examples**

```text
compress {"file_path":"/tmp/base.tar"}
```

