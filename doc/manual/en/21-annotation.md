\newpage

# Annotation

**Natural language description**

Annotate a backup with a comment key.

**Example**

```text
Add the ticket annotation to the latest backup
```

## Tool: /annotate_backup

**Tool description**

Add, update, or remove a backup annotation.

**Arguments**

- `server`: The pgmoneta server name.
- `backup_id`: Backup label or one of `newest`, `latest`, `oldest`.
- `action`: One of `add`, `update`, `remove`.
- `key`: Annotation key.
- `comment`: Required for `add` and `update`; omitted for `remove`.

**Behavior**

- The tool normalizes the action name before execution.
- `add` and `update` fail if `comment` is missing or empty.
- `remove` ignores any comment value.
- `username` is required by the MCP API and is typically injected by `pgmoneta-mcp-client`.

**Examples**

```text
annotate_backup {"server":"primary","backup_id":"latest","action":"add","key":"ticket","comment":"before release"}
annotate_backup {"server":"primary","backup_id":"latest","action":"update","key":"ticket","comment":"after validation"}
annotate_backup {"server":"primary","backup_id":"latest","action":"remove","key":"ticket"}
```

