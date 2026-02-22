# TODO
- Add TROUBLESHOOTING.md
  - https://github.com/mihai-dinculescu/tapo/commit/6fd421ec7c426390a2b9d0d060c1b6b847d5d864
  - https://github.com/mihai-dinculescu/tapo/issues/441

- Tapo S200D Smart Remote Dimmer Switch
- Tapo S210 & S220 Smart Light Switch
- P304 energy monitoring
- simplify get trigger logs function
- refactor to_dict python
- refactor to_dict rust
- refactor device support readme section
- get_power_data for plugs
- discover CLI

# Traffic Sources
community.tp-link.com
forum.domoticz.com

# Plug
export IP_ADDRESS=192.168.1.108
export TAPO_USERNAME=mihai.dinculescu@outlook.com
export TAPO_PASSWORD=eDDGURFaPK4zoa

cargo run --example tapo_p110
cargo run --example tapo_p100

# Office Plug
export IP_ADDRESS=192.168.1.102
export TAPO_USERNAME=mihai.dinculescu@outlook.com
export TAPO_PASSWORD=eDDGURFaPK4zoa

cargo run --example tapo_p110

# Office Heater P110M
export IP_ADDRESS=192.168.1.115
export TAPO_USERNAME=mihai.dinculescu@outlook.com
export TAPO_PASSWORD=eDDGURFaPK4zoa

cargo run --example tapo_p110

# Tumble Dryer plug
export IP_ADDRESS=192.168.1.111
export TAPO_USERNAME=mihai.dinculescu@outlook.com
export TAPO_PASSWORD=eDDGURFaPK4zoa

cargo run --example tapo_p110

# Light bulb Office hw v1
export IP_ADDRESS=192.168.1.101
export TAPO_USERNAME=mihai.dinculescu@outlook.com
export TAPO_PASSWORD=eDDGURFaPK4zoa

cargo run --example tapo_l530

# Light bulb Office hw v3
export IP_ADDRESS=192.168.1.118
export TAPO_USERNAME=mihai.dinculescu@outlook.com
export TAPO_PASSWORD=eDDGURFaPK4zoa

cargo run --example tapo_l530

# Light bulb Moon Lamp hw v2
export IP_ADDRESS=192.168.1.114
export TAPO_USERNAME=mihai.dinculescu@outlook.com
export TAPO_PASSWORD=eDDGURFaPK4zoa

cargo run --example tapo_l530

# Light strip
export IP_ADDRESS=192.168.1.112
export TAPO_USERNAME=mihai.dinculescu@outlook.com
export TAPO_PASSWORD=eDDGURFaPK4zoa

cargo run --example tapo_l930

# Hub
export IP_ADDRESS=192.168.1.117
export TAPO_USERNAME=mihai.dinculescu@outlook.com
export TAPO_PASSWORD=eDDGURFaPK4zoa
export NAME=KE100
export TARGET_TEMPERATURE=25

cargo run --example tapo_h100

# Power Strip
export IP_ADDRESS=192.168.1.195
export TAPO_USERNAME=mihai.dinculescu@outlook.com
export TAPO_PASSWORD=eDDGURFaPK4zoa

cargo run --example tapo_p304

# Discovery
export TAPO_USERNAME=mihai.dinculescu@outlook.com
export TAPO_PASSWORD=eDDGURFaPK4zoa

cargo run --example tapo_discover_devices

# Discovery Python
cd tapo-py
poetry shell

export TAPO_USERNAME=mihai.dinculescu@outlook.com
export TAPO_PASSWORD=eDDGURFaPK4zoa
maturin develop && python examples/tapo_discover_devices.py

cargo run -- tapo discover --discovery-target 192.168.1.255
cargo run -- tapo check --ip 192.168.1.118 --name "Office Bulb 2" --model L535B
cargo run -- tapo set --ip 192.168.1.118 --name "Office Bulb 2" --model L535B on
