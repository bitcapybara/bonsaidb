//! A programmable document database inspired by `CouchDB` written in Rust.
//!
//! This crate provides a convenient way to access all functionality of
//! `BonsaiDb`. The crates that are re-exported are:
//!
//! - [`bonsaidb-core`](https://docs.rs/bonsaidb-core): Common types and traits
//!   used when interacting with `BonsaiDb`.
//! - [`bonsaidb-local`](https://docs.rs/bonsaidb-local): Local, file-based
//!   database implementation.
//! - [`bonsaidb-server`](https://docs.rs/bonsaidb-server): Networked `BonsaiDb`
//!   server implementation.
//! - [`bonsaidb-client`](https://docs.rs/bonsaidb-client): Client to access a
//!   `BonsaiDb` server.
//!
//! ## Feature Flags
//!
//! No feature flags are enabled by default in the `bonsaidb` crate. This is
//! because in most Rust executables, you will only need a subset of the
//! functionality. If you'd prefer to enable everything, you can use the `full`
//! feature:
//!
//! ```toml
//! [dependencies]
//! bonsaidb = { version = "*", default-features = false, features = "full" }
//! ```
//!
//! - `full`: Enables `local-full`, `server-full`, and `client-full`.
//! - `cli`: Enables the `bonsaidb` executable.
//!
//! ### Local databases only
//!
//! ```toml
//! [dependencies]
//! bonsaidb = { version = "*", default-features = false, features = "local-full" }
//! ```
//!
//! - `local-full`: Enables `local` and `local-cli`
//! - `local`: Enables the [`local`] module, which re-exports the crate
//!   `bonsaidb-local`.
//! - `local-cli`: Enables the `StructOpt` structures for embedding database
//!   management commands into your own command-line interface.
//!
//! ### `BonsaiDb` server
//!
//! ```toml
//! [dependencies]
//! bonsaidb = { version = "*", default-features = false, features = "server-full" }
//! ```
//!
//! - `server-full`: Enables `server`, `server-acme`, and `server-websockets`
//! - `server`: Enables the [`server`] module, which re-exports the crate
//!   `bonsaidb-server`.
//! - `server-websockets`: Enables `WebSocket` support for `bonsaidb-server`.
//! - `server-acme`: Enables automatic TLS certificate acquisition via `ACME`.
//!
//! ### Client for accessing a `BonsaiDb` server.
//!
//! ```toml
//! [dependencies]
//! bonsaidb = { version = "*", default-features = false, features = "client-full" }
//! ```
//!
//! - `client-full`: Enables `client`, `client-trusted-dns`, and
//!   `client-websockets`
//! - `client`: Enables the [`client`] module, which re-exports the crate
//!   `bonsaidb-client`.
//! - `client-trusted-dns`: Enables using trust-dns for DNS resolution. If not
//!   enabled, all DNS resolution is done with the OS's default name resolver.
//! - `client-websockets`: Enables `WebSocket` support for `bonsaidb-client`.
//!
//! ### `VaultKeyStorage` options
//!
//! - `keystorage-s3`: Enables S3-compatible key storage.

#![forbid(unsafe_code)]
#![warn(
    clippy::cargo,
    missing_docs,
    // clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::pedantic,
    future_incompatible,
    rust_2018_idioms,
)]
#![cfg_attr(doc, deny(rustdoc::all))]
#![allow(
    clippy::missing_errors_doc, // TODO clippy::missing_errors_doc
    clippy::option_if_let_else,
    clippy::multiple_crate_versions, // TODO custodian-password deps + x25119 deps
)]

#[cfg(feature = "client")]
#[doc(inline)]
pub use bonsaidb_client as client;
#[doc(inline)]
pub use bonsaidb_core as core;
#[cfg(feature = "local")]
#[doc(inline)]
pub use bonsaidb_local as local;
#[cfg(feature = "server")]
#[doc(inline)]
pub use bonsaidb_server as server;
#[cfg(feature = "cli")]
pub mod cli;

/// `VaultKeyStorage` implementors.
#[cfg(feature = "keystorage-s3")]
pub mod keystorage {
    #[cfg(feature = "keystorage-s3")]
    #[doc(inline)]
    pub use bonsaidb_keystorage_s3 as s3;
}
