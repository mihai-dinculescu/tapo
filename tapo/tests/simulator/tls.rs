//! The TLS side of a device that takes its requests over TLS, as a camera
//! does. The certificate, in `test-only.cert.pem` with its key in
//! `test-only.key.pem`, is self-signed for `localhost`, `127.0.0.1` and
//! `::1`, and every device shares it, though each binds an address of its
//! own: the client accepts any certificate, as it must with a real device.

use std::sync::{Arc, LazyLock};

use tokio_rustls::TlsAcceptor;
use tokio_rustls::rustls::ServerConfig;
use tokio_rustls::rustls::pki_types::pem::PemObject as _;
use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};

static ACCEPTOR: LazyLock<TlsAcceptor> = LazyLock::new(|| {
    let certificate = CertificateDer::from_pem_slice(include_bytes!("test-only.cert.pem"))
        .expect("the certificate is PEM");
    let key =
        PrivateKeyDer::from_pem_slice(include_bytes!("test-only.key.pem")).expect("the key is PEM");

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![certificate], key)
        .expect("the certificate and the key match");

    TlsAcceptor::from(Arc::new(config))
});

/// The acceptor that every device takes its TLS connections with.
pub fn acceptor() -> TlsAcceptor {
    ACCEPTOR.clone()
}
