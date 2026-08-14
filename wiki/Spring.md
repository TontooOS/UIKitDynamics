# Spring

A damped harmonic oscillator in the spirit of Apple's `UISpringTimingParameters`.
Springs are the foundation of the "full animation" feel in TontooOS: the dock
bounce, the genie effect, and window snaps all use spring curves instead of
fixed easing functions.

## Spring

```rust
pub struct Spring {
    pub stiffness: f32,
    pub damping: f32,
    pub mass: f32,
    pub settle_threshold: f32,
}
```

The oscillator integrates a value toward a target using a semi-implicit
(symplectic) Euler step. Under-damped parameters make the value overshoot and
bounce back.

### Constructors

| Constructor | Behavior |
|---|---|
| `new() -> Self` | Default snappy spring (`stiffness 180`, `damping 25`, `mass 1`). |
| `soft() -> Self` | Softer, slower spring. |
| `bouncy() -> Self` | Over/under-damped bouncy spring. |

### Builder Methods

| Method | Effect |
|---|---|
| `stiffness(mut self, f32) -> Self` | Sets stiffness (force per unit distance). |
| `damping(mut self, f32) -> Self` | Sets damping (force per unit velocity). |
| `mass(mut self, f32) -> Self` | Sets oscillating mass. |

### Methods

| Method | Behavior |
|---|---|
| `advance(current, target, &mut velocity, dt) -> f32` | One integration step; returns the new value. `dt` is clamped to `[0, 1/20]`. |
| `advance_vec(current, target, &mut velocity, dt) -> Vec2` | Advances a 2D position. |
| `at_rest(current, target, velocity) -> bool` | True when within `settle_threshold` and velocity is negligible. |
| `at_rest_vec(current, target, velocity) -> bool` | 2D variant. |

## SpringPreset

Named spring presets matching common TontooOS animations.

```rust
pub enum SpringPreset { Default, Snappy, Bouncy, Soft }
```

### Resolve

```rust
pub fn spring(self) -> Spring
```

| Preset | Result |
|---|---|
| `Default` | `Spring::new()` |
| `Snappy` | Fast, responsive snap (`stiffness 300`, `damping 30`). |
| `Bouncy` | `Spring::bouncy()`. |
| `Soft` | `Spring::soft()`. |

## Usage / Example

```rust
use uikitdynamics::spring::Spring;

let spring = Spring::new();
let mut velocity = 0.0;
let mut current = 0.0;
for _ in 0..1000 {
    current = spring.advance(current, 100.0, &mut velocity, 1.0 / 120.0);
    if spring.at_rest(current, 100.0, velocity) {
        break;
    }
}
assert!((current - 100.0).abs() < 1.0);
```

## Cross References

- [Easing.md](Easing.md) – fixed-duration alternative
- [Behaviors.md](Behaviors.md) – attachment and snap behaviors use springs
- [Animator.md](Animator.md) – scale and opacity springs on `DynamicItem`