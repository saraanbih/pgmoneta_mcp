\newpage

# Delete

**Natural language description**

Delete a backup from pgmoneta.

**Example**

```text
Delete the oldest backup for the primary server
```

## Tool: /delete

**Tool description**

Delete a backup. `force` defaults to `false`.

**Arguments**

- `server`: The pgmoneta server name.
- `backup_id`: Backup label or one of `newest`, `latest`, `oldest`.
- Optional `force`: Force deletion, default `false`.

**Behavior**

- If `force` is omitted, pgmoneta_mcp sends `false`.
- Use `force` only when you want to override normal deletion safeguards.
- `username` is required by the MCP API and is typically injected by `pgmoneta-mcp-client`.

**Examples**

```text
delete {"server":"primary","backup_id":"oldest"}
delete {"server":"primary","backup_id":"oldest","force":true}
```

