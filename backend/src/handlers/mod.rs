// Handlers for REST API endpoints
// Implements the complete evaluation flow from seq_diagram.md

pub mod evaluations;
pub mod responses;
pub mod ai_generation;
pub mod public;
pub mod analytics;
pub mod auth;

pub use evaluations::*;
pub use responses::*;
pub use ai_generation::*;
pub use public::*;
pub use analytics::*;
pub use auth::*;
