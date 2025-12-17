//! `lib-core` contains the core domain logic and data access for the MCP Agent Mail application.

//! # lib-core: Core Business Logic for MCP Agent Mail
//!
//! This crate provides the core business logic layer for the MCP Agent Mail system,
//! following the BMC (Backend Model Controller) pattern for consistent data access.
//!
//! ## Architecture
//!
//! - **BMC Layer**: Stateless controllers for all database operations
//! - **Model Structs**: Data transfer objects (DTOs) for all entities
//! - **ModelManager**: Central manager for database connections and Git operations
//! - **Git Integration**: All entities archived to Git for auditability
//!
//! ## Key Modules
//!
//! - [`model`]: All BMC controllers and data models
//! - [`store`]: Low-level database and Git operations
//! - [`ctx`]: Request context for RBAC
//!
//! ## Example
//!
//! ```no_run
//! use lib_core::model::{ModelManager, agent::AgentBmc};
//! use lib_core::ctx::Ctx;
//!
//! async fn example() -> lib_core::Result<()> {
//!     let mm = ModelManager::new().await?;
//!     let ctx = Ctx::root_ctx();
//!     
//!     // List all agents in a project
//!     let agents = AgentBmc::list_all_for_project(&ctx, &mm, 1).await?;
//!     println!("Found {} agents", agents.len());
//!     Ok(())
//! }
//! ```

pub mod ctx;
pub mod error;
pub mod model;
pub mod store;
pub mod utils; // Added utils module // New error module

// Re-export core types
pub use ctx::Ctx;
pub use error::{Error, Result};
pub use model::ModelManager;
