# predisposition

A Rust type that wraps a boolean with a third state. Inside Rust, a value is `True`, `False`, or a **`third state`**. When this state is resolved: crossing an API boundary, serializing, or evaluating in a condition (essentially **exiting** rust) it renders as `true`, `false`, or `none` depending on the type's predisposition.

- **`OptimisticBool`**: tristate resolves to `true`
- **`PessimisticBool`**: tristate resolves to `false`
- **`UncertainBool`**: tristate resolves to `none`
