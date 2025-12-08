//! `lib-core` contains the core domain logic and data access for the MCP Agent Mail application.

pub mod ctx;
pub mod model;
pub mod store;
pub mod error; // New error module

// Re-export core types
pub use ctx::Ctx;
pub use model::ModelManager;
pub use error::{Error, Result};
