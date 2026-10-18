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
>
> "What's the temperature in the kitchen?"
>
> "Show me the last 24 hours of temperature from the living room temperature and humidity sensor"
>
> "List the 5 most recent events on the smart button"
>
> "How much energy did the washing machine plug use each day this month?"
>
> "Show me the TV's power draw over the last 6 hours"

## Tools

| Tool               | Description                                                                                                                                                                                                                                   |
| ------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `list_devices`     | List available Tapo devices on the network (includes set and get capabilities).                                                                                                                                                               |
| `check_device`     | Verify a device ID matches at a given IP.                                                                                                                                                                                                     |
| `get_device_state` | Get a device's current state (e.g. `{"type": "DeviceInfo"}`). Checks the capability's parameters, then runs `check_device`.                                                                                                                   |
| `control_device`   | Control a device by applying up to 12 set capabilities. Checks the count, then runs `check_device`.                                                                                                                                           |
| `take_snapshot`    | Capture a still JPEG snapshot from a Tapo camera (~640x360), with the device id, capture time and size as text, plus a short-lived link when `TAPO_MCP_PUBLIC_URL` is set (see [Snapshot links](#snapshot-links)). Runs `check_device` first. |

## Resources

| URI              | Description                           |
| ---------------- | ------------------------------------- |
| `tapo://devices` | JSON list of discovered Tapo devices. |

## Capabilities

Devices and child devices expose separate lists of set and get capabilities they support. Child devices have no IP of their own; their `ip` is their parent's, which is the one to pass with the child's `id`.

### Set Capabilities

| Capability   | Description                              |
| ------------ | ---------------------------------------- |
| `Brightness` | Set the device brightness (1-100)        |
| `Color`      | Set the device color using a preset name |
| `OnOff`      | Turn the device on or off                |

### Get Capabilities

| Capability                   | Description                                                                                          |
| ---------------------------- | ---------------------------------------------------------------------------------------------------- |
| `DeviceInfo`                 | Read the device's current state                                                                      |
| `EnergyDataHourly`           | Read the energy used per hour, in Wh, for up to 8 days (P110, P110M, P115, P304M, P316M)             |
| `EnergyDataDaily`            | Read the energy used per day, in Wh, for a quarter of a year (P110, P110M, P115, P304M, P316M)       |
| `EnergyDataMonthly`          | Read the energy used per month, in Wh, for a year (P110, P110M, P115, P304M, P316M)                  |
| `PowerDataEvery5Minutes`     | Read the power draw every 5 minutes, in W, for up to 12 hours (P110, P110M, P115, P304M, P316M)      |
| `PowerDataHourly`            | Read the power draw every hour, in W, for up to 6 days (P110, P110M, P115, P304M, P316M)             |
| `Snapshot`                   | Capture a still JPEG snapshot. Served by the dedicated `take_snapshot` tool (binary, not JSON state) |
| `TemperatureHumidityRecords` | Read the last 24 hours of temperature and humidity records (T310, T315) at 15 minute intervals       |
| `TriggerLogs`                | Read paginated trigger logs from a hub child sensor (S200, T100, T110, T300)                         |

## Configuration

All configuration is via environment variables prefixed with `TAPO_MCP_`:

| Variable                      | Required | Default          | Description                                                                                                                     |
| ----------------------------- | -------- | ---------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| `TAPO_MCP_USERNAME`           | Yes      | —                | Tapo account email                                                                                                              |
| `TAPO_MCP_PASSWORD`           | Yes      | —                | Tapo account password                                                                                                           |
| `TAPO_MCP_CAMERA_USERNAME`    | No       | —                | Camera account username[^camera]. Required by `take_snapshot`.                                                                  |
| `TAPO_MCP_CAMERA_PASSWORD`    | No       | —                | Camera account password[^camera]. Required by `take_snapshot`.                                                                  |
| `TAPO_MCP_DISCOVERY_TARGET`   | Yes      | —                | Network target for device discovery (e.g. `192.168.1.255`)                                                                      |
| `TAPO_MCP_HTTP_ADDR`          | No       | `127.0.0.1:3000` | Address the server listens on                                                                                                   |
| `TAPO_MCP_DISCOVERY_TIMEOUT`  | No       | `5`              | Discovery timeout in seconds. Must be between `1` and `60`[^discovery]                                                                       |
| `TAPO_MCP_DISCOVERY_INTERVAL` | No       | `600`            | Seconds between background discoveries that refresh the device list. Must be at least `TAPO_MCP_DISCOVERY_TIMEOUT` and at most `86400` (one day)[^discovery] |
| `TAPO_MCP_API_KEY`            | No       | —                | Bearer token for HTTP authentication (see below)                                                                                |
| `TAPO_MCP_ALLOWED_HOSTS`      | No       | loopback only    | Comma-separated `Host` header allowlist (see Network exposure)                                                                  |
| `TAPO_MCP_PUBLIC_URL`         | No       | —                | Base URL clients reach the server at (e.g. `https://tapo.example.com`). Enables short-lived snapshot links (see Snapshot links) |

[^camera]: Set on each camera in the Tapo app under Camera Settings > Advanced Settings > Camera Account. Distinct from your TP-Link cloud account.

[^discovery]: While a background discovery runs, tool calls that contact a device wait for it to finish, and the discovery waits for calls already in progress. A run takes about `TAPO_MCP_DISCOVERY_TIMEOUT` seconds plus a few more to log in to the devices, and longer if a device is slow to respond. A short interval means these pauses happen often: at `15` with the default timeout, tool calls can be held up for several seconds out of every 15. `list_devices` and the `tapo://devices` resource never wait.

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
- Each link is valid for 5 minutes. Snapshots are kept in memory only and never written to disk. Expired snapshots are cleared out within a minute, and at most the 1024 most recent are kept.
- Behind an authenticating reverse proxy (Cloudflare Access and similar), links opened in a browser that is already signed in to the proxy work. Non-browser fetches (`curl`, an agent downloading the file) need the proxy's own credential or a policy exception for `/snapshots/`.

When `TAPO_MCP_PUBLIC_URL` is unset, no link is returned and the `/snapshots/` route is not served.

## OpenTelemetry

Set `OTEL_EXPORTER_OTLP_ENDPOINT` to the full URL of an OTLP/HTTP traces endpoint, including the path (e.g. `http://alloy.monitoring:4318/v1/traces`), to export traces. `OTEL_RESOURCE_ATTRIBUTES` adds resource attributes. `RUST_LOG` (default `tapo_mcp=info`) also decides which spans are exported.

What gets traced:

- **Tool calls.** Each `tools/call` request gets one server span named `tools/call <tool>`, covering the whole call, following the [OpenTelemetry MCP semantic conventions](https://github.com/open-telemetry/semantic-conventions-genai/blob/main/docs/gen-ai/mcp.md). It carries `mcp.method.name`, `gen_ai.tool.name` (only for tools the server provides), `gen_ai.operation.name`, `jsonrpc.request.id`, `mcp.session.id` and `mcp.protocol.version`, plus `tapo.device.id` and `tapo.device.ip` where the tool takes them. A failed call has error status and `error.type` (the JSON-RPC error code, or `tool_error`). Inside it, `check_device` covers the device verification, and `get_capability`, `set_capability` or `capture_snapshot` covers the device request. `get_capability` and `set_capability` carry `tapo.capability`, and `control_device` has one `set_capability` span for each capability it applies. A failed step has error status and `error.type` set to the kind of failure, such as `DeviceNotFound` or `WrongDeviceType`.
- **Background jobs.** `refresh_devices` for each device discovery, and `prune_snapshots` for each sweep of expired snapshot links.

Credentials, the API key and image data are never recorded. Tool arguments are not recorded either, except `id` and `ip`, and a failed call's error message, which can repeat the value that was rejected.

Incoming W3C trace context is honoured. When the request's `params._meta` has a `traceparent` (and optionally `tracestate`), as MCP clients that trace their calls send, the tool call span becomes its child and joins that trace. A `traceparent` header on the HTTP request that carries the call, for example from a tracing reverse proxy, is then recorded as a span link. Without `_meta` context, the HTTP header is used as the parent instead. Without either, the span starts a new trace.

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

> **Note:** The image binds to `0.0.0.0:3000`, so `TAPO_MCP_API_KEY` is required — the server refuses to start on a non-loopback address without it (see [Network exposure](#network-exposure)). To reach the server by hostname or LAN IP rather than loopback, also set `TAPO_MCP_ALLOWED_HOSTS`. To get [snapshot links](#snapshot-links), add `-e TAPO_MCP_PUBLIC_URL="https://tapo.example.com"`.

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

> **Note:** `hostNetwork: true` is required for UDP broadcast discovery, similar to `--network host` in Docker. Because clients reach the server by node IP or hostname rather than loopback, set `TAPO_MCP_ALLOWED_HOSTS` accordingly (see [Network exposure](#network-exposure)). To get [snapshot links](#snapshot-links), add `TAPO_MCP_PUBLIC_URL` to the ConfigMap and the Deployment's `env`.

## OpenClaw

The [tapo skill](https://clawhub.ai/mihai-dinculescu/tapo) for [OpenClaw](https://clawhub.ai) makes it easy to use a deployed tapo MCP server from OpenClaw agents.

Install it with:

```bash
npx clawhub install tapo
```

## Contributing

Contributions are welcome and encouraged! See [/tapo-mcp/CONTRIBUTING.md][contributing].

[contributing]: https://github.com/mihai-dinculescu/tapo/blob/main/tapo-mcp/CONTRIBUTING.md
