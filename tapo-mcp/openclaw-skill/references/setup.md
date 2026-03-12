# Tapo Skill Setup

Detailed guide for connecting a [Tapo MCP](https://github.com/mihai-dinculescu/tapo/tree/main/tapo-mcp) server to OpenClaw via mcporter.

For deploying the Tapo MCP server itself (Docker, Kubernetes, configuration), see [tapo-mcp-setup.md](tapo-mcp-setup.md).

## Prerequisites

- A running [Tapo MCP](https://github.com/mihai-dinculescu/tapo/tree/main/tapo-mcp) server on your network (HTTP transport)
- The server URL (e.g. `http://192.168.1.100`)
- A Bearer auth token, if one was configured (highly recommended)

## Step 1: Add the Tapo MCP server

```bash
mcporter config add tapo http://<TAPO_MCP_IP> \
  --transport http \
  --header "Authorization=Bearer <YOUR_TOKEN>" \
  --scope home
```

Replace:
- `<TAPO_MCP_IP>` — the IP/hostname of your Tapo MCP server
- `<YOUR_TOKEN>` — the Bearer token for authentication

The `--scope home` flag writes the config to `~/.mcporter/mcporter.json`, making it available system-wide.

If you configured a Bearer token, restrict file permissions to prevent other local users from reading it:

```bash
chmod 600 ~/.mcporter/mcporter.json
```

## Step 2: Verify the connection

```bash
mcporter list tapo --schema
```

You should see:

| Tool               | Description                          | Parameters              |
|--------------------|--------------------------------------|-------------------------|
| `list_devices`     | List all available Tapo devices      | (none)                  |
| `check_device`     | Verify a device ID matches at an IP  | `id`, `ip`              |
| `set_device_state` | Set a device's state (on/off)        | `id`, `ip`, `capability`|

## Step 3: Test a tool call

```bash
# List all devices on the network
mcporter call tapo.list_devices

# Turn a device off
mcporter call tapo.set_device_state \
  id="<DEVICE_ID>" \
  ip="<DEVICE_IP>" \
  capability='{"OnOff": false}'
```

## Managing the config

```bash
# List all configured servers
mcporter config list

# Inspect the tapo server config
mcporter config get tapo

# Remove the server
mcporter config remove tapo

# Validate config
mcporter config doctor
```

## Troubleshooting

- **Connection refused**: Verify the Tapo MCP server is running and reachable (`curl http://<IP>/`).
- **401 Unauthorized**: Check the Bearer token is correct.
- **Tools not showing**: Run `mcporter list tapo --schema` to verify the server responds. Check `mcporter config doctor` for config issues.
- **Agent can't find tools**: Ensure the mcporter skill is enabled — it's bundled with OpenClaw by default.
