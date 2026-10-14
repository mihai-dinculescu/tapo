//! Runs the TPAP login against a simulated P110 built from its TPAP fixture
//! in `tests/fixtures/smart/`.

mod simulator;

use std::error::Error;

use tapo::ApiClient;

use simulator::SimulatedDevice;
use simulator::assertions::assert_unauthorized;

const FIXTURE: &str = "tests/fixtures/smart/P110(UK)_1.0_1.4.8_TPAP.json";

/// A TPAP login is made as the `admin` user of the device, so only the
/// password counts.
#[tokio::test]
async fn a_tpap_login_checks_the_password_only() -> Result<(), Box<dyn Error>> {
    let device = SimulatedDevice::new(FIXTURE).start().await;

    let login = ApiClient::new(device.username(), "not the password")
        .p110(device.address())
        .await;
    assert_unauthorized(login, "TPAP_CREDENTIALS");

    ApiClient::new("someone-else@example.com", device.password())
        .p110(device.address())
        .await?;

    Ok(())
}
