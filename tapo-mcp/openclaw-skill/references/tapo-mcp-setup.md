# Setting Up the Tapo MCP Server

How to configure, deploy, and connect to the [Tapo MCP](https://github.com/mihai-dinculescu/tapo/tree/main/tapo-mcp) server.

## Overview

Tapo MCP is an HTTP server (Streamable HTTP transport) that exposes TP-Link Tapo smart-home devices as AI-callable tools and resources via the [Model Context Protocol](https://modelcontextprotocol.io/).

### Tools

| Tool               | Description                                                                                                                                |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `list_devices`     | List available Tapo devices on the network (includes set and get capabilities).                                                            |
| `check_device`     | Verify a device ID matches at a given IP.                                                                                                  |
| `get_device_state` | Get a device's current state (e.g. `{"type": "DeviceInfo"}`). Runs `check_device` first.                                                   |
| `control_device`   | Control a device by applying one or more set capabilities. Runs `check_device` first.                                                      |
| `take_snapshot`    | Capture a still JPEG snapshot from a Tapo camera (~640x360), with the device id, capture time and size as text, plus a short-lived link when `TAPO_MCP_PUBLIC_URL` is set (see Snapshot links). Runs `check_device` first. |

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
| `TAPO_MCP_CAMERA_USERNAME`   | No       | —                | Camera account username[^camera]. Required by `take_snapshot`. |
| `TAPO_MCP_CAMERA_PASSWORD`   | No       | —                | Camera account password[^camera]. Required by `take_snapshot`. |
| `TAPO_MCP_DISCOVERY_TARGET`  | Yes      | —                | Network target for device discovery (e.g. `192.168.1.255`) |
| `TAPO_MCP_HTTP_ADDR`         | No       | `127.0.0.1:3000` | Address the server listens on                              |
| `TAPO_MCP_DISCOVERY_TIMEOUT` | No       | `5`              | Discovery timeout in seconds                               |
| `TAPO_MCP_API_KEY`           | No       | —                | Bearer token for HTTP authentication (see below)           |
| `TAPO_MCP_ALLOWED_HOSTS`     | No       | loopback only    | Comma-separated `Host` header allowlist (see Network exposure) |
| `TAPO_MCP_PUBLIC_URL`        | No       | —                | Base URL clients reach the server at (e.g. `https://tapo.example.com`). Enables short-lived snapshot links (see Snapshot links) |

[^camera]: Set on each camera in the Tapo app under Camera Settings > Advanced Settings > Camera Account. Distinct from your TP-Link cloud account.

## Authentication

When `TAPO_MCP_API_KEY` is set, the server requires all HTTP requests to include an `Authorization: Bearer <key>` header. Requests with a missing or invalid token receive a `401 Unauthorized` response.

When the variable is unset (or empty/whitespace-only), the server runs without authentication.

## Network exposure

The server enforces the MCP Streamable HTTP DNS-rebinding protection. By default only loopback `Host` headers (`localhost`, `127.0.0.1`, `::1`) are accepted, which prevents a malicious web page from reaching a locally running server via DNS rebinding. Requests to the MCP endpoint with any other `Host` receive a `403 Forbidden` response. Snapshot links are not covered by this check (see Snapshot links).

To reach the server over the LAN or from another host, set `TAPO_MCP_ALLOWED_HOSTS` to the exact hostname(s) or `host:port` authorities clients connect to, for example `TAPO_MCP_ALLOWED_HOSTS="tapo-mcp.lan:3000,192.168.1.50:3000"`. This replaces the loopback default, so include loopback entries as well if you still need them.

To avoid shipping unauthenticated smart-home control, the server refuses to start when it binds to a non-loopback address (for example `0.0.0.0:3000`) without `TAPO_MCP_API_KEY` set. Set an API key, or bind to a loopback address.

## Snapshot links

Some MCP clients don't pass image results on to the model, and others show the image to the model but give it nothing it can hand to the user. When `TAPO_MCP_PUBLIC_URL` is set, `take_snapshot` also returns a `url` (and its `expires_at`) pointing at `<TAPO_MCP_PUBLIC_URL>/snapshots/<token>.jpg`, so the agent can share the link or download the file.

- The link is served without the API key, so a browser can open it directly. The `Host` allowlist from `TAPO_MCP_ALLOWED_HOSTS` does not apply to it either. The random token in the path is the only credential: anyone holding the link can view that frame until it expires.
- Each link is valid for 5 minutes. Snapshots are kept in memory only and never written to disk. Expired snapshots are cleared out when the next one is taken, and at most the 1024 most recent are kept.
- Behind an authenticating reverse proxy (Cloudflare Access and similar), links opened in a browser that is already signed in to the proxy work. Non-browser fetches (`curl`, an agent downloading the file) need the proxy's own credential or a policy exception for `/snapshots/`.

When `TAPO_MCP_PUBLIC_URL` is unset, no link is returned and the `/snapshots/` route is not served.

## Deployment

### Docker

```bash
docker run --rm \
  --network host \
  -e TAPO_MCP_USERNAME="you@example.com" \
  -e TAPO_MCP_PASSWORD="<YOUR_TAPO_PASSWORD>" \
  -e TAPO_MCP_CAMERA_USERNAME="<YOUR_CAMERA_ACCOUNT_USERNAME>" \
  -e TAPO_MCP_CAMERA_PASSWORD="<YOUR_CAMERA_ACCOUNT_PASSWORD>" \
  -e TAPO_MCP_DISCOVERY_TARGET="192.168.1.255" \
  -e TAPO_MCP_API_KEY="<YOUR_TAPO_MCP_API_KEY>" \
  ghcr.io/mihai-dinculescu/tapo-mcp:latest
```

> **Note:** The image binds to `0.0.0.0:3000`, so `TAPO_MCP_API_KEY` is required — the server refuses to start on a non-loopback address without it (see Network exposure above). To reach the server by hostname or LAN IP rather than loopback, also set `TAPO_MCP_ALLOWED_HOSTS`. To get snapshot links, add `-e TAPO_MCP_PUBLIC_URL="https://tapo.example.com"`.

> **Note:** `--network host` is required so the container can reach Tapo devices on your local network via UDP broadcast for discovery. On macOS and Windows, `--network host` is not supported — you can use `-p 3000:3000` instead, but device discovery won't work as Docker Desktop runs containers inside a VM without LAN access.

### Kubernetes

Create the Secret and ConfigMap first:

```bash
kubectl create secret generic tapo-mcp-secrets \
  --from-literal=TAPO_MCP_USERNAME="you@example.com" \
  --from-literal=TAPO_MCP_PASSWORD="<YOUR_TAPO_PASSWORD>" \
  --from-literal=TAPO_MCP_CAMERA_USERNAME="<YOUR_CAMERA_ACCOUNT_USERNAME>" \
  --from-literal=TAPO_MCP_CAMERA_PASSWORD="<YOUR_CAMERA_ACCOUNT_PASSWORD>" \
  --from-literal=TAPO_MCP_API_KEY="<YOUR_TAPO_MCP_API_KEY>"

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
            - name: TAPO_MCP_CAMERA_USERNAME
              valueFrom:
                secretKeyRef:
                  name: tapo-mcp-secrets
                  key: TAPO_MCP_CAMERA_USERNAME
            - name: TAPO_MCP_CAMERA_PASSWORD
              valueFrom:
                secretKeyRef:
                  name: tapo-mcp-secrets
                  key: TAPO_MCP_CAMERA_PASSWORD
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

> **Note:** `hostNetwork: true` is required for UDP broadcast discovery, similar to `--network host` in Docker. Because clients reach the server by node IP or hostname rather than loopback, set `TAPO_MCP_ALLOWED_HOSTS` accordingly (see Network exposure above). To get snapshot links, add `TAPO_MCP_PUBLIC_URL` to the ConfigMap and the Deployment's `env`.

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
  --header "Authorization: Bearer <YOUR_TAPO_MCP_API_KEY>" \
  -- tapo http://127.0.0.1:3000
```

Then use `/mcp` in Claude Code to verify the server is connected and its tools appear.

## Source

Based on [tapo-mcp/README.md](https://github.com/mihai-dinculescu/tapo/blob/main/tapo-mcp/README.md).
