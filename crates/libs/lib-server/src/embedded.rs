//! Embedded static assets for single-binary web UI distribution.
//!
//! This module is only compiled when the `with-web-ui` feature is enabled.
//! It embeds the Leptos WASM frontend from web-ui-leptos/dist at compile time.

use rust_embed::Embed;

#[derive(Embed)]
#[folder = "../../services/web-ui-leptos/dist"]
pub struct Assets;
