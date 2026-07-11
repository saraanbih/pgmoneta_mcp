\newpage

# Expunge

**Natural language description**

Remove retention protection from a backup.

**Example**

```text
Remove retention protection from the latest backup on primary
```

## Tool: /expunge

**Tool description**

Remove retention protection from a backup.

**Arguments**

- `server`: The pgmoneta server name.
- `backup_id`: Backup label or one of `newest`, `latest`, `oldest`.
- Optional `cascade`: Whether dependent backups should also be expunged.

**Behavior**

- If `cascade` is omitted, it defaults to `false`.
- `cascade=true` applies expunge to dependent backups as well.
- `username` is required by the MCP API and is typically injected by `pgmoneta-mcp-client`.

**Examples**

```text
expunge {"server":"primary","backup_id":"latest","cascade":false}
expunge {"server":"primary","backup_id":"latest","cascade":true}
expunge {"server":"primary","backup_id":"latest"}
```

