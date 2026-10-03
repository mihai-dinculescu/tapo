# Troubleshooting

Common issues encountered when using this library, and how to resolve them.

## Before you start

### Use the latest version of the library

Always try the latest release first. It is the version that gets tested against current device firmware, and the one that receives a fix when an issue is found.

### Enable debug logs

Debug logs show which protocol the library negotiated and where the connection failed. Include them when reporting an issue.

Rust:

```bash
RUST_LOG=trace cargo run --example tapo_p110
```

Python (call this before creating the client, as the log level is cached on first use):

```python
import logging
logging.basicConfig(level=5)
```

## Handshake fails with 403 Forbidden (Third-Party Compatibility)

Reported in [#441][issue_441], [#449][issue_449], [#473][issue_473] and [#643][issue_643] after plugs (P100, P110, P115) were upgraded to firmware 1.4.0 or newer.

### Symptoms

- Connecting to the device fails with `Unauthorized: FORBIDDEN: Make sure Third-Party Compatibility is turned on in the Tapo app...`.
- With debug logging enabled, the log shows `Using KLAP protocol` followed by `Handshake1 error: 403 Forbidden`.
- The device worked previously and stopped after a firmware update. Devices still on older firmware keep working.

### Cause

Starting with firmware 1.4.0, the device speaks one of two protocols, depending on the "Third-Party Compatibility" option in the Tapo app: KLAP with the option on, TPAP with it off. The library asks the device which of the two it speaks and supports both, so the option can be left either way. This error means that the device did not announce TPAP and then refused KLAP. The setting is stored per account and pushed to the devices, and that push does not always reach a device that was updated after the option was changed.

### Solutions

Try these in order:

1. **Switch Third-Party Compatibility off and back on again in the Tapo app.** Navigate to Me > Third-Party Services > Third-Party Compatibility. Switch it off, leave the settings screen, wait about a minute, go back in and switch it on. Wait another minute before retrying.
2. **Update the Tapo app.** On older app versions the option sits under "Tapo Lab", and toggling it there has no effect. After updating the app, the option moves to Third-Party Services and toggling it there works.
3. **Factory reset the device.** As a last resort, reset the device to factory settings, remove it from the Tapo app, set it up again, and then toggle Third-Party Compatibility once more.

Note: on firmware 1.4.0 some users reported that the handshake succeeds but later requests intermittently fail with 403 Forbidden. Testing against a P110 reproduced this without any clear pattern, which points to a firmware issue rather than an authentication problem. Retrying the request or re-creating the device handler works around it.

## Handshake fails with a hash mismatch (invalid credentials)

Reported in [#320][issue_320] and [#373][issue_373] across plugs (P100, P110, P115) and bulbs (L530, L535). Often affects only some of the devices on an account while the others work with the same credentials.

### Symptoms

- Connecting to the device fails with `Unauthorized: HASH_MISMATCH: The device response did not match the challenge issued by the library...`.
- The device works fine in the Tapo app.
- Other devices on the same account and network work with the same credentials.

### Cause

During the KLAP handshake the device proves that it holds the same hash of your email and password as the library. A mismatch means the device was set up with different credentials than the ones you are passing in. The two known causes are:

- The email or password differs in case. Both are case-sensitive, and the email must match the case of the account exactly.
- The device received its credentials from another TP-Link device during setup. The TP-Link Simple Setup (TSS) protocol shares credentials from previously configured devices on the network, and the copied credentials do not always match your account. This explains why the first device added to a fresh account works while later ones fail.

### Solutions

Try these in order:

1. **Verify the email and password.** Compare them character by character with the account shown in the Tapo app, including the case of the email.
2. **Set the device up over Bluetooth.** Factory reset the device, remove it from the Tapo app, and add it again using the app's Bluetooth setup flow so that it does not pick up credentials from other devices.
3. **Set the device up with no other TP-Link devices active.** Factory reset the device, remove it from the Tapo app, power off or disconnect every other TP-Link/Tapo device on the network, and add the device again.

## Device does not respond on port 80 (local API offline)

Reported in [#577][issue_577] where a P110 device has been upgraded to firmware 1.4.6. Might apply to other devices, firmware versions and scenarios.

### Symptoms

- Connection attempts fail with errors such as `error sending request for url (http://<device ip>/)`.
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

[issue_320]: https://github.com/mihai-dinculescu/tapo/issues/320
[issue_373]: https://github.com/mihai-dinculescu/tapo/issues/373
[issue_441]: https://github.com/mihai-dinculescu/tapo/issues/441
[issue_449]: https://github.com/mihai-dinculescu/tapo/issues/449
[issue_473]: https://github.com/mihai-dinculescu/tapo/issues/473
[issue_577]: https://github.com/mihai-dinculescu/tapo/issues/577
[issue_643]: https://github.com/mihai-dinculescu/tapo/issues/643
[discover_example]: https://github.com/mihai-dinculescu/tapo/blob/main/tapo/examples/tapo_discover_devices.rs
