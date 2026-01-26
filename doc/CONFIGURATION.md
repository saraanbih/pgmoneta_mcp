# pgmoneta-mcp configuration

The configuration is loaded from either the path specified by the `-c` flag or `/etc/pgmoneta-mcp/pgmoneta-mcp.conf`.

The configuration of `pgmoneta` is split into sections using the `[` and `]` characters.

The main section, called `[pgmoneta_mcp]`, is where you configure the overall properties
of the MCP server.

The other section, called `[pgmoneta]`, is where you configure connection with `pgmoneta` server.

## [pgmoneta_mcp]

| Property | Default | Unit | Required | Description |
| :------- | :------ | :--- | :------- | :---------- |
| port | 8000 | Int | No | The port MCP server starts on |
| log_type | console | String | No | The logging type (console, file, syslog) |
| log_level | info | String | No | The logging level, any of the strings `trace`, `debug`, `info`, `warn` and `error`|
| log_path | pgmoneta_mcp.log | String | No | The log file location |

## [pgmoneta]

| Property | Default | Unit | Required | Description |
| :------- | :------ | :--- | :------- | :---------- |
| host | | String | Yes | The address of pgmoneta instance |
| port | | Int | Yes | The port of pgmoneta instance |