//! Types and traits for TLS support.
pub use priv_io::Stream;

use std::error::Error;
use std::fmt;
use std::io::prelude::*;

#[cfg(feature = "with-native-tls")]
#[deprecated(since = "0.15.2", note = "use the postgres-native-tls crate")]
pub mod native_tls;
#[cfg(feature = "with-openssl")]
#[deprecated(since = "0.15.2", note = "use the postgres-openssl crate")]
pub mod openssl;
#[cfg(feature = "with-schannel")]
pub mod schannel;
#[cfg(feature = "with-security-framework")]
pub mod security_framework;

/// A trait implemented by TLS streams.
pub trait TlsStream: fmt::Debug + Read + Write + Send {
    /// Returns a reference to the underlying `Stream`.
    fn get_ref(&self) -> &Stream;

    /// Returns a mutable reference to the underlying `Stream`.
    fn get_mut(&mut self) -> &mut Stream;
}

/// A trait implemented by types that can initiate a TLS session over a Postgres
/// stream.
pub trait TlsHandshake: fmt::Debug {
    /// Performs a client-side TLS handshake, returning a wrapper around the
    /// provided stream.
    ///
    /// The host portion of the connection parameters is provided for hostname
    /// verification.
    fn tls_handshake(
        &self,
        host: &str,
        stream: Stream,
    ) -> Result<Box<TlsStream>, Box<Error + Sync + Send>>;
}

impl<T: TlsHandshake + ?Sized> TlsHandshake for Box<T> {
    fn tls_handshake(
        &self,
        host: &str,
        stream: Stream,
    ) -> Result<Box<TlsStream>, Box<Error + Sync + Send>> {
        (**self).tls_handshake(host, stream)
    }
}