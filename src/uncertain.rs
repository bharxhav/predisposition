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
