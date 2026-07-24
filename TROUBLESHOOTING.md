# Troubleshooting

Common issues encountered when using this library, and how to resolve them.

## Device does not respond on port 80 (local API offline)

Reported in [#577][issue_577] where a P110 device has been upgraded to firmware 1.4.6. Might apply to other devices, firmware versions and scenarios.

### Symptoms

- Connection attempts fail with errors such as `error sending request for url (http://<device ip>/app)`.
- The device responds to pings and works fine in the Tapo app, but a port scan shows that port 80 (HTTP) is closed, and navigating to `http://<device ip>/app` in a browser times out instead of returning `200 OK`.
- The device worked previously and stopped after a firmware update or a power loss.

### Cause

Some devices use lazy initialization for their local API service. After booting, the device prioritizes core functions and connects to Wi-Fi and the cloud, but only starts the local web server on port 80 after receiving a TDP (TP-Link Discovery Protocol) probe packet on UDP port 20002.

### Solutions

Try these in order:

1. **Trigger a TDP (TP-Link Discovery Protocol) probe packet to be sent to the device.** This should bring the local API online within a few seconds. Either of these options will do it:
   - Run the [`tapo_discover_devices.rs`][discover_example] example.
   - Refresh the device list in the Tapo app.
2. **Factory reset the device.** As a last resort, reset the device to factory settings, remove it from the Tapo app, and set it up again.

[issue_577]: https://github.com/mihai-dinculescu/tapo/issues/577
[discover_example]: https://github.com/mihai-dinculescu/tapo/blob/main/tapo/examples/tapo_discover_devices.rs
