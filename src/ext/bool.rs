// From<bool> for all predisposition types.

use crate::{OptimisticBool, PessimisticBool, UncertainBool};

impl From<bool> for OptimisticBool {
    fn from(value: bool) -> Self {
        if value { Self::True } else { Self::False }
    }
}

impl From<bool> for PessimisticBool {
    fn from(value: bool) -> Self {
        if value { Self::True } else { Self::False }
    }
}

impl From<bool> for UncertainBool {
    fn from(value: bool) -> Self {
        if value { Self::True } else { Self::False }
    }
}

// From predisposition types into bool (resolving).

impl From<OptimisticBool> for bool {
    fn from(value: OptimisticBool) -> Self {
        !matches!(value, OptimisticBool::False)
    }
}

impl From<PessimisticBool> for bool {
    fn from(value: PessimisticBool) -> Self {
        matches!(value, PessimisticBool::True)
    }
}

impl TryFrom<UncertainBool> for bool {
    type Error = crate::ParseError;

    fn try_from(value: UncertainBool) -> Result<Self, Self::Error> {
        match value {
            UncertainBool::True => Ok(true),
            UncertainBool::False => Ok(false),
            UncertainBool::None => Err(crate::ParseError { _priv: () }),
        }
    }
}
