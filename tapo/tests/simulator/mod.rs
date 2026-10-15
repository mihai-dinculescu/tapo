//! A simulated Tapo device on the loopback address, for tests that run the
//! library end to end. It answers UDP discovery on the port that real devices
//! use, and HTTP, or HTTPS for a camera, on a port of its own. What it
//! answers, and over which protocol, comes from a [`fixture::Fixture`].

// Each test file compiles its own copy of this module and uses only part of it.
#![allow(dead_code)]

pub mod assertions;
mod crypto;
mod device;
mod discovery;
mod fixture;
mod http;
mod klap;
mod sync;
mod tls;
mod tpap;

pub use device::SimulatedDevice;

use sync::lock;
