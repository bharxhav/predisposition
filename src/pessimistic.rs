/// A bool that assumes `false` when uncertain.
///
/// `PessimisticBool` is a three-valued boolean where the third variant, `Assume`,
/// resolves to `false`. Inside Rust, `Assume` lets you distinguish "explicitly false"
/// from "false by default". Outside Rust, both are just `false`.
///

pub enum PessimisticBool {
    /// Explicitly true.
    True,
    /// Explicitly false.
    False,
    /// Unknown but pessimistically assumed to be `false`.
    Assume,
}

impl Default for PessimisticBool {
    fn default() -> Self {
        Self::False
    }
}

impl core::ops::Not for PessimisticBool {
    type Output = crate::OptimisticBool;

    fn not(self) -> crate::OptimisticBool {
        match self {
            Self::True => crate::OptimisticBool::False,
            Self::False => crate::OptimisticBool::True,
            Self::Assume => crate::OptimisticBool::Assume,
        }
    }
}
