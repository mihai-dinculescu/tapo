from typing import Protocol

from tapo.responses import CloudConnectionState

class HubCloudExt(Protocol):
    """Extension class for the hub's cloud connection."""

    async def get_connect_cloud_state(self) -> CloudConnectionState:
        """Returns the hub's connection state to the Tapo Cloud.

        Returns:
            CloudConnectionState: The cloud connection state of the hub.
        """
