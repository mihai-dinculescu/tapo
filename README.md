# Tapo

[![Crates][crates_badge]][crates]
[![Documentation][documentation_badge]][documentation]
[![CI][ci_badge]][ci]
[![license][license_badge]][license]
[![Crates.io][crates_downloads_badge]][crates]\
Unofficial Tapo API Client. Works with TP-Link Tapo smart devices. Tested with light bulbs (L530, L510) and plugs (P110, P100).

## Device support

| Feature               |    L530 |    L510 |    P110 |    P100 | GenericDevice |
| --------------------- | ------: | ------: | ------: | ------: | ------------: |
| on                    | &check; | &check; | &check; | &check; |       &check; |
| off                   | &check; | &check; | &check; | &check; |       &check; |
| get_device_info       | &check; | &check; | &check; | &check; |       &check; |
| get_device_usage      | &check; | &check; | &check; | &check; |       &check; |
| get_energy_usage      |         |         | &check; |         |               |
| get_energy_data       |         |         | &check; |         |               |
| set_brightness        | &check; | &check; |         |         |               |
| set_color             | &check; |         |         |         |               |
| set_hue_saturation    | &check; |         |         |         |               |
| set_color_temperature | &check; |         |         |         |               |
| set() API \*          | &check; | &check; |         |         |               |

\* The `set()` API allows multiple properties to be set in a single request.

## Examples

```bash
export IP_ADDRESS=
export TAPO_USERNAME=
export TAPO_PASSWORD=

cargo run --example tapo_l530
```

See all examples in [/examples][examples].

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

[crates_badge]: https://img.shields.io/crates/v/tapo.svg
[crates]: https://crates.io/crates/tapo
[documentation_badge]: https://docs.rs/tapo/badge.svg
[documentation]: https://docs.rs/tapo
[ci_badge]: https://github.com/mihai-dinculescu/tapo/workflows/CI/badge.svg?branch=main
[ci]: https://github.com/mihai-dinculescu/tapo/actions
[license_badge]: https://img.shields.io/crates/l/tapo.svg
[license]: https://github.com/mihai-dinculescu/tapo/blob/main/LICENSE
[crates_downloads_badge]: https://img.shields.io/crates/d/tapo?label=downloads
[examples]: https://github.com/mihai-dinculescu/tapo/tree/main/examples
[contributing]: https://github.com/mihai-dinculescu/tapo/blob/main/CONTRIBUTING.md
[inspired_by]: https://github.com/petretiandrea/plugp100
