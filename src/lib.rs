//! Three-valued boolean types with different predispositions for the unknown case.
//!
//! This crate provides three enum types that extend `bool` with a third variant
//! representing an unknown or unspecified value. Each type differs in how it
//! resolves that third variant:
//!
//! - [`OptimisticBool`] assumes `true` when uncertain.
//! - [`PessimisticBool`] assumes `false` when uncertain.
//! - [`UncertainBool`]  makes no assumption; behaves exactly like `Option<bool>`.

mod optimistic;
mod pessimistic;
mod uncertain;
mod ext;

pub use optimistic::OptimisticBool;
pub use pessimistic::PessimisticBool;
pub use uncertain::UncertainBool;

use core::fmt;

/// Error returned when parsing or converting a predisposition bool fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    _priv: (),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid predisposition value")
    }
}

impl std::error::Error for ParseError {}
