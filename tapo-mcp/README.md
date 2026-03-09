# tapo-mcp

MCP server that exposes [Tapo](https://www.tapo.com/) smart-home devices as AI-callable tools and resources via the [Model Context Protocol](https://modelcontextprotocol.io/).

Built on the `tapo` crate and the [rmcp](https://crates.io/crates/rmcp) SDK. Runs as an HTTP server (Streamable HTTP transport).

## Table of Contents

- [Capabilities](#capabilities)
- [Tools](#tools)
- [Resources](#resources)
- [Configuration](#configuration)
- [Authentication](#authentication)
- [Deployment](#deployment)
- [Contributing](#contributing)

## Capabilities

Devices and child devices expose a list of capabilities they support. Currently supported:

| Capability | Description               |
| ---------- | ------------------------- |
| `OnOff`    | Turn the device on or off |

## Tools

| Tool               | Description                                                                     |
| ------------------ | ------------------------------------------------------------------------------- |
| `list_devices`     | List available Tapo devices on the network (includes capabilities).             |
| `check_device`     | Verify a device ID matches at a given IP.                                       |
| `set_device_state` | Apply a capability to a device (e.g. `OnOff(true)`). Runs `check_device` first. |

## Resources

| URI              | Description                           |
| ---------------- | ------------------------------------- |
| `tapo://devices` | JSON list of discovered Tapo devices. |

## Configuration

All configuration is via environment variables prefixed with `TAPO_MCP_`:

| Variable                     | Required | Default          | Description                                                |
| ---------------------------- | -------- | ---------------- | ---------------------------------------------------------- |
| `TAPO_MCP_USERNAME`          | Yes      | —                | Tapo account email                                         |
| `TAPO_MCP_PASSWORD`          | Yes      | —                | Tapo account password                                      |
| `TAPO_MCP_DISCOVERY_TARGET`  | Yes      | —                | Network target for device discovery (e.g. `192.168.1.255`) |
| `TAPO_MCP_HTTP_ADDR`         | No       | `127.0.0.1:3000` | Address the server listens on                              |
| `TAPO_MCP_DISCOVERY_TIMEOUT` | No       | `5`              | Discovery timeout in seconds                               |
| `TAPO_MCP_API_KEY`           | No       | —                | Bearer token for HTTP authentication (see below)           |

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

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: tapo-mcp
spec:
  replicas: 1
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
              value: "you@example.com"
            - name: TAPO_MCP_PASSWORD
              value: "your-password"
            - name: TAPO_MCP_DISCOVERY_TARGET
              value: "192.168.1.255"
```

> **Note:** `hostNetwork: true` is required for UDP broadcast discovery, similar to `--network host` in Docker. For production use, consider storing credentials in a [Secret](https://kubernetes.io/docs/concepts/configuration/secret/) instead of plain-text env values.

## Contributing

Contributions are welcome and encouraged! See [/tapo-mcp/CONTRIBUTING.md][contributing].

[contributing]: https://github.com/mihai-dinculescu/tapo/blob/main/tapo-mcp/CONTRIBUTING.md
