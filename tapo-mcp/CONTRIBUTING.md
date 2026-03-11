# Contributing to tapo-mcp

## Running locally

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
claude mcp add --transport http --header "Authorization: Bearer your-api-key" -- tapo http://127.0.0.1:3000
```

Then use `/mcp` in Claude Code to verify the server is connected and its tools appear.

## Releasing new versions

- Update version in `tapo-mcp/Cargo.toml`
- Commit
- Add tag

  ```bash
  git tag -a tapo-mcp-vX.X.X -m "tapo-mcp-vX.X.X"
  ```

- Push

  ```bash
  git push && git push origin tapo-mcp-vX.X.X
  ```

- Create the [release][releases].

[releases]: https://github.com/mihai-dinculescu/tapo/releases
