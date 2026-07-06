\newpage

# Tools

## Available tools

The MCP server currently exposes the following tools:

- `annotate_backup`
- `archive`
- `backup`
- `clear`
- `compress`
- `conf_get`
- `conf_ls`
- `conf_reload`
- `conf_set`
- `decompress`
- `decrypt`
- `delete`
- `encrypt`
- `expunge`
- `get_info`
- `get_metrics`
- `list_backups`
- `metric`
- `ping`
- `restore`
- `retain`
- `set_mode`
- `shutdown`
- `status`
- `verify`

## Tool input rules

- All tools require `username`.
- Tools that target a backup commonly accept backup aliases: `newest`, `latest`, `oldest`.
- When using `pgmoneta-mcp-client`, `username` is normally injected from the selected users file.

### annotate_backup

Add, update, or remove a backup annotation.

Arguments:

- `server`: The pgmoneta server name
- `backup_id`: Backup label or one of `newest`, `latest`, `oldest`
- `action`: One of `add`, `update`, `remove`
- `key`: Annotation key
- `comment`: Required for `add` and `update`; omitted for `remove`

Behavior:

- The tool normalizes the action name before execution.
- `add` and `update` fail if `comment` is missing or empty.
- `remove` ignores any comment value.

Examples:

```text
annotate_backup {"server":"primary","backup_id":"latest","action":"add","key":"ticket","comment":"before release"}
annotate_backup {"server":"primary","backup_id":"latest","action":"update","key":"ticket","comment":"after validation"}
annotate_backup {"server":"primary","backup_id":"latest","action":"remove","key":"ticket"}
```

### archive

Archive a backup to a directory. Position defaults to `current` when not provided.

Arguments:

- `server`: The pgmoneta server name
- `backup_id`: Backup label or one of `newest`, `latest`, `oldest`
- `directory`: Target archive directory
- Optional position controls: `current`, `name`, `xid`, `time`, `lsn`, `inclusive`, `timeline`, `action`, `primary`, `replica`

Behavior:

- If no position controls are supplied, pgmoneta_mcp uses `current`.
- `action` controls what happens after archive processing, for example `pause`.
- `primary` and `replica` describe the target role and are independent of the server name.

Examples:

```text
archive {"server":"primary","backup_id":"latest","directory":"/tmp/archive"}
archive {"server":"primary","backup_id":"latest","directory":"/tmp/archive","name":"recovery-label"}
archive {"server":"primary","backup_id":"latest","directory":"/tmp/archive","xid":"734560"}
archive {"server":"primary","backup_id":"latest","directory":"/tmp/archive","time":"2026-07-06 11:30:00"}
archive {"server":"primary","backup_id":"latest","directory":"/tmp/archive","lsn":"0/5000000"}
archive {"server":"primary","backup_id":"latest","directory":"/tmp/archive","timeline":"2","inclusive":"true","action":"pause"}
archive {"server":"primary","backup_id":"latest","directory":"/tmp/archive","primary":true}
archive {"server":"primary","backup_id":"latest","directory":"/tmp/archive","replica":true}
```

### backup

Create a full or incremental backup.

Arguments:

- `server`: The pgmoneta server name
- Optional `backup_id`: Base backup label for incremental backup

Behavior:

- Without `backup_id`, the tool creates a full backup.
- With `backup_id`, the tool creates an incremental backup based on that backup.

Examples:

```text
backup {"server":"primary"}
backup {"server":"primary","backup_id":"latest"}
backup {"server":"primary","backup_id":"20260706113507"}
```

### clear

Clear Prometheus-related statistics.

Arguments:

- No tool-specific arguments beyond `username`

Behavior:

- This tool resets the pgmoneta statistics exposed through the metrics endpoint.

Examples:

```text
clear {}
```

### compress

Compress a file using configured compression.

Arguments:

- `file_path`: Path of the file to compress on the pgmoneta host

Behavior:

- The compression algorithm comes from the server-side pgmoneta configuration.

Examples:

```text
compress {"file_path":"/tmp/base.tar"}
```

### conf_get

Read full runtime configuration.

Arguments:

- No tool-specific arguments beyond `username`

Behavior:

- Returns detailed configuration content rather than only configuration keys.

Examples:

```text
conf_get {}
```

### conf_ls

List available configuration entries.

Arguments:

- No tool-specific arguments beyond `username`

Behavior:

- Use this when you want an overview of configuration entries before calling `conf_get` or `conf_set`.

Examples:

```text
conf_ls {}
```

### conf_reload

Reload configuration from server files.

Arguments:

- No tool-specific arguments beyond `username`

Behavior:

- Reloads the active pgmoneta configuration without changing tool input.

Examples:

```text
conf_reload {}
```

### conf_set

Set a single configuration key/value.

Arguments:

- `config_key`: Name of the configuration entry to update
- `config_value`: New value to assign

Behavior:

- This updates one configuration value per call.
- A common workflow is `conf_ls` -> `conf_get` -> `conf_set` -> `conf_reload`.

Examples:

```text
conf_set {"config_key":"retention_days","config_value":"7"}
conf_set {"config_key":"log_level","config_value":"debug"}
```

### decompress

Decompress a file.

Arguments:

- `file_path`: Path of the file to decompress on the pgmoneta host

Behavior:

- The decompression method follows the file and configured server behavior.

Examples:

```text
decompress {"file_path":"/tmp/base.tar.zst"}
```

### decrypt

Decrypt a file.

Arguments:

- `file_path`: Path of the file to decrypt on the pgmoneta host

Behavior:

- Decryption uses the encryption setup configured on the pgmoneta side.

Examples:

```text
decrypt {"file_path":"/tmp/base.tar.aes"}
```

### delete

Delete a backup. `force` defaults to `false`.

Arguments:

- `server`: The pgmoneta server name
- `backup_id`: Backup label or one of `newest`, `latest`, `oldest`
- Optional `force`: Force deletion, default `false`

Behavior:

- If `force` is omitted, pgmoneta_mcp sends `false`.
- Use `force` only when you want to override normal deletion safeguards.

Examples:

```text
delete {"server":"primary","backup_id":"oldest"}
delete {"server":"primary","backup_id":"oldest","force":true}
```

### encrypt

Encrypt a file.

Arguments:

- `file_path`: Path of the file to encrypt on the pgmoneta host

Behavior:

- Encryption uses the configured pgmoneta encryption settings.

Examples:

```text
encrypt {"file_path":"/tmp/base.tar"}
```

### expunge

Remove retention protection from a backup.

Arguments:

- `server`: The pgmoneta server name
- `backup_id`: Backup label or one of `newest`, `latest`, `oldest`
- Optional `cascade`: Whether dependent backups should also be expunged

Behavior:

- If `cascade` is omitted, it defaults to `false`.
- `cascade=true` applies expunge to dependent backups as well.

Examples:

```text
expunge {"server":"primary","backup_id":"latest","cascade":false}
expunge {"server":"primary","backup_id":"latest","cascade":true}
expunge {"server":"primary","backup_id":"latest"}
```

### get_info

Retrieve detailed backup metadata.

Arguments:

- `server`: The pgmoneta server name
- `backup_id`: Backup label or one of `newest`, `latest`, `oldest`

Behavior:

- Returns translated fields such as human-readable sizes and decoded compression and encryption names.

Examples:

```text
get_info {"server":"primary","backup_id":"latest"}
get_info {"server":"primary","backup_id":"oldest"}
get_info {"server":"primary","backup_id":"20260706113507"}
```

### get_metrics

Return all metrics.

Arguments:

- No tool-specific arguments beyond `username`

Behavior:

- Returns the full Prometheus exposition from the configured pgmoneta metrics endpoint.
- Use `metric` instead when you only want one metric name or one filtered sample.

Examples:

```text
get_metrics {}
```

### list_backups

List backups with sort control. Default sort is ascending.

Arguments:

- `server`: The pgmoneta server name
- Optional `sort`: `asc` or `desc`

Behavior:

- If `sort` is omitted, empty, or effectively null, pgmoneta_mcp uses `asc`.

Examples:

```text
list_backups {"server":"primary"}
list_backups {"server":"primary","sort":"asc"}
list_backups {"server":"primary","sort":"desc"}
```

### metric

Return one metric by name, optionally filtered by labels.

Arguments:

- `name`: Prometheus metric name
- Optional `attributes`: Exact label filters
- Optional `labels`: Alias for `attributes`

Behavior:

- Use either `attributes` or `labels`, not both.
- Filter values may be strings, numbers, or booleans.
- If one sample matches, the tool returns that sample line.
- If multiple samples match, the tool returns all matching lines.
- If no metric name matches, the tool fails.

Examples:

```text
metric {"name":"pgmoneta_version"}
metric {"name":"pgmoneta_retention_server","attributes":{"server":"primary"}}
metric {"name":"pgmoneta_retention_server","labels":{"server":"primary"}}
```

### ping

Check server reachability.

Arguments:

- No tool-specific arguments beyond `username`

Behavior:

- Use this as the simplest health check before backup or restore operations.

Examples:

```text
ping {}
```

### restore

Restore a backup into a directory. Position defaults to `current` when omitted.

Arguments:

- `server`: The pgmoneta server name
- `backup_id`: Backup label or one of `newest`, `latest`, `oldest`
- `directory`: Target restore directory
- Optional position controls: `current`, `name`, `xid`, `time`, `lsn`, `inclusive`, `timeline`, `action`, `primary`, `replica`

Behavior:

- If no position controls are provided, pgmoneta_mcp uses `current`.
- `name`, `xid`, `time`, `lsn`, and `timeline` are different restore selection modes.
- `action` controls post-restore behavior such as `pause` or `shutdown`.
- `primary` and `replica` describe the resulting cluster role and are not aliases for the source server name.

Examples:

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

### retain

Mark a backup as retained (protected).

Arguments:

- `server`: The pgmoneta server name
- `backup_id`: Backup label or one of `newest`, `latest`, `oldest`
- Optional `cascade`: Whether dependent backups should also be retained

Behavior:

- If `cascade` is omitted, it defaults to `false`.
- `cascade=true` retains dependent backups as well.

Examples:

```text
retain {"server":"primary","backup_id":"latest","cascade":false}
retain {"server":"primary","backup_id":"latest","cascade":true}
retain {"server":"primary","backup_id":"latest"}
```

### set_mode

Set server mode.

Arguments:

- `server`: The pgmoneta server name
- `action`: Must be `online` or `offline`

Behavior:

- This switches the pgmoneta server mode for the named server.
- Invalid action names are rejected by pgmoneta.

Examples:

```text
set_mode {"server":"primary","action":"online"}
set_mode {"server":"primary","action":"offline"}
```

### shutdown

Shutdown pgmoneta.

Arguments:

- No tool-specific arguments beyond `username`

Behavior:

- Use this carefully, because subsequent backup-related operations fail until pgmoneta is started again.

Examples:

```text
shutdown {}
```

### status

Get status in compact or detailed view.

Arguments:

- `in_details`: `false` for summary output, `true` for detailed output

Behavior:

- Summary mode returns high-level information such as version and storage totals.
- Detailed mode adds more operational data such as backup sizes, WAL, retention, hot standby size, and workers.

Examples:

```text
status {"in_details":false}
status {"in_details":true}
```

### verify

Verify backup integrity, with optional output directory.

Arguments:

- `server`: The pgmoneta server name
- `backup_id`: Backup label or one of `newest`, `latest`, `oldest`
- Optional `directory`: Verification target directory, default `/tmp`

Behavior:

- If `directory` is omitted, pgmoneta_mcp uses `/tmp`.
- Use an explicit directory when verification artifacts should be kept in a controlled location.

Examples:

```text
verify {"server":"primary","backup_id":"latest"}
verify {"server":"primary","backup_id":"latest","directory":"/tmp/verify"}
```
