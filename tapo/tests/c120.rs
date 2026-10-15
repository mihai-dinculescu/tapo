//! Runs the library end to end against a simulated C120 built from its
//! fixtures in `tests/fixtures/smartcam/`. The login over AES SSL is not
//! simulated.

mod simulator;

use std::error::Error;

use rstest::rstest;
use tapo::{ApiClient, DeviceType, StreamExt};

use simulator::SimulatedDevice;
use simulator::assertions::assert_unauthorized;

/// A C120 on firmware 1.9.4 asks for the MD5 of the password, as a C220 does.
const MD5_FIXTURE: &str = "tests/fixtures/smartcam/C120_1.0_1.9.4_TPAP.json";

/// The state that one C120 was in for a day after it was updated from 1.4.4,
/// on which it had already failed every login: `pake_register` named
/// `password_shadow` id 5 with a salt that it picked anew for each login, and
/// the login took the SHA-256-crypt of the raw password. Setting up a camera
/// account in the app left it asking for MD5, and a factory reset followed by
/// a fresh setup did too, so that state cannot be captured again. The fixture
/// keeps it covered for cameras that are still in it. Its `extra_crypt`,
/// discovery response and device info are what the camera sent in that state;
/// its PBKDF2 `iterations` were not recorded and the error code of a wrong
/// password was never reached in that state, so both are taken from the MD5
/// one.
const SHA256_CRYPT_FIXTURE: &str = "tests/fixtures/smartcam/C120_1.0_1.9.4_TPAP_SHA-256-crypt.json";

#[rstest]
#[case::md5(MD5_FIXTURE)]
#[case::sha256_crypt(SHA256_CRYPT_FIXTURE)]
#[tokio::test]
async fn discovery_finds_the_device_and_logs_in(
    #[case] fixture: &str,
) -> Result<(), Box<dyn Error>> {
    let device = SimulatedDevice::new(fixture).start_with_discovery().await;

    let mut discovery = ApiClient::new(device.username(), device.password())
        .discover_devices(device.ip(), 5)
        .await?;

    let result = discovery
        .next()
        .await
        .ok_or("the device did not answer discovery")??;

    assert_eq!(result.device_type(), DeviceType::Camera, "{result:?}");

    // The stream ends once the device at the target address has answered.
    assert!(discovery.next().await.is_none());

    Ok(())
}

#[rstest]
#[case::md5(MD5_FIXTURE)]
#[case::sha256_crypt(SHA256_CRYPT_FIXTURE)]
#[tokio::test]
async fn a_c120_takes_a_login_by_ip_address(#[case] fixture: &str) -> Result<(), Box<dyn Error>> {
    let device = SimulatedDevice::new(fixture).start().await;

    let handler = ApiClient::new(device.username(), device.password())
        .c120(device.address())
        .await?;

    handler.get_device_info().await?;

    Ok(())
}

/// The device is discovered first: it announces TPAP, so the login does not
/// fall back to AES SSL, which is not simulated.
#[rstest]
#[case::md5(MD5_FIXTURE)]
#[case::sha256_crypt(SHA256_CRYPT_FIXTURE)]
#[tokio::test]
async fn a_c120_login_checks_the_password_only(
    #[case] fixture: &str,
) -> Result<(), Box<dyn Error>> {
    let device = SimulatedDevice::new(fixture).start_with_discovery().await;

    let login = ApiClient::new(device.username(), "not the password")
        .discover_devices(device.ip(), 5)
        .await?
        .next()
        .await
        .ok_or("the device did not answer discovery")?
        .map_err(|error| error.source);
    assert_unauthorized(login, "TPAP_CREDENTIALS");

    ApiClient::new("someone-else@example.com", device.password())
        .discover_devices(device.ip(), 5)
        .await?
        .next()
        .await
        .ok_or("the device did not answer discovery")??;

    Ok(())
}
