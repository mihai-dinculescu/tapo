# Tapo P110 & P115 Control

Control Tapo smart devices (P110 & P115) by turning the device on/off, getting device information, resetting the device, and retrieving energy usage data.

## Requirements

- Python 3.11+
- `dotenv` package for loading environment variables
- `tapo` package for interacting with Tapo devices

## Installation

Setup a virtual Python environment if desired.

Install the required packages:

    ```bash
    pip install python-dotenv tapo
    ```

Create a `.env` file in the root directory with your Tapo username and password:

    ```plaintext
    TAPO_USERNAME=your_tapo_username
    TAPO_PASSWORD=your_tapo_password
    ```


## Usage

Use the following command:

```bash
python tapo_control.py <ip_address> <action>
```

Replace `<ip_address>` with the IP address of your Tapo device and `<action>` with one of the available commands listed next.

### Available Actions

- `on` - Turn the device on
- `off` - Turn the device off
- `device_reset` - Reset the device
- `get_device_info` - Get detailed device information
- `get_device_info_json` - Get detailed device information in JSON format
- `get_device_usage` - Get the device usage statistics
- `get_current_power` - Get the current power consumption
- `get_energy_usage` - Get the energy usage statistics
- `get_energy_data` - Get energy data at different intervals (hourly, daily, monthly)
- `refresh_session` - Refresh the session with the device

### Example

```bash
python tapo_control.py 192.168.0.123 on
```

This command will turn on the Tapo device with the IP address `192.168.0.123`.

### Available Commands

Available actions are provided with the `--help` argument.

```bash
python tapo_control.py --help
```
