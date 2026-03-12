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

impl core::fmt::Debug for PessimisticBool {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::True => f.write_str("True"),
            Self::False => f.write_str("False"),
            Self::Assume => f.write_str("AssumeFalse"),
        }
    }
}

impl core::fmt::Display for PessimisticBool {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::True => f.write_str("true"),
            Self::False => f.write_str("false"),
            Self::Assume => f.write_str("assume_false"),
        }
    }
}

impl core::str::FromStr for PessimisticBool {
    type Err = crate::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "assume" | "assume_false" => Ok(Self::Assume),
            _ => Err(crate::ParseError { _priv: () }),
        }
    }
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
