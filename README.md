# Tapo


[![License][license_badge]][license]
[![Crates][crates_badge]][crates]
[![Documentation][crates_documentation_badge]][crates_documentation]
[![Crates.io][crates_downloads_badge]][crates]
[![PyPI][pypi_badge]][pypi]
[![Python][pypi_versions_badge]][pypi]
[![PyPI][pypi_downloads_badge]][pypi]\
Unofficial Tapo API Client. Works with TP-Link Tapo smart devices. Tested with light bulbs (L510, L520, L530, L610, L630), light strips (L900, L920, L930), plugs (P100, P105, P110, P115), hubs (H100), switches (S200B) and sensors (KE100, T100, T110, T300, T310, T315).

[license_badge]: https://img.shields.io/crates/l/tapo.svg
[license]: https://github.com/mihai-dinculescu/tapo/blob/main/LICENSE
[crates_badge]: https://img.shields.io/crates/v/tapo.svg?logo=rust&color=F75101
[crates]: https://crates.io/crates/tapo
[crates_documentation_badge]: https://img.shields.io/docsrs/tapo.svg?logo=rust&color=F75101
[crates_documentation]: https://docs.rs/tapo
[crates_downloads_badge]: https://img.shields.io/crates/d/tapo?logo=rust&label=downloads&color=F75101

[pypi_badge]: https://img.shields.io/pypi/v/tapo.svg?logo=pypi&color=00ADD4
[pypi]: https://pypi.org/project/tapo
[pypi_versions_badge]: https://img.shields.io/pypi/pyversions/tapo.svg?logo=python&color=00ADD4
[pypi_downloads_badge]: https://img.shields.io/pypi/dm/tapo?logo=python&color=00ADD4

## Device support

&check; - Rust only\
&#x2705; - Rust and Python

| Feature               | GenericDevice | L510, L520, L610 | L530, L630, L900 | L920, L930 | P100, P105 | P110, P115 |
| --------------------- | ------------: | ---------------: | ---------------: | ---------: | ---------: | ---------: |
| device_reset          |               |         &#x2705; |         &#x2705; |    &check; |   &#x2705; |   &#x2705; |
| get_current_power     |               |                  |                  |            |            |   &#x2705; |
| get_device_info       |      &#x2705; |         &#x2705; |         &#x2705; |    &check; |   &#x2705; |   &#x2705; |
| get_device_info_json  |      &#x2705; |         &#x2705; |         &#x2705; |    &check; |   &#x2705; |   &#x2705; |
| get_device_usage      |               |         &#x2705; |         &#x2705; |    &check; |   &#x2705; |   &#x2705; |
| get_energy_data       |               |                  |                  |            |            |   &#x2705; |
| get_energy_usage      |               |                  |                  |            |            |   &#x2705; |
| off                   |      &#x2705; |         &#x2705; |         &#x2705; |    &check; |   &#x2705; |   &#x2705; |
| on                    |      &#x2705; |         &#x2705; |         &#x2705; |    &check; |   &#x2705; |   &#x2705; |
| set_brightness        |               |         &#x2705; |         &#x2705; |    &check; |            |            |
| set_color             |               |                  |         &#x2705; |    &check; |            |            |
| set_color_temperature |               |                  |         &#x2705; |    &check; |            |            |
| set_hue_saturation    |               |                  |         &#x2705; |    &check; |            |            |
| set_lighting_effect   |               |                  |                  |    &check; |            |            |
| set() API \*          |               |                  |         &#x2705; |    &check; |            |            |

\* The `set()` API allows multiple properties to be set in a single request.

## Hub (H100) Support

| Feature                          |   KE100 |   S200B |    T100 |    T110 |    T300 | T310, T315 |
| -------------------------------- | ------: | ------: | ------: | ------: | ------: | ---------: |
| get_device_info \*               | &check; | &check; | &check; | &check; | &check; |    &check; |
| get_device_info_json             | &check; | &check; | &check; | &check; | &check; |    &check; |
| get_temperature_humidity_records |         |         |         |         |         |    &check; |
| get_trigger_logs                 |         | &check; | &check; | &check; | &check; |            |
| set_child_protection             | &check; |         |         |         |         |            |
| set_frost_protection             | &check; |         |         |         |         |            |
| set_max_control_temperature      | &check; |         |         |         |         |            |
| set_min_control_temperature      | &check; |         |         |         |         |            |
| set_target_temperature           | &check; |         |         |         |         |            |
| set_temperature_offset           | &check; |         |         |         |         |            |

\* Obtained by calling `get_child_device_list` on the hub device or `get_device_info` on a child handler.


## Rust

### Usage

> Cargo.toml
```toml
[dependencies]
tapo = "0.7"
```

> main.rs
```rust
let device = ApiClient::new("<tapo-username>", "tapo-password")?
    .p110("<device ip address>")
    .await?;

device.on().await?;
```

### Examples

```bash
export TAPO_USERNAME=
export TAPO_PASSWORD=
export IP_ADDRESS=

cargo run --example tapo_l530
```

See all examples in [/tapo/examples][examples].

### Wrapper REST API
[tapo-rest][tapo_rest] is a REST wrapper of this library that can be deployed as a service or serve as an advanced example.

## Python

### Usage

```bash
pip install tapo
```

```python
client = ApiClient("<tapo-username>", "tapo-password")
device = await client.p110("<device ip address>")

await device.on()
```

### Examples

```bash
cd tapo-py
poetry install
poetry shell

export TAPO_USERNAME=
export TAPO_PASSWORD=
export IP_ADDRESS=
```

```bash
python examples/tapo_p110.py
```

See all examples in [/tapo-py/examples][examples-py].

## Contributing

Contributions are welcome and encouraged! See [/CONTRIBUTING.md][contributing].

## Troubleshooting

### 1. Installing openssl on Windows

With chocolatey

```powershell
choco install openssl
[System.Environment]::SetEnvironmentVariable('OPENSSL_DIR', $Env:Programfiles + "\OpenSSL-Win64", "User")
```

or with vcpkg

```powershell
git clone git@github.com:microsoft/vcpkg.git
cd vcpkg
./bootstrap-vcpkg.bat
./vcpkg.exe install openssl-windows:x64-windows
./vcpkg.exe install openssl:x64-windows-static
./vcpkg.exe integrate install
[System.Environment]::SetEnvironmentVariable('OPENSSL_DIR', (Get-Location).Path + "\installed\x64-windows-static", "User")
```

## Credits

Inspired by [petretiandrea/plugp100][inspired_by].

[examples]: https://github.com/mihai-dinculescu/tapo/tree/main/tapo/examples
[examples-py]: https://github.com/mihai-dinculescu/tapo/tree/main/tapo-py/examples
[tapo_rest]: https://github.com/ClementNerma/tapo-rest
[contributing]: https://github.com/mihai-dinculescu/tapo/blob/main/CONTRIBUTING.md
[inspired_by]: https://github.com/petretiandrea/plugp100
