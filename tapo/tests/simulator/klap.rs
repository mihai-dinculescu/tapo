//! The device side of KLAP: a handshake of two steps in which each side
//! proves to the other that it knows the account, and the sessions that it
//! opens, whose requests and responses are encrypted and signed. What the
//! device answers a request with comes from the fixture.
//!
//! A request that a client is not supposed to send ends in a panic, which
//! drops the connection and names the reason in the test output.

mod cipher;
mod device;

pub use device::Klap;
