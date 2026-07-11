\newpage

# Get Info

**Natural language description**

Retrieve detailed metadata for a backup.

**Example**

```text
Get detailed information about the latest backup for the primary server
```

## Tool: /get_info

**Tool description**

Retrieve detailed backup metadata.

**Arguments**

- `server`: The pgmoneta server name.
- `backup_id`: Backup label or one of `newest`, `latest`, `oldest`.

**Behavior**

- Returns translated fields such as human-readable sizes and decoded compression and encryption names.
- `username` is required by the MCP API and is typically injected by `pgmoneta-mcp-client`.

**Examples**

```text
get_info {"server":"primary","backup_id":"latest"}
get_info {"server":"primary","backup_id":"oldest"}
get_info {"server":"primary","backup_id":"20260706113507"}
```

