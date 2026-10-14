//! Runs the KLAP login against a simulated P110 built from its KLAP fixture
//! in `tests/fixtures/smart/`.

mod simulator;

use tapo::ApiClient;

use simulator::SimulatedDevice;
use simulator::assertions::assert_unauthorized;

const FIXTURE: &str = "tests/fixtures/smart/P110(UK)_1.0_1.4.8_KLAP.json";

/// A KLAP device proves that it knows the account, username and password
/// both, before the client proves anything, so a client of another account
/// sees the mismatch first.
#[tokio::test]
async fn a_klap_login_checks_the_account() {
    let device = SimulatedDevice::new(FIXTURE).start().await;

    let login = ApiClient::new(device.username(), "not the password")
        .p110(device.address())
        .await;
    assert_unauthorized(login, "HASH_MISMATCH");

    let login = ApiClient::new("someone-else@example.com", device.password())
        .p110(device.address())
        .await;
    assert_unauthorized(login, "HASH_MISMATCH");
}
