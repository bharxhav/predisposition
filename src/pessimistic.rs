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

// -------------------- identity --------------------

/// Compares resolved values: `Assume` equals `False`.
impl PartialEq for PessimisticBool {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::True, Self::True) => true,
            (Self::True, _) | (_, Self::True) => false,
            _ => true,
        }
    }
}

impl Eq for PessimisticBool {}

/// Orders by resolved value: `False < True`, `Assume` equals `False`.
impl Ord for PessimisticBool {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let lhs = matches!(self, Self::True);
        let rhs = matches!(other, Self::True);
        lhs.cmp(&rhs)
    }
}

impl PartialOrd for PessimisticBool {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Hashes the resolved `bool`, consistent with [`Eq`].
impl core::hash::Hash for PessimisticBool {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::True => true.hash(state),
            Self::False | Self::Assume => false.hash(state),
        }
    }
}

// -------------------- formatting --------------------

/// Prints `True`, `False`, or `AssumeFalse`.
impl core::fmt::Debug for PessimisticBool {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::True => f.write_str("True"),
            Self::False => f.write_str("False"),
            Self::Assume => f.write_str("AssumeFalse"),
        }
    }
}

/// Prints `true`, `false`, or `assume_false`.
impl core::fmt::Display for PessimisticBool {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::True => f.write_str("true"),
            Self::False => f.write_str("false"),
            Self::Assume => f.write_str("assume_false"),
        }
    }
}

/// Parses `"true"`, `"false"`, `"assume"`, or `"assume_false"`.
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

// -------------------- defaults --------------------

/// Returns [`Assume`](PessimisticBool::Assume).
impl Default for PessimisticBool {
    fn default() -> Self {
        Self::Assume
    }
}

// -------------------- ops --------------------

/// Negation flips the predisposition: returns [`OptimisticBool`](crate::OptimisticBool).
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

/// Resolves both operands to `bool`, then ANDs.
impl core::ops::BitAnd for PessimisticBool {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        if bool::from(self) & bool::from(rhs) { Self::True } else { Self::False }
    }
}

/// Resolves both operands to `bool`, then ORs.
impl core::ops::BitOr for PessimisticBool {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        if bool::from(self) | bool::from(rhs) { Self::True } else { Self::False }
    }
}

/// Resolves both operands to `bool`, then XORs.
impl core::ops::BitXor for PessimisticBool {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        if bool::from(self) ^ bool::from(rhs) { Self::True } else { Self::False }
    }
}

impl core::ops::BitAndAssign for PessimisticBool {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl core::ops::BitOrAssign for PessimisticBool {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl core::ops::BitXorAssign for PessimisticBool {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

// -------------------- serde --------------------

#[cfg(feature = "serde")]
impl serde::Serialize for PessimisticBool {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PessimisticBool {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
