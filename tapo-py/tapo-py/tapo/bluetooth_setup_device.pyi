from tapo.to_dict_ext import ToDictExt

class BluetoothSetupDevice(ToDictExt):
    """A new or factory-reset device that advertises setup over Bluetooth, as
    found by `ApiClient.discover_bluetooth_setup_devices`."""

    @property
    def name(self) -> str:
        """The name that the device advertises, as in `P110M_lgR0GOlb`."""

    @property
    def model(self) -> str:
        """The model, from the name, as in `P110M`."""

    @property
    def address(self) -> str:
        """The Bluetooth address, as in `98:BA:5F:A3:5F:07`. Empty where the
        operating system hides it, as macOS does."""
