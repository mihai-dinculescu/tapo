---
name: tapo
description: Control TP-Link Tapo smart home devices (lights, plugs, strips) via [Tapo MCP](https://github.com/mihai-dinculescu/tapo/tree/main/tapo-mcp).
metadata:
  {
    "openclaw":
      {
        "emoji": "📦",
        "requires": { "bins": ["npx"] },
      },
  }
---

# Tapo

Control Tapo devices using `npx mcporter call tapo.<tool>`.

## Setup

You need a [Tapo MCP](https://github.com/mihai-dinculescu/tapo/tree/main/tapo-mcp) server running on your network (HTTP transport). Bearer token auth is recommended.

1. **Add the Tapo server**:

   ```bash
   npx mcporter config add tapo http://<TAPO_MCP_IP> \
     --transport http \
     --header "Authorization=Bearer <YOUR_TOKEN>" \
     --scope home
   ```

2. **Verify**:
   ```bash
   npx mcporter list tapo --schema
   ```
   You should see `list_devices`, `check_device`, `get_device_state`, and `control_device`.

See [references/setup.md](references/setup.md) for the full walkthrough, config management, and troubleshooting.

## Tools

### list_devices

List all Tapo devices on the network.

```bash
npx mcporter call tapo.list_devices
```

Returns each device's `id`, `name`, `model`, `ip`, `set_capabilities`, `get_capabilities`, and `children` (for power strips).

### check_device

Verify a device ID matches at a given IP.

```bash
npx mcporter call tapo.check_device id="<DEVICE_ID>" ip="<IP>"
```

### get_device_state

Get a device's current state. Automatically runs `check_device` first.

```bash
npx mcporter call tapo.get_device_state id="<DEVICE_ID>" ip="<IP>" capability='{"type": "DeviceInfo"}'
```

### control_device

Control a device. Automatically runs `check_device` first.

```bash
# Turn on
npx mcporter call tapo.control_device id="<DEVICE_ID>" ip="<IP>" capabilities='[{"type": "OnOff", "value": true}]'

# Turn off
npx mcporter call tapo.control_device id="<DEVICE_ID>" ip="<IP>" capabilities='[{"type": "OnOff", "value": false}]'

# Set brightness (1-100, lights only)
npx mcporter call tapo.control_device id="<DEVICE_ID>" ip="<IP>" capabilities='[{"type": "Brightness", "value": 50}]'

# Set color (color lights only)
npx mcporter call tapo.control_device id="<DEVICE_ID>" ip="<IP>" capabilities='[{"type": "Color", "value": "Coral"}]'

# Set multiple capabilities at once
npx mcporter call tapo.control_device id="<DEVICE_ID>" ip="<IP>" capabilities='[{"type": "Color", "value": "Coral"}, {"type": "Brightness", "value": 50}]'
```

## Usage rules

1. Always run `list_devices` first if you don't have a recent device list. Cache results for up to 30 minutes.
2. Use the device `id` and `ip` from the list — never guess or hardcopy these values.
3. For power strips (e.g. P304M), children have their own `id`. Use the child `id` with the parent's `ip`.
