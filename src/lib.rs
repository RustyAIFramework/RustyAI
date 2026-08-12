//! A comprehensive Rust framework for building intelligent, autonomous multi-agent systems.
#![warn(missing_docs)]
#![warn(clippy::all)]

pub use agent_core;
pub use messaging as messaging;
pub use cognition as cognition;
pub use patterns as patterns;
pub use runtime as runtime;

/// Unified prelude for convenient imports
pub mod prelude;

// Re-export commonly used types at crate root for convenience
pub use agent_core::{Agent, AgentId};
pub use messaging::{Message, Performative, Router, Mailbox};
pub use runtime::{Runtime, RuntimeConfig};