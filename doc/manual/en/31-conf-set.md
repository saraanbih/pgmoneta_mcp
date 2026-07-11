\newpage

# Conf Set

**Natural language description**

Update a single runtime configuration value.

**Example**

```text
Set pgmoneta log_level to debug
```

## Tool: /conf_set

**Tool description**

Set a single configuration key/value.

**Arguments**

- `config_key`: Name of the configuration entry to update.
- `config_value`: New value to assign.

**Behavior**

- This updates one configuration value per call.
- A common workflow is `conf_ls` -> `conf_get` -> `conf_set` -> `conf_reload`.
- `username` is required by the MCP API and is typically injected by `pgmoneta-mcp-client`.

**Examples**

```text
conf_set {"config_key":"retention_days","config_value":"7"}
conf_set {"config_key":"log_level","config_value":"debug"}
```

