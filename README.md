# Tapo


[![License][license_badge]][license]
[![Crates][crates_badge]][crates]
[![Documentation][crates_documentation_badge]][crates_documentation]
[![Crates.io][crates_downloads_badge]][crates]
[![PyPI][pypi_badge]][pypi]
[![Python][pypi_versions_badge]][pypi]
[![PyPI][pypi_downloads_badge]][pypi]\
Unofficial Tapo API Client. Works with TP-Link Tapo smart devices. Tested with light bulbs (L510, L520, L530, L535, L610, L630), light strips (L900, L920, L930), plugs (P100, P105, P110, P110M, P115), power strips (P300, P304M, P306, P316M), hubs (H100), switches (S200B) and sensors (KE100, T100, T110, T300, T310, T315).

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

## Supported Devices

See [/SUPPORTED_DEVICES.md][supported_devices] for the supported devices and feature matrix.

## Rust

### Usage

> Cargo.toml
```toml
[dependencies]
tapo = "0.8"
```

> main.rs
```rust
let device = ApiClient::new("<tapo-username>", "tapo-password")
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
poetry install # On the initial run
poetry shell
maturin develop # On the initial run and whenever the Rust code is modified

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

## Credits

Inspired by [petretiandrea/plugp100][inspired_by].

[supported_devices]: https://github.com/mihai-dinculescu/tapo/blob/main/SUPPORTED_DEVICES.md
[examples]: https://github.com/mihai-dinculescu/tapo/tree/main/tapo/examples
[examples-py]: https://github.com/mihai-dinculescu/tapo/tree/main/tapo-py/examples
[tapo_rest]: https://github.com/ClementNerma/tapo-rest
[contributing]: https://github.com/mihai-dinculescu/tapo/blob/main/CONTRIBUTING.md
[inspired_by]: https://github.com/petretiandrea/plugp100
