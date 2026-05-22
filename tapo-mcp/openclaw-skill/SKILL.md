---
name: tapo
description: Control TP-Link Tapo smart home devices (lights, plugs, power strips, hubs and sensors, cameras) via [Tapo MCP](https://github.com/mihai-dinculescu/tapo/tree/main/tapo-mcp).
version: 0.4.1
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

You need a [Tapo MCP](https://github.com/mihai-dinculescu/tapo/tree/main/tapo-mcp) server (v0.4.0 or later) running on your network (HTTP transport). Bearer token auth is recommended.

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
   You should see `list_devices`, `check_device`, `get_device_state`, `control_device`, and `take_snapshot`.

See [references/setup.md](references/setup.md) for the full walkthrough, config management, and troubleshooting.

## Tools

### list_devices

List all Tapo devices on the network.

```bash
npx mcporter call tapo.list_devices
```

Returns each device's `id`, `name`, `model`, `ip`, `set_capabilities`, `get_capabilities`, and `children` (for power strips and the H100 hub).

### check_device

Verify a device ID matches at a given IP.

```bash
npx mcporter call tapo.check_device id="<DEVICE_ID>" ip="<IP>"
```

### get_device_state

Get a device's current state. Automatically runs `check_device` first.

```bash
# Device info
npx mcporter call tapo.get_device_state id="<DEVICE_ID>" ip="<IP>" capability='{"type": "DeviceInfo"}'

# Trigger logs (S200, T100, T110, T300 hub children) — newest first
npx mcporter call tapo.get_device_state id="<CHILD_ID>" ip="<HUB_IP>" capability='{"type": "TriggerLogs", "page_size": 20, "start_id": 0}'

# Last 24h temperature/humidity records (T310, T315 hub children)
npx mcporter call tapo.get_device_state id="<CHILD_ID>" ip="<HUB_IP>" capability='{"type": "TemperatureHumidityRecords"}'
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

### take_snapshot

Capture a still JPEG snapshot from a Tapo camera (~640x360). Automatically runs `check_device` first. Requires `TAPO_MCP_CAMERA_USERNAME` and `TAPO_MCP_CAMERA_PASSWORD` configured on the server (Camera Settings > Advanced Settings > Camera Account in the Tapo app).

```bash
npx mcporter call tapo.take_snapshot id="<DEVICE_ID>" ip="<IP>"
```

## Usage rules

1. Always run `list_devices` first if you don't have a recent device list. Cache results for up to 30 minutes.
2. Use the device `id` and `ip` from the list — never guess or hardcopy these values.
3. For power strips (e.g. P304M) and the H100 hub, children have their own `id`. Use the child `id` with the parent's `ip`.
