/// A bool that assumes `true` when uncertain.
///
/// `OptimisticBool` behaves like an [`Option<bool>`] where `None` defaults to
/// `true`. The enum's `True` maps to `Some(true)`, `False` maps to `Some(false)`.
///
/// Inside rust, your extrinsic uncertainity is your optimism.
///

#[derive(Clone, Copy)]
pub enum OptimisticBool {
    /// Explicitly true.
    True,
    /// Explicitly false.
    False,
    /// Unknown but optimistically assumed to be `true`.
    Assume,
}

impl PartialEq for OptimisticBool {
    fn eq(&self, other: &Self) -> bool {
        // Resolved: Assume == True (both resolve to true)
        match (self, other) {
            (Self::False, Self::False) => true,
            (Self::False, _) | (_, Self::False) => false,
            _ => true, // True/Assume are all equal
        }
    }
}

impl Eq for OptimisticBool {}

impl Ord for OptimisticBool {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let lhs = !matches!(self, Self::False);
        let rhs = !matches!(other, Self::False);
        lhs.cmp(&rhs)
    }
}

impl PartialOrd for OptimisticBool {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl core::hash::Hash for OptimisticBool {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        // Hash the resolved bool so Hash is consistent with Eq
        match self {
            Self::True | Self::Assume => true.hash(state),
            Self::False => false.hash(state),
        }
    }
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
        Self::Assume
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for OptimisticBool {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for OptimisticBool {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
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
