/// A bool that assumes `false` when uncertain.
///
/// `PessimisticBool` behaves like an [`Option<bool>`] where `None` defaults to
/// `false`. The enum's `True` maps to `Some(true)`, `False` maps to `Some(false)`.
///
/// Inside rust, your extrinsic uncertainity is your pessimism.
///

#[derive(Clone, Copy)]
pub enum PessimisticBool {
    /// Explicitly true.
    True,
    /// Explicitly false.
    False,
    /// Unknown but pessimistically assumed to be `false`.
    Assume,
}

impl PartialEq for PessimisticBool {
    fn eq(&self, other: &Self) -> bool {
        // Resolved: Assume == False (both resolve to false)
        match (self, other) {
            (Self::True, Self::True) => true,
            (Self::True, _) | (_, Self::True) => false,
            _ => true, // False/Assume are all equal
        }
    }
}

impl Eq for PessimisticBool {}

impl core::hash::Hash for PessimisticBool {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        // Hash the resolved bool so Hash is consistent with Eq
        match self {
            Self::True => true.hash(state),
            Self::False | Self::Assume => false.hash(state),
        }
    }
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
        Self::Assume
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for PessimisticBool {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PessimisticBool {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = <&str>::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
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
