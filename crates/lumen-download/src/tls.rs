//! TLS for every HTTP client of the app. Desktop keeps reqwest's default, which
//! asks the operating system to verify certificates. Android has no such
//! verifier unless the app wires one through Java, and without it the first
//! HTTPS request panics, so there the clients trust the Mozilla root
//! certificates bundled in the app instead.

use std::sync::Arc;

use rustls::crypto::aws_lc_rs;
use rustls::{ClientConfig, RootCertStore};

/// The Mozilla root certificates, compiled into the app.
#[cfg_attr(not(target_os = "android"), allow(dead_code))]
fn bundled_root_store() -> RootCertStore {
    RootCertStore {
        roots: webpki_roots::TLS_SERVER_ROOTS.to_vec(),
    }
}

#[cfg_attr(not(target_os = "android"), allow(dead_code))]
fn bundled_tls_config() -> Result<ClientConfig, rustls::Error> {
    Ok(
        ClientConfig::builder_with_provider(Arc::new(aws_lc_rs::default_provider()))
            .with_safe_default_protocol_versions()?
            .with_root_certificates(bundled_root_store())
            .with_no_client_auth(),
    )
}

/// The builder every platform client starts from.
pub fn client_builder() -> reqwest::ClientBuilder {
    let builder = reqwest::Client::builder();
    #[cfg(target_os = "android")]
    if let Ok(config) = bundled_tls_config() {
        return builder.tls_backend_preconfigured(config);
    }
    builder
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_bundled_roots_hold_the_public_certificate_authorities() {
        assert!(bundled_root_store().len() > 100);
    }

    #[test]
    fn the_bundled_tls_config_builds() {
        assert!(bundled_tls_config().is_ok());
    }

    #[test]
    fn the_shared_builder_builds_a_client() {
        assert!(client_builder().build().is_ok());
    }
}
