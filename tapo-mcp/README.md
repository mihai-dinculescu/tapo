# tapo-mcp

MCP server that exposes [Tapo](https://www.tapo.com/) smart-home devices as AI-callable tools and resources via the [Model Context Protocol](https://modelcontextprotocol.io/).

Built on the `tapo` crate and the [rmcp](https://crates.io/crates/rmcp) SDK. Runs as an HTTP server (Streamable HTTP transport).

## Example Prompts

> "List all my Tapo devices"
>
> "Turn off the office light"
>
> "Turn on smart plug 4 on the power strip"
>
> "Is the office light on?"
>
> "Set the bedroom light to 50% brightness"
>
> "Change the living room light to Coral"
>
> "Take a snapshot from the baby monitor"

## Tools

| Tool               | Description                                                                                    |
| ------------------ | ---------------------------------------------------------------------------------------------- |
| `list_devices`     | List available Tapo devices on the network (includes set and get capabilities).                |
| `check_device`     | Verify a device ID matches at a given IP.                                                      |
| `get_device_state` | Get a device's current state (e.g. `{"type": "DeviceInfo"}`). Runs `check_device` first.       |
| `control_device`   | Control a device by applying one or more set capabilities. Runs `check_device` first.          |
| `take_snapshot`    | Capture a still JPEG snapshot from a Tapo camera (~640x360). Runs `check_device` first.        |

## Resources

| URI              | Description                           |
| ---------------- | ------------------------------------- |
| `tapo://devices` | JSON list of discovered Tapo devices. |

## Capabilities

Devices and child devices expose separate lists of set and get capabilities they support.

### Set Capabilities

| Capability   | Description                                |
| ------------ | ------------------------------------------ |
| `Brightness` | Set the device brightness (1-100)          |
| `Color`      | Set the device color using a preset name   |
| `OnOff`      | Turn the device on or off                  |

### Get Capabilities

| Capability   | Description                                                                                          |
| ------------ | ---------------------------------------------------------------------------------------------------- |
| `DeviceInfo` | Read the device's current state                                                                      |
| `Snapshot`   | Capture a still JPEG snapshot. Served by the dedicated `take_snapshot` tool (binary, not JSON state) |

## Configuration

All configuration is via environment variables prefixed with `TAPO_MCP_`:

| Variable                     | Required | Default          | Description                                                |
| ---------------------------- | -------- | ---------------- | ---------------------------------------------------------- |
| `TAPO_MCP_USERNAME`          | Yes      | —                | Tapo account email                                         |
| `TAPO_MCP_PASSWORD`          | Yes      | —                | Tapo account password                                      |
| `TAPO_MCP_CAMERA_USERNAME`   | No       | —                | Camera account username[^camera]. Required by `take_snapshot`. |
| `TAPO_MCP_CAMERA_PASSWORD`   | No       | —                | Camera account password[^camera]. Required by `take_snapshot`. |
| `TAPO_MCP_DISCOVERY_TARGET`  | Yes      | —                | Network target for device discovery (e.g. `192.168.1.255`) |
| `TAPO_MCP_HTTP_ADDR`         | No       | `127.0.0.1:3000` | Address the server listens on                              |
| `TAPO_MCP_DISCOVERY_TIMEOUT` | No       | `5`              | Discovery timeout in seconds                               |
| `TAPO_MCP_API_KEY`           | No       | —                | Bearer token for HTTP authentication (see below)           |

[^camera]: Set on each camera in the Tapo app under Camera Settings > Advanced Settings > Camera Account. Distinct from your TP-Link cloud account.

## Authentication

When `TAPO_MCP_API_KEY` is set, the server requires all HTTP requests to include an `Authorization: Bearer <key>` header. Requests with a missing or invalid token receive a `401 Unauthorized` response.

When the variable is unset (or empty/whitespace-only), the server runs without authentication.

## Deployment

### Docker

```bash
docker run --rm \
  --network host \
  -e TAPO_MCP_USERNAME="you@example.com" \
  -e TAPO_MCP_PASSWORD="your-password" \
  -e TAPO_MCP_DISCOVERY_TARGET="192.168.1.255" \
  ghcr.io/mihai-dinculescu/tapo-mcp:latest
```

> **Note:** `--network host` is required so the container can reach Tapo devices on your local network via UDP broadcast for discovery. On macOS and Windows, `--network host` is not supported — you can use `-p 3000:3000` instead, but device discovery won't work as Docker Desktop runs containers inside a VM without LAN access.

### Kubernetes

Create the Secret and ConfigMap first:

```bash
kubectl create secret generic tapo-mcp-secrets \
  --from-literal=TAPO_MCP_USERNAME="you@example.com" \
  --from-literal=TAPO_MCP_PASSWORD="your-password" \
  --from-literal=TAPO_MCP_API_KEY="your-api-key"

kubectl create configmap tapo-mcp-config \
  --from-literal=TAPO_MCP_DISCOVERY_TARGET="192.168.1.255"
```

Then apply the Deployment:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: tapo-mcp
spec:
  replicas: 1
  strategy:
    type: Recreate
  selector:
    matchLabels:
      app: tapo-mcp
  template:
    metadata:
      labels:
        app: tapo-mcp
    spec:
      hostNetwork: true
      containers:
        - name: tapo-mcp
          image: ghcr.io/mihai-dinculescu/tapo-mcp:latest
          env:
            - name: TAPO_MCP_USERNAME
              valueFrom:
                secretKeyRef:
                  name: tapo-mcp-secrets
                  key: TAPO_MCP_USERNAME
            - name: TAPO_MCP_PASSWORD
              valueFrom:
                secretKeyRef:
                  name: tapo-mcp-secrets
                  key: TAPO_MCP_PASSWORD
            - name: TAPO_MCP_API_KEY
              valueFrom:
                secretKeyRef:
                  name: tapo-mcp-secrets
                  key: TAPO_MCP_API_KEY
            - name: TAPO_MCP_DISCOVERY_TARGET
              valueFrom:
                configMapKeyRef:
                  name: tapo-mcp-config
                  key: TAPO_MCP_DISCOVERY_TARGET

```

> **Note:** `hostNetwork: true` is required for UDP broadcast discovery, similar to `--network host` in Docker.

## OpenClaw

The [tapo skill](https://clawhub.ai/mihai-dinculescu/tapo) for [OpenClaw](https://clawhub.ai) makes it easy to use a deployed tapo MCP server from OpenClaw agents.

Install it with:

```bash
npx clawhub install tapo
```

## Contributing

Contributions are welcome and encouraged! See [/tapo-mcp/CONTRIBUTING.md][contributing].

[contributing]: https://github.com/mihai-dinculescu/tapo/blob/main/tapo-mcp/CONTRIBUTING.md
