//! This crate contains an implementation of the multipart/form-data media
//! type described in [RFC 7578](https://tools.ietf.org/html/rfc7578).

mod boundary;
mod client_;
mod error;

pub mod client {
    pub use crate::error::Error;

    /// This module contains data structures for building a multipart/form
    /// body to send a server.
    pub mod multipart {
        pub use crate::{
            boundary::BoundaryGenerator,
            client_::{Body, Form},
        };
    }
}
