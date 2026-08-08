class IrRemoteHandler:
    """Handler for the IR remotes paired with a
    [H110](https://www.tapo.com/en/search/?q=H110) hub.

    IR remotes are virtual child devices that are created by the Tapo app, so they
    don't report device info of their own. Their properties, including the list of
    keys that can be sent, are available from `HubHandler.get_child_device_list`
    as `IrRemoteResult`.
    """

    def __init__(self, handler: object):
        """Private constructor.
        It should not be called from outside the tapo library.
        """

    async def send_ir_cmd_by_id(self, key_name: str) -> None:
        """Sends one of the IR keys stored on this remote.

        Args:
            key_name (str): the `name` of a key from this remote's `key_list`
        """
