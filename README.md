# predisposition

Three-valued boolean types for Rust. Each extends `bool` with a third variant representing an unknown value, and differs in how that unknown is handled when converted to `bool`:

- **`OptimisticBool`** — unknown resolves to `true`
- **`PessimisticBool`** — unknown resolves to `false`
- **`UncertainBool`** — unknown stays unknown (`TryFrom<bool>` fails, Kleene logic preserves `None`)

## Installation

```sh
cargo add predisposition
```

## Usage

```rust
use predisposition::{OptimisticBool, PessimisticBool};

// user hasn't set a dark-mode preference — API returned None
let preference: Option<bool> = None;

// optimist: absent preference means dark mode is on
let dark_mode = OptimisticBool::from(preference);
render_theme(bool::from(dark_mode)); // true

// pessimist: absent preference means dark mode is off
let dark_mode = PessimisticBool::from(preference);
render_theme(bool::from(dark_mode)); // false
```

With `cargo add predisposition --features serde`, uncertainty round-trips without being resolved:

```rust
use predisposition::UncertainBool;

// store the preference as-is — don't pick a side
let dark_mode = UncertainBool::from(None);
let json = serde_json::to_string(&dark_mode).unwrap(); // "none"

// later, deserialize back — still unknown
let restored: UncertainBool = serde_json::from_str(&json).unwrap();
let inner: Option<bool> = restored.into(); // None
```
