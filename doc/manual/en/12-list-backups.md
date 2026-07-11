\newpage

# List Backups

**Natural language description**

Show the backups available for a server.

**Example**

```text
List the backups for the primary server in descending order
```

## Tool: /list_backups

**Tool description**

List backups with sort control. Default sort is ascending.

**Arguments**

- `server`: The pgmoneta server name.
- Optional `sort`: `asc` or `desc`.

**Behavior**

- If `sort` is omitted, empty, or effectively null, pgmoneta_mcp uses `asc`.
- `username` is required by the MCP API and is typically injected by `pgmoneta-mcp-client`.

**Examples**

```text
list_backups {"server":"primary"}
list_backups {"server":"primary","sort":"asc"}
list_backups {"server":"primary","sort":"desc"}
```

