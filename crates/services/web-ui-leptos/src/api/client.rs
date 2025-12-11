//! HTTP client for MCP Agent Mail API.
//! Will be implemented in task mcp-agent-mail-rs-l0o.
//!
//! This module will provide:
//! - API client wrapper with base URL configuration
//! - Typed request/response handling
//! - Shared types from lib-core (when WASM feature is added)

use serde::{Deserialize, Serialize};

/// API base URL - defaults to localhost for development.
pub const API_BASE_URL: &str = "http://127.0.0.1:8765";

/// Health check response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: Option<String>,
}

/// Project response (placeholder - will use lib-core types later).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub created_at: String,
}

/// Agent response (placeholder - will use lib-core types later).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub project_id: String,
    pub created_at: String,
}

/// Message response (placeholder - will use lib-core types later).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub thread_id: String,
    pub sender: String,
    pub recipient: String,
    pub subject: Option<String>,
    pub body: String,
    pub created_at: String,
}
