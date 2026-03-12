/// A bool that assumes `true` when uncertain.
///
/// `OptimisticBool` is a three-valued boolean where the third variant, `Assume`,
/// resolves to `true`. Inside Rust, `Assume` lets you distinguish "explicitly true"
/// from "true by default". Outside Rust, both are just `true`.
///

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptimisticBool {
    /// Explicitly true.
    True,
    /// Explicitly false.
    False,
    /// Unknown but optimistically assumed to be `true`.
    Assume,
}
