//! The HTTP side of a device, over TLS when the device takes its requests
//! that way. Every device takes the `login` requests on `/`.
//! A device in TPAP mode takes the encrypted requests of a session on
//! `/stok=<token>/ds`; one in KLAP mode takes its handshake on
//! `/app/handshake1` and `/app/handshake2` and the encrypted requests on
//! `/app/request?seq=<seq>`, with the session cookie.

use std::convert::Infallible;
use std::io::ErrorKind;
use std::sync::Arc;

use http_body_util::{BodyExt, Full};
use hyper::body::{Bytes, Incoming};
use hyper::header::{CONTENT_TYPE, COOKIE, SET_COOKIE};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use log::{debug, error};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;

use super::device::Device;

pub async fn serve(listener: TcpListener, tls: Option<TlsAcceptor>, device: Arc<Device>) {
    loop {
        let stream = match listener.accept().await {
            Ok((stream, _)) => stream,
            // A client that closed its connection before it was accepted
            // is no reason to stop accepting the others.
            Err(error)
                if matches!(
                    error.kind(),
                    ErrorKind::ConnectionReset | ErrorKind::ConnectionAborted
                ) =>
            {
                debug!("A connection was lost before it was accepted: {error}");
                continue;
            }
            Err(error) => {
                error!("Stopping, the listener cannot accept: {error}");
                break;
            }
        };
        let device = device.clone();
        let tls = tls.clone();

        tokio::spawn(async move {
            match tls {
                Some(tls) => match tls.accept(stream).await {
                    Ok(stream) => serve_connection(stream, device).await,
                    Err(error) => error!("A TLS handshake failed: {error}"),
                },
                None => serve_connection(stream, device).await,
            }
        });
    }
}

async fn serve_connection<S>(stream: S, device: Arc<Device>)
where
    S: AsyncRead + AsyncWrite + Unpin + 'static,
{
    let service = service_fn(|request| {
        let device = device.clone();
        async move { Ok::<_, Infallible>(handle(&device, request).await) }
    });

    if let Err(error) = http1::Builder::new()
        .serve_connection(TokioIo::new(stream), service)
        .await
    {
        debug!("A connection ended with an error: {error}");
    }
}

async fn handle(device: &Device, request: Request<Incoming>) -> Response<Full<Bytes>> {
    let path = request.uri().path().to_string();
    let seq = request.uri().query().and_then(|query| {
        query
            .strip_prefix("seq=")
            .and_then(|seq| seq.parse::<i32>().ok())
    });
    let cookie = request
        .headers()
        .get(COOKIE)
        .and_then(|cookie| cookie.to_str().ok())
        .and_then(|cookie| cookie.strip_prefix("TP_SESSIONID="))
        .map(str::to_string);
    let body = request
        .into_body()
        .collect()
        .await
        .expect("the request has a body")
        .to_bytes();

    let tpap_session = path
        .strip_prefix("/stok=")
        .and_then(|rest| rest.strip_suffix("/ds"));

    match (device, path.as_str(), tpap_session) {
        // A camera also takes an AES SSL login, which the simulator does
        // not speak.
        (Device::Tpap(tpap), "/", _) => match tpap.login(&body) {
            Ok(response) => json(response),
            Err(reason) => not_implemented(&reason),
        },
        (Device::Tpap(tpap), _, Some(token)) => octets(tpap.request(token, &body)),
        (Device::Klap(klap), "/", _) => json(klap.discover()),
        (Device::Klap(klap), "/app/handshake1", _) => {
            let (cookie, body) = klap.handshake1(&body);
            Response::builder()
                .header(SET_COOKIE, format!("TP_SESSIONID={cookie};TIMEOUT=1440"))
                .body(Full::new(Bytes::from(body)))
                .expect("the response is valid")
        }
        (Device::Klap(klap), "/app/handshake2", _) => {
            let cookie = cookie.expect("`handshake2` carries the session cookie");
            let status = if klap.handshake2(&cookie, &body) {
                StatusCode::OK
            } else {
                StatusCode::FORBIDDEN
            };
            Response::builder()
                .status(status)
                .body(Full::default())
                .expect("the response is valid")
        }
        (Device::Klap(klap), "/app/request", _) => {
            let cookie = cookie.expect("a session request carries the session cookie");
            let seq = seq.expect("a session request names its sequence number");
            octets(klap.request(&cookie, seq, &body))
        }
        _ => not_implemented(&format!("the simulator does not answer {path}")),
    }
}

/// A request that the simulator cannot answer. A panic here would only end
/// the connection, so the reason is logged, which the test prints when it
/// fails, and sent back as the body of the response.
fn not_implemented(reason: &str) -> Response<Full<Bytes>> {
    error!("{reason}");

    Response::builder()
        .status(StatusCode::NOT_IMPLEMENTED)
        .body(Full::new(Bytes::from(reason.to_string())))
        .expect("the response is valid")
}

fn json(body: serde_json::Value) -> Response<Full<Bytes>> {
    Response::builder()
        .header(CONTENT_TYPE, "application/json")
        .body(Full::new(Bytes::from(body.to_string())))
        .expect("the response is valid")
}

fn octets(body: Vec<u8>) -> Response<Full<Bytes>> {
    Response::builder()
        .header(CONTENT_TYPE, "application/octet-stream")
        .body(Full::new(Bytes::from(body)))
        .expect("the response is valid")
}
