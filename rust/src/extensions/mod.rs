//! Stable, hand-written ergonomics layered over Fern-generated API types.

mod partner_client;
pub mod webhooks;

pub use partner_client::{Error, ReveroClient, ReveroClientOptions, Scope, WaitOptions};
