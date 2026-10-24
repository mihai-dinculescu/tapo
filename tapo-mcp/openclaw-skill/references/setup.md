# Tapo Skill Setup

Detailed guide for connecting a [Tapo MCP](https://github.com/mihai-dinculescu/tapo/tree/main/tapo-mcp) server to OpenClaw via mcporter.

For deploying the Tapo MCP server itself (Docker, Kubernetes, configuration), see [tapo-mcp-setup.md](tapo-mcp-setup.md).

## Prerequisites

- A running [Tapo MCP](https://github.com/mihai-dinculescu/tapo/tree/main/tapo-mcp) server on your network (HTTP transport)
- The server URL (e.g. `http://192.168.1.100`)
- A Bearer auth token, if one was configured (highly recommended; required when the server binds to a non-loopback address)
- When connecting by LAN IP or hostname (anything other than loopback), the server's `TAPO_MCP_ALLOWED_HOSTS` must include that host, or requests are rejected with `403 Forbidden` (see [tapo-mcp-setup.md](tapo-mcp-setup.md))

## Step 1: Add the Tapo MCP server

```bash
npx mcporter config add tapo http://<TAPO_MCP_IP> \
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
npx mcporter list tapo --schema
```

You should see these five tools (the printed descriptions are longer than this summary):

| Tool               | Description                           | Parameters                 |
| ------------------ | ------------------------------------- | -------------------------- |
| `list_devices`     | List all available Tapo devices       | (none)                     |
| `check_device`     | Verify a device ID matches at an IP   | `id`, `ip`                 |
| `get_device_state` | Get a device's current state          | `id`, `ip`, `capability`   |
| `control_device`   | Control a device                      | `id`, `ip`, `capabilities` |
| `take_snapshot`    | Capture a JPEG snapshot from a camera | `id`, `ip`                 |

## Step 3: Test a tool call

```bash
# List all devices on the network
npx mcporter call tapo.list_devices

# Get a device's current state
npx mcporter call tapo.get_device_state \
  id="<DEVICE_ID>" \
  ip="<DEVICE_IP>" \
  capability='{"type": "DeviceInfo"}'

# Turn a device off
npx mcporter call tapo.control_device \
  id="<DEVICE_ID>" \
  ip="<DEVICE_IP>" \
  capabilities='[{"type": "OnOff", "value": false}]'
```

## Managing the config

```bash
# List all configured servers
npx mcporter config list

# Inspect the tapo server config
npx mcporter config get tapo

# Remove the server
npx mcporter config remove tapo

# Validate config
npx mcporter config doctor
```

## Troubleshooting

- **Connection refused**: Verify the Tapo MCP server is running and reachable (`curl http://<IP>/`).
- **401 Unauthorized**: Check the Bearer token is correct.
- **403 Forbidden**: The `Host` you connect with isn't in the server's allowlist. Add the exact hostname or `host:port` you use to `TAPO_MCP_ALLOWED_HOSTS` on the server (see [tapo-mcp-setup.md](tapo-mcp-setup.md)).
- **Device discovery unavailable**: No discovery has succeeded since the server started. Right after startup, the first one is still running; it takes `TAPO_MCP_DISCOVERY_TIMEOUT` seconds (default 5) plus the time to log in to every device and read the hub and power strip child lists, so wait a little longer than that and retry. Otherwise every discovery so far has failed and the error gives the reason, which is a problem with `TAPO_MCP_DISCOVERY_TARGET` or the server's network, not with the credentials: a wrong Tapo password still produces a list, with each device under `errors`. Once a discovery has succeeded, a later failure no longer produces this error; `list_devices` keeps returning the last successful list, with the failure in `refresh_error`.
- **Device not found**: Nothing answered at that `ip`. The device is off or unreachable, or its IP changed. Run `list_devices` again and use the `id` and `ip` it returns.
- **Device mismatch**: A device answered at that `ip`, but neither it nor any of its children has that `id`. The IP now belongs to another device, or the child was unpaired from the hub. Run `list_devices` again and use the `id` and `ip` it returns.
- **Failed to discover device at `<ip>`**: The device answered, but the server could not log in to it or read it. With a wrong `TAPO_MCP_USERNAME` or `TAPO_MCP_PASSWORD` every device fails this way, and `list_devices` reports them all under `errors`. The message after the colon gives the device's reason.
- **Device timed out**: Finding the device at that `ip`, logging in and making the request together took longer than `TAPO_MCP_DISCOVERY_TIMEOUT` plus 60 seconds. That limit counts from when the call starts contacting the device, after any wait for another call to the same `ip` or for a background discovery to finish, so the call can take longer overall. The device is slow or stopped responding partway through; retry, and if it keeps happening check its Wi-Fi signal and run `list_devices` again to confirm it is still at that `ip`. For `control_device`, the error says how many capabilities were applied before the timeout.
- **Tools not showing**: Run `npx mcporter list tapo --schema` to verify the server responds. Check `npx mcporter config doctor` for config issues.
- **Agent can't find tools**: Ensure the mcporter skill is enabled — it's bundled with OpenClaw by default.
