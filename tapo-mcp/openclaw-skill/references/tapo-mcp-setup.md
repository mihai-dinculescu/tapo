# Setting Up the Tapo MCP Server

How to configure, deploy, and connect to the [Tapo MCP](https://github.com/mihai-dinculescu/tapo/tree/main/tapo-mcp) server.

## Overview

Tapo MCP is an HTTP server (Streamable HTTP transport) that exposes TP-Link Tapo smart-home devices as AI-callable tools and resources via the [Model Context Protocol](https://modelcontextprotocol.io/).

### Tools

| Tool               | Description                                                                      |
| ------------------ | -------------------------------------------------------------------------------- |
| `list_devices`     | List available Tapo devices on the network (includes capabilities).              |
| `check_device`     | Verify a device ID matches at a given IP.                                        |
| `set_device_state` | Apply a capability to a device (e.g. `OnOff(true)`). Runs `check_device` first. |

### Resources

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

## Testing

### MCP Inspector

```bash
npx @modelcontextprotocol/inspector http://127.0.0.1:3000

# With authentication:
npx @modelcontextprotocol/inspector \
  --header "Authorization: Bearer $TAPO_MCP_API_KEY" \
  http://127.0.0.1:3000
```

Opens a browser UI where you can list tools/resources and invoke them manually.

### Claude Code

```bash
claude mcp add --transport http tapo http://127.0.0.1:3000

# With authentication:
claude mcp add --transport http \
  --header "Authorization: Bearer your-api-key" \
  -- tapo http://127.0.0.1:3000
```

Then use `/mcp` in Claude Code to verify the server is connected and its tools appear.

## Source

Based on [tapo-mcp/README.md](https://github.com/mihai-dinculescu/tapo/blob/main/tapo-mcp/README.md).
