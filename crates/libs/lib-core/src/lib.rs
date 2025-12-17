//! `lib-core` contains the core domain logic and data access for the MCP Agent Mail application.

pub mod ctx;
pub mod error;
pub mod model;
pub mod store;
pub mod utils; // Added utils module // New error module

// Re-export core types
pub use ctx::Ctx;
pub use error::{Error, Result};
pub use model::ModelManager;
