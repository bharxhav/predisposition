// From<Option<bool>> for all predisposition types.

use crate::{OptimisticBool, PessimisticBool, UncertainBool};

impl From<Option<bool>> for OptimisticBool {
    fn from(value: Option<bool>) -> Self {
        match value {
            Some(true) => Self::True,
            Some(false) => Self::False,
            None => Self::Assume,
        }
    }
}

impl From<Option<bool>> for PessimisticBool {
    fn from(value: Option<bool>) -> Self {
        match value {
            Some(true) => Self::True,
            Some(false) => Self::False,
            None => Self::Assume,
        }
    }
}

impl From<Option<bool>> for UncertainBool {
    fn from(value: Option<bool>) -> Self {
        match value {
            Some(true) => Self::True,
            Some(false) => Self::False,
            None => Self::None,
        }
    }
}

// From predisposition types into Option<bool> (lossless).

impl From<OptimisticBool> for Option<bool> {
    fn from(value: OptimisticBool) -> Self {
        match value {
            OptimisticBool::True => Some(true),
            OptimisticBool::False => Some(false),
            OptimisticBool::Assume => None,
        }
    }
}

impl From<PessimisticBool> for Option<bool> {
    fn from(value: PessimisticBool) -> Self {
        match value {
            PessimisticBool::True => Some(true),
            PessimisticBool::False => Some(false),
            PessimisticBool::Assume => None,
        }
    }
}

impl From<UncertainBool> for Option<bool> {
    fn from(value: UncertainBool) -> Self {
        match value {
            UncertainBool::True => Some(true),
            UncertainBool::False => Some(false),
            UncertainBool::None => None,
        }
    }
}
