\newpage

# Restore

**Natural language description**

Restore a backup into a target directory.

**Example**

```text
Restore the latest backup for the primary server into /tmp/restore
```

## Tool: /restore

**Tool description**

Restore a backup into a directory. Position defaults to `current` when omitted.

**Arguments**

- `server`: The pgmoneta server name.
- `backup_id`: Backup label or one of `newest`, `latest`, `oldest`.
- `directory`: Target restore directory.
- Optional position controls: `current`, `name`, `xid`, `time`, `lsn`, `inclusive`, `timeline`, `action`, `primary`, `replica`.

**Behavior**

- If no position controls are provided, pgmoneta_mcp uses `current`.
- `name`, `xid`, `time`, `lsn`, and `timeline` are different restore selection modes.
- `action` controls post-restore behavior such as `pause` or `shutdown`.
- `primary` and `replica` describe the resulting cluster role.
- `username` is required by the MCP API and is typically injected by `pgmoneta-mcp-client`.

**Examples**

```text
restore {"server":"primary","backup_id":"latest","directory":"/tmp/restore"}
restore {"server":"primary","backup_id":"latest","directory":"/tmp/restore","name":"recovery-label"}
restore {"server":"primary","backup_id":"latest","directory":"/tmp/restore","xid":"734560"}
restore {"server":"primary","backup_id":"latest","directory":"/tmp/restore","time":"2026-07-06 11:30:00"}
restore {"server":"primary","backup_id":"latest","directory":"/tmp/restore","lsn":"0/5000000"}
restore {"server":"primary","backup_id":"latest","directory":"/tmp/restore","timeline":"2","inclusive":"true","action":"shutdown"}
restore {"server":"primary","backup_id":"latest","directory":"/tmp/restore","primary":true}
restore {"server":"primary","backup_id":"latest","directory":"/tmp/restore","replica":true}
```

