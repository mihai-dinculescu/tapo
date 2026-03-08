# tapo-mcp

MCP server that exposes [Tapo](https://www.tapo.com/) smart-home devices as AI-callable tools and resources via the [Model Context Protocol](https://modelcontextprotocol.io/).

Built on the `tapo` crate and the [rmcp](https://crates.io/crates/rmcp) SDK. Runs as an HTTP server (Streamable HTTP transport).

## Table of Contents

- [Capabilities](#capabilities)
- [Tools](#tools)
- [Resources](#resources)
- [Configuration](#configuration)
- [Authentication](#authentication)
- [Running](#running)
- [Testing](#testing)

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

## Running

```bash
export TAPO_MCP_USERNAME="you@example.com"
export TAPO_MCP_PASSWORD="your-password"
export TAPO_MCP_DISCOVERY_TARGET="192.168.1.255"

cargo run -p tapo-mcp
```

## Testing

### MCP Inspector

The quickest way to interactively explore the server:

```bash
npx @modelcontextprotocol/inspector http://127.0.0.1:3000

# With authentication:
npx @modelcontextprotocol/inspector --header "Authorization: Bearer $TAPO_MCP_API_KEY" http://127.0.0.1:3000
```

Opens a browser UI where you can list tools/resources and invoke them manually. Requires Node.js.

### Claude Code

Add the server to your Claude Code MCP config:

```bash
claude mcp add --transport http tapo http://127.0.0.1:3000

# With authentication:
claude mcp add --transport http --header "Authorization: Bearer your-api-key" tapo http://127.0.0.1:3000
```

Then use `/mcp` in Claude Code to verify the server is connected and its tools appear.
