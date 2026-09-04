//! # Revero Public API SDK
//!
//! The official Rust SDK for the Revero Public API.
//!
//! ## Getting Started
//!
//! ```rust
//! use revero::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = ClientConfig {
//!         username: Some("<username>".to_string()),
//!         password: Some("<password>".to_string()),
//!         ..Default::default()
//!     };
//!     let client = GeneratedReveroClient::new(config).expect("Failed to build client");
//!     client
//!         .oauth
//!         .get_oauth_authorization_server_metadata(None)
//!         .await;
//! }
//! ```
//!
//! ## Modules
//!
//! - [`api`] - Core API types and models
//! - [`client`] - Client implementations
//! - [`config`] - Configuration options
//! - [`core`] - Core utilities and infrastructure
//! - [`error`] - Error types and handling
//! - [`prelude`] - Common imports for convenience

pub mod api;
pub mod client;
pub mod config;
pub mod core;
pub mod environment;
pub mod error;
pub mod extensions;
pub mod prelude;

pub use api::*;
pub use client::*;
pub use config::*;
pub use core::*;
pub use environment::*;
pub use error::{ApiError, BuildError};
pub use extensions::webhooks;
pub use extensions::{Error, ReveroClient, ReveroClientOptions, Scope, WaitOptions};
