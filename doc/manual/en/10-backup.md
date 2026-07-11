\newpage

# Backup

**Natural language description**

Take a backup of a pgmoneta server.

**Example**

```text
Take a full backup of the primary server
```

## Tool: /backup

**Tool description**

Create a full or incremental backup.

**Arguments**

- `server`: The pgmoneta server name.
- Optional `backup_id`: Base backup label for incremental backup.

**Behavior**

- Without `backup_id`, the tool creates a full backup.
- With `backup_id`, the tool creates an incremental backup based on that backup.
- `username` is required by the MCP API and is typically injected by `pgmoneta-mcp-client`.

**Examples**

```text
backup {"server":"primary"}
backup {"server":"primary","backup_id":"latest"}
backup {"server":"primary","backup_id":"20260706113507"}
```

