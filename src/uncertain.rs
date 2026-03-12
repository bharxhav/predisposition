/// A bool that makes no assumption — `None` means unknown.
///
/// `UncertainBool` behaves exactly like [`Option<bool>`]. It provides a named
/// enum with the same semantics: `True` maps to `Some(true)`, `False` maps to
/// `Some(false)`, and `None` maps to `Option::None`. No value is assumed for
/// the unknown case.
///

#[derive(Debug)]
pub enum UncertainBool {
    /// Explicitly true (`Some(true)`).
    True,
    /// Explicitly false (`Some(false)`).
    False,
    /// Unknown (`Option::None`) no assumption is made.
    None,
}

impl core::fmt::Display for UncertainBool {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::True => f.write_str("true"),
            Self::False => f.write_str("false"),
            Self::None => f.write_str("none"),
        }
    }
}

impl core::str::FromStr for UncertainBool {
    type Err = crate::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "none" => Ok(Self::None),
            _ => Err(crate::ParseError { _priv: () }),
        }
    }
}

impl Default for UncertainBool {
    fn default() -> Self {
        Self::None
    }
}

impl core::ops::Not for UncertainBool {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Self::True => Self::False,
            Self::False => Self::True,
            Self::None => Self::None,
        }
    }
}
