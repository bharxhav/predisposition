/// A bool that assumes `true` when uncertain.
///
/// `OptimisticBool` is a three-valued boolean where the third variant, `Assume`,
/// resolves to `true`. Inside Rust, `Assume` lets you distinguish "explicitly true"
/// from "true by default". Outside Rust, both are just `true`.
///

pub enum OptimisticBool {
    /// Explicitly true.
    True,
    /// Explicitly false.
    False,
    /// Unknown but optimistically assumed to be `true`.
    Assume,
}

impl Default for OptimisticBool {
    fn default() -> Self {
        Self::True
    }
}

impl core::ops::Not for OptimisticBool {
    type Output = crate::PessimisticBool;

    fn not(self) -> crate::PessimisticBool {
        match self {
            Self::True => crate::PessimisticBool::False,
            Self::False => crate::PessimisticBool::True,
            Self::Assume => crate::PessimisticBool::Assume,
        }
    }
}
