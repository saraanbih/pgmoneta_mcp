\newpage

# Status

**Natural language description**

Get the current pgmoneta status.

**Example**

```text
Show detailed status for pgmoneta
```

## Tool: /status

**Tool description**

Get status in compact or detailed view.

**Arguments**

- `in_details`: `false` for summary output, `true` for detailed output.

**Behavior**

- Summary mode returns high-level information such as version and storage totals.
- Detailed mode adds operational data such as backup sizes, WAL, retention, hot standby size, and workers.
- `username` is required by the MCP API and is typically injected by `pgmoneta-mcp-client`.

**Examples**

```text
status {"in_details":false}
status {"in_details":true}
```

