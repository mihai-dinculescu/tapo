//! The device side of TPAP: the login, which is a SPAKE2+ handshake, and the
//! sessions that it opens, whose requests and responses are encrypted with
//! AES-CCM. What the device answers a request with comes from the fixture.
//!
//! A request that a client is not supposed to send ends in a panic, which
//! drops the connection and names the reason in the test output.

mod cipher;
mod device;
mod spake2p;

pub use device::Tpap;
