"""H110 Example

The H110 is handled by the same `HubHandler` as the H100. On top of the sensors
that the H100 supports, it can also have IR remotes as child devices, which must
be configured in the Tapo app first.

Set the optional `IR_REMOTE` and `IR_KEY` environment variables to send one of
the keys stored on a remote:

```bash
export IR_REMOTE="Living Room TV"
export IR_KEY=POWER
```
"""

import asyncio
import os

from tapo import ApiClient
from tapo.responses import IrRemoteResult

from common import require_env_vars


async def main():
    tapo_username, tapo_password, ip_address = require_env_vars(
        "TAPO_USERNAME", "TAPO_PASSWORD", "IP_ADDRESS"
    )

    client = ApiClient(tapo_username, tapo_password)
    hub = await client.h110(ip_address)

    device_info = await hub.get_device_info()
    print(f"Device info: {device_info.to_dict()}")

    child_device_list = await hub.get_child_device_list()

    for child in child_device_list:
        if isinstance(child, IrRemoteResult):
            keys = ", ".join(f"{key.name} ({key.display_name})" for key in child.key_list)

            print(
                "Found IR remote child device with nickname: {}, id: {}, model: {}, keys: {}.".format(
                    child.nickname, child.device_id, child.model, keys
                )
            )
        else:
            print(
                "Found child device with nickname: {}, id: {}, model: {}.".format(
                    child.nickname, child.device_id, child.model
                )
            )

    remote_nickname = os.environ.get("IR_REMOTE")
    key_name = os.environ.get("IR_KEY")

    if remote_nickname and key_name:
        print(f"Sending the '{key_name}' key on the '{remote_nickname}' remote...")

        remote = await hub.ir_remote(nickname=remote_nickname)
        await remote.send_ir_cmd_by_id(key_name)

        print("The IR command has been sent.")
    else:
        print("Set the IR_REMOTE and IR_KEY environment variables to send an IR command.")


if __name__ == "__main__":
    asyncio.run(main())
