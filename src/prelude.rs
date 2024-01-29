//! Convenience re-export of common members
//!
//! Like the standard library's prelude, this module simplifies importing of
//! common items. Unlike the standard prelude, the contents of this module must
//! be imported manually:
//!
//! ```rust
//! use e::prelude::*;
//! ```

#[cfg(feature = "macros")]
pub use crate::macros::*;
