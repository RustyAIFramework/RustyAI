//! Unified prelude for the rustyai framework.
//!
//! Import everything you need with a single use statement:
//!
//! ```rust
//! use rustyai::prelude::*;
//! ```

// Core primitives
pub use crate::agent_core::prelude::*;

// Messaging
pub use crate::messaging::prelude::*;

// Cognition (BDI)
pub use crate::cognition::prelude::*;

// Organizational patterns
pub use crate::patterns::prelude::*;

// Runtime (just use the prelude to avoid conflicts)
pub use crate::runtime::prelude::*;
