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

impl core::fmt::Debug for OptimisticBool {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::True => f.write_str("True"),
            Self::False => f.write_str("False"),
            Self::Assume => f.write_str("AssumeTrue"),
        }
    }
}

impl core::fmt::Display for OptimisticBool {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::True => f.write_str("true"),
            Self::False => f.write_str("false"),
            Self::Assume => f.write_str("assume_true"),
        }
    }
}

impl core::str::FromStr for OptimisticBool {
    type Err = crate::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "assume" | "assume_true" => Ok(Self::Assume),
            _ => Err(crate::ParseError { _priv: () }),
        }
    }
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
