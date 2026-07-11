\newpage

# Retain

**Natural language description**

Protect a backup from retention cleanup.

**Example**

```text
Retain the latest backup for the primary server and include dependent backups
```

## Tool: /retain

**Tool description**

Mark a backup as retained (protected).

**Arguments**

- `server`: The pgmoneta server name.
- `backup_id`: Backup label or one of `newest`, `latest`, `oldest`.
- Optional `cascade`: Whether dependent backups should also be retained.

**Behavior**

- If `cascade` is omitted, it defaults to `false`.
- `cascade=true` retains dependent backups as well.
- `username` is required by the MCP API and is typically injected by `pgmoneta-mcp-client`.

**Examples**

```text
retain {"server":"primary","backup_id":"latest","cascade":false}
retain {"server":"primary","backup_id":"latest","cascade":true}
retain {"server":"primary","backup_id":"latest"}
```

