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

// -------------------- identity --------------------

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

// -------------------- formatting --------------------

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

// -------------------- defaults --------------------

impl Default for UncertainBool {
    fn default() -> Self {
        Self::None
    }
}

// -------------------- ops (Kleene three-valued logic) --------------------

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

impl core::ops::BitAnd for UncertainBool {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::False, _) | (_, Self::False) => Self::False,
            (Self::True, Self::True) => Self::True,
            _ => Self::None,
        }
    }
}

impl core::ops::BitOr for UncertainBool {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::True, _) | (_, Self::True) => Self::True,
            (Self::False, Self::False) => Self::False,
            _ => Self::None,
        }
    }
}

impl core::ops::BitXor for UncertainBool {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::None, _) | (_, Self::None) => Self::None,
            (Self::True, Self::True) | (Self::False, Self::False) => Self::False,
            _ => Self::True,
        }
    }
}

impl core::ops::BitAndAssign for UncertainBool {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl core::ops::BitOrAssign for UncertainBool {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl core::ops::BitXorAssign for UncertainBool {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

// -------------------- serde --------------------

#[cfg(feature = "serde")]
impl serde::Serialize for UncertainBool {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for UncertainBool {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
