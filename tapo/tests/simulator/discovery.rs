//! The UDP side of the device, which answers discovery queries with the
//! announcement of the fixture.

use std::io::ErrorKind;

use log::{debug, error, warn};
use serde_json::Value;
use tokio::net::UdpSocket;

/// The header that precedes the JSON of both the query and the response.
const HEADER_LENGTH: usize = 16;

pub async fn serve(socket: UdpSocket, announcement: Value) {
    let payload = serde_json::to_vec(&announcement).expect("the announcement is JSON");
    let mut buffer = [0u8; 2048];

    loop {
        let (size, from) = match socket.recv_from(&mut buffer).await {
            Ok(received) => received,
            // Windows reports an answer that reached a port the client had
            // already closed as a reset on the next receive, which is no
            // reason to stop answering.
            Err(error) if error.kind() == ErrorKind::ConnectionReset => continue,
            Err(error) => {
                error!("Stopping, the socket cannot receive: {error}");
                break;
            }
        };

        // The port is shared with whatever else on the machine speaks to
        // Tapo devices, so a datagram that is not a discovery query is
        // reported and left unanswered, as a real device would leave it.
        let Some((header, query)) = buffer[..size].split_at_checked(HEADER_LENGTH) else {
            warn!("Ignoring a {size}-byte datagram from {from} without a header");
            continue;
        };

        // A real device answers only a query that carries an RSA public key,
        // with which it encrypts a part of its answer that the client does
        // not read.
        let query: Value = match serde_json::from_slice(query) {
            Ok(query) => query,
            Err(error) => {
                warn!("Ignoring a datagram from {from} that is not JSON: {error}");
                continue;
            }
        };
        if !query["params"]["rsa_key"].is_string() {
            warn!("Ignoring a query from {from} without an RSA public key: {query}");
            continue;
        }

        debug!("Answering a query from {from}");

        // The client reads only what follows the header.
        let response = [header, &payload].concat();
        if let Err(error) = socket.send_to(&response, from).await {
            warn!("The answer to {from} was not sent: {error}");
        }
    }
}
