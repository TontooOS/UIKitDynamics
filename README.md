# Tontoo UIKit Dynamics

A Physics and Animation Lib for TontooUI/OS Apps

## Made for TontooOS

Explore more at https://github.com/TontooOS/Libs

## Adding to Your Project

Add to your `Cargo.toml`:

```toml
[dependencies]
sdk = { path = "/Library/System/sdk", features = ["UIKitDynamics"] }
```

Then at the crate root:

```rust
sdk::preinclude!();
use UIKitDynamics::{ /* ... */ };
```

## License

TCL v26.1