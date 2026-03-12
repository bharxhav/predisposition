/// A bool that assumes `false` when uncertain.
///
/// `PessimisticBool` is a three-valued boolean where the third variant, `Assume`,
/// resolves to `false`. Inside Rust, `Assume` lets you distinguish "explicitly false"
/// from "false by default". Outside Rust, both are just `false`.
///

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PessimisticBool {
    /// Explicitly true.
    True,
    /// Explicitly false.
    False,
    /// Unknown but pessimistically assumed to be `false`.
    Assume,
}
