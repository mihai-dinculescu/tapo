---
name: tapo
description: Control TP-Link Tapo smart home devices (lights, plugs, power strips, hubs and their child devices, cameras) via [Tapo MCP](https://github.com/mihai-dinculescu/tapo/tree/main/tapo-mcp).
version: 0.6.1
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

You need a [Tapo MCP](https://github.com/mihai-dinculescu/tapo/tree/main/tapo-mcp) server (v0.6.1 or later) running on your network (HTTP transport). Bearer token auth is recommended (and required when the server binds to a non-loopback address). When connecting by LAN IP or hostname, the server's `TAPO_MCP_ALLOWED_HOSTS` must include that host, or requests get `403 Forbidden`.

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

Returns `discovered_at`, the time the discovery that produced the list started, and each device's `id`, `name`, `model`, `ip`, `set_capabilities`, `get_capabilities`, and `children` (for power strips and the H100, H110, H200 and H500 hubs). The server refreshes the list in the background (every 10 minutes by default), so the call returns immediately. Until a discovery has succeeded after the server starts, it fails with "Device discovery unavailable" and the message says why: either the first discovery is still running, so wait a little longer than `TAPO_MCP_DISCOVERY_TIMEOUT` seconds and retry, or every discovery so far has failed, in which case the message carries the reason and the retry interval, so report the reason instead of retrying. A device that joined the network after `discovered_at` appears after the next refresh, which is at most `TAPO_MCP_DISCOVERY_INTERVAL` seconds (600 by default) away. A device that answered the discovery but could not be logged in to is listed under `errors` with its `ip` and the reason. A hub or power strip whose children could not be read stays under `devices` with an empty `children` list, and the reason is under `errors` with the parent's `ip`. A Tapo device the server cannot control is listed under `unsupported`. When the most recent refresh failed, the result also includes `refresh_error` with the reason, and the devices are the ones from the last successful discovery.

### check_device

Verify a device ID matches at a given IP.

```bash
npx mcporter call tapo.check_device id="<DEVICE_ID>" ip="<IP>"
```

### get_device_state

Get a device's current state. Checks the capability's parameters, then automatically runs `check_device`.

```bash
# Device info
npx mcporter call tapo.get_device_state id="<DEVICE_ID>" ip="<IP>" capability='{"type": "DeviceInfo"}'

# Energy used per hour, in Wh, for 1-8 days from start_date (P110, P110M, P115 plugs; for a P304M or P316M outlet, pass the outlet's id as <DEVICE_ID> and the strip's ip as <IP>)
npx mcporter call tapo.get_device_state id="<DEVICE_ID>" ip="<IP>" capability='{"type": "EnergyDataHourly", "start_date": "2026-01-01", "days": 1}'

# Energy used per day, in Wh, for a quarter (1-4) of a year
npx mcporter call tapo.get_device_state id="<DEVICE_ID>" ip="<IP>" capability='{"type": "EnergyDataDaily", "year": 2026, "quarter": 1}'

# Energy used per month, in Wh, for a year
npx mcporter call tapo.get_device_state id="<DEVICE_ID>" ip="<IP>" capability='{"type": "EnergyDataMonthly", "year": 2026}'

# Power draw every 5 minutes, in W, for 1-12 hours from start_date_time (UTC, rounded up to the next 5 minute mark)
npx mcporter call tapo.get_device_state id="<DEVICE_ID>" ip="<IP>" capability='{"type": "PowerDataEvery5Minutes", "start_date_time": "2026-01-01T06:00:00Z", "hours": 6}'

# Power draw every hour, in W, for 1-6 days from start_date_time (UTC, rounded up to the next hour)
npx mcporter call tapo.get_device_state id="<DEVICE_ID>" ip="<IP>" capability='{"type": "PowerDataHourly", "start_date_time": "2026-01-01T00:00:00Z", "days": 2}'

# Trigger logs (S200, T100, T110, T300 hub children) — newest first
npx mcporter call tapo.get_device_state id="<CHILD_ID>" ip="<HUB_IP>" capability='{"type": "TriggerLogs", "page_size": 20, "start_id": 0}'

# Last 24h temperature/humidity records (T310, T315 hub children)
npx mcporter call tapo.get_device_state id="<CHILD_ID>" ip="<HUB_IP>" capability='{"type": "TemperatureHumidityRecords"}'

# Detections stored on a camera's SD card between start_date_time and end_date_time (UTC, at most 24 hours apart), as `detections`, oldest first, each with its start, end, trigger and the events detected (Person, Motion, BabyCry, ...). Fails with STORAGE_NOT_EXIST when the camera has no SD card
npx mcporter call tapo.get_device_state id="<CAMERA_ID>" ip="<IP>" capability='{"type": "Detections", "start_date_time": "2026-01-01T00:00:00Z", "end_date_time": "2026-01-02T00:00:00Z"}'
```

Energy dates are read in the plug's time zone, so the entries match the Tapo app. Hours, days and months that have not started yet are left out.

### control_device

Control a device by applying 1 to 12 set capabilities, in order. The count and every value are checked before the device is contacted; then it automatically runs `check_device`, and a capability the device does not support fails the call before any capability is applied. `Brightness`, `Color`, `HueSaturation` and `ColorTemperature` also turn the device on if it is off. When the device returns an error partway through, the error says how many capabilities were applied and which one failed.

```bash
# Turn on (plugs, lights, power strip outlets, and S210 switches paired to a hub)
npx mcporter call tapo.control_device id="<DEVICE_ID>" ip="<IP>" capabilities='[{"type": "OnOff", "value": true}]'

# Turn off
npx mcporter call tapo.control_device id="<DEVICE_ID>" ip="<IP>" capabilities='[{"type": "OnOff", "value": false}]'

# Set brightness (1-100, lights only)
npx mcporter call tapo.control_device id="<DEVICE_ID>" ip="<IP>" capabilities='[{"type": "Brightness", "value": 50}]'

# Set color (color lights only)
npx mcporter call tapo.control_device id="<DEVICE_ID>" ip="<IP>" capabilities='[{"type": "Color", "value": "Coral"}]'

# Set a custom color by hue (0-360) and saturation (1-100) (color lights only)
npx mcporter call tapo.control_device id="<DEVICE_ID>" ip="<IP>" capabilities='[{"type": "HueSaturation", "hue": 120, "saturation": 75}]'

# Set color temperature (2500-6500 kelvin) (color lights only)
npx mcporter call tapo.control_device id="<DEVICE_ID>" ip="<IP>" capabilities='[{"type": "ColorTemperature", "value": 3000}]'

# Set multiple capabilities at once
npx mcporter call tapo.control_device id="<DEVICE_ID>" ip="<IP>" capabilities='[{"type": "Color", "value": "Coral"}, {"type": "Brightness", "value": 50}]'
```

### take_snapshot

Capture a still JPEG snapshot from a Tapo camera (~640x360), with the device id, capture time and size as text. When the server has `TAPO_MCP_PUBLIC_URL` set, the text also includes a `url` to the image that is valid for 5 minutes; share it with the user or download the file. Automatically runs `check_device` first. Requires `TAPO_MCP_CAMERA_USERNAME` and `TAPO_MCP_CAMERA_PASSWORD` configured on the server (Camera Settings > Advanced Settings > Camera Account in the Tapo app).

```bash
npx mcporter call tapo.take_snapshot id="<DEVICE_ID>" ip="<IP>"
```

## Usage rules

1. Always run `list_devices` first if you don't have a recent device list. It returns immediately from the server's background discovery, so there is no need to cache it. If a device the user expects is missing, check `errors`, `unsupported`, `discovered_at` and `refresh_error` as described under `list_devices` before concluding it is gone. A device that joined after `discovered_at` shows up within `TAPO_MCP_DISCOVERY_INTERVAL` seconds; do not poll `list_devices` in a loop.
2. Use the device `id` and `ip` from the list — never guess or hardcode these values.
3. For power strips (e.g. P304M) and the H100, H110, H200 and H500 hubs, children have their own `id`. Use the child `id` with the parent's `ip`.
