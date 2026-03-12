---
name: tapo
description: Control TP-Link Tapo smart home devices (lights, plugs, strips) via [Tapo MCP](https://github.com/mihai-dinculescu/tapo/tree/main/tapo-mcp).
metadata:
  {
    "openclaw":
      {
        "emoji": "📦",
        "requires": { "bins": ["mcporter"] },
        "install":
          [
            {
              "id": "node",
              "kind": "node",
              "package": "mcporter",
              "bins": ["mcporter"],
              "label": "Install mcporter (node)",
            },
          ],
      },
  }
---

# Tapo

Control Tapo devices using `mcporter call tapo.<tool>`.

## Setup

You need a [Tapo MCP](https://github.com/mihai-dinculescu/tapo/tree/main/tapo-mcp) server running on your network (HTTP transport). Bearer token auth is recommended.

1. **Add the Tapo server**:
   ```bash
   mcporter config add tapo http://<TAPO_MCP_IP> \
     --transport http \
     --header "Authorization=Bearer <YOUR_TOKEN>" \
     --scope home
   ```

2. **Verify**:
   ```bash
   mcporter list tapo --schema
   ```
   You should see `list_devices`, `check_device`, and `set_device_state`.

See [references/setup.md](references/setup.md) for the full walkthrough, config management, and troubleshooting.

## Tools

### list_devices

List all Tapo devices on the network.

```bash
mcporter call tapo.list_devices
```

Returns each device's `id`, `name`, `model`, `ip`, `capabilities`, and `children` (for power strips).

### check_device

Verify a device ID matches at a given IP.

```bash
mcporter call tapo.check_device id="<DEVICE_ID>" ip="<IP>"
```

### set_device_state

Turn a device on or off. Automatically runs `check_device` first.

```bash
# Turn on
mcporter call tapo.set_device_state id="<DEVICE_ID>" ip="<IP>" capability='{"OnOff": true}'

# Turn off
mcporter call tapo.set_device_state id="<DEVICE_ID>" ip="<IP>" capability='{"OnOff": false}'
```

## Usage rules

1. Always run `list_devices` first if you don't have a recent device list. Cache results for up to 30 minutes.
2. Use the device `id` and `ip` from the list — never guess or hardcopy these values.
3. For power strips (e.g. P304M), children have their own `id`. Use the child `id` with the parent's `ip`.
