# UIKitDynamics – Wiki

A UIKit-Dynamics-style physics and animation engine for TontooOS. Pure physics
without GTK; the `uikit` crate binds it to real widgets.

- Repository: https://github.com/TontooOS/UIKitDynamics
- License: TCL v26.1
- Version: 0.3.0

## Feature Index

| Feature | File | Description |
|---|---|---|
| Main index | [MAIN.md](MAIN.md) | This page |
| Rules | [RULE.md](RULE.md) | Development and usage rules |
| Math | [Math.md](Math.md) | `Vec2`, `Size`, `Rect` primitives |
| Easing | [Easing.md](Easing.md) | Named easing curves + `Tween` |
| Spring | [Spring.md](Spring.md) | Damped harmonic oscillator + presets |
| Behaviors | [Behaviors.md](Behaviors.md) | Gravity, collision, attachment, push, snap |
| Animator | [Animator.md](Animator.md) | `Animator` + `DynamicItem` physics simulation |

## Quick Start

```rust
use uikitdynamics::prelude::*;

let mut animator = Animator::new();
let item = animator.add_item(DynamicItem::new(v(0.0, 0.0), Size::new(60.0, 60.0)));
animator.item_mut(item).unwrap().wake();

animator.add_behavior(GravityBehavior::new(v(0.0, 980.0)));
animator.set_bounds(Some(Rect::new(0.0, 0.0, 800.0, 600.0)));

for _ in 0..120 {
    animator.tick(1.0 / 60.0);
}
assert!(animator.is_running());
```

See [Animator.md](Animator.md) for details.