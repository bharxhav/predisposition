/// A bool that makes no assumption — `None` means unknown.
///
/// `UncertainBool` behaves exactly like [`Option<bool>`]. It provides a named
/// enum with the same semantics: `True` maps to `Some(true)`, `False` maps to
/// `Some(false)`, and `None` maps to `Option::None`. No value is assumed for
/// the unknown case.
///

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UncertainBool {
    /// Explicitly true (`Some(true)`).
    True,
    /// Explicitly false (`Some(false)`).
    False,
    /// Unknown (`Option::None`) no assumption is made.
    None,
}

impl Ord for UncertainBool {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        // None < False < True (mirrors Option<bool>)
        let lhs = match self {
            Self::None => 0,
            Self::False => 1,
            Self::True => 2,
        };
        let rhs = match other {
            Self::None => 0,
            Self::False => 1,
            Self::True => 2,
        };
        lhs.cmp(&rhs)
    }
}

impl PartialOrd for UncertainBool {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
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

#[cfg(feature = "serde")]
impl serde::Serialize for UncertainBool {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for UncertainBool {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = <&str>::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
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
