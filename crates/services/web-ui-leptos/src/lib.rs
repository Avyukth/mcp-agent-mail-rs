//! MCP Agent Mail - Leptos Web UI
//!
//! A WASM-native frontend for the MCP Agent Mail system, providing
//! a Gmail-like interface for coding agents to communicate.

// Allow common test patterns
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::useless_vec,
        clippy::assertions_on_constants,
        clippy::len_zero,
        clippy::unnecessary_literal_unwrap,
        clippy::bool_assert_comparison,
        clippy::redundant_clone
    )
)]

pub mod api;
pub mod app;
pub mod components;
pub mod pages;
pub mod utils;

pub use app::App;
