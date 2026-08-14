# Easing

Named easing curves and a fixed-duration `Tween` for deterministic animations
(fade-in, slide panels, notification banners). Spring physics live in
[Spring.md](Spring.md).

## Easing

```rust
pub enum Easing {
    Linear, QuadIn, QuadOut, QuadInOut,
    CubicIn, CubicOut, CubicInOut,
    SineIn, SineOut, SineInOut,
    BackOut, BackIn, BounceOut,
}
```

### Apply

```rust
pub fn apply(&self, t: f32) -> f32
```

Applies the easing curve to a progress value in `[0, 1]`. Input is clamped to
`[0, 1]`; output is also `[0, 1]`, except `BackIn`/`BackOut`/`BounceOut` which
overshoot past the target slightly before settling.

## Tween

A fixed-duration, single-value tween driven by an `Easing` curve.

```rust
pub struct Tween { /* ... */ }
```

### Constructors

```rust
pub fn new(from: f32, to: f32, duration: f32) -> Self
```

Creates a tween from `from` to `to` over `duration` seconds using a
`QuadOut` easing by default.

### Methods

| Method | Behavior |
|---|---|
| `easing(mut self, easing: Easing) -> Self` | Overrides the easing curve. |
| `progress(&self) -> f32` | Progress in `[0, 1]`; 0 = start, 1 = done. |
| `value(&self) -> f32` | Current value with easing applied. |
| `advance(&mut self, dt: f32) -> bool` | Advances by `dt` seconds; returns true when finished. |
| `finished(&self) -> bool` | True when the tween reached its duration. |
| `target(&self) -> f32` | Target value. |

> **Note:** `advance` returns `false` until the elapsed time reaches
> `duration`. Duration is clamped to a minimum of `0.0001` seconds to avoid
> division by zero.

## Usage / Example

```rust
use uikitdynamics::easing::{Easing, Tween};

let mut tween = Tween::new(0.0, 10.0, 1.0).easing(Easing::CubicOut);
assert!(!tween.advance(0.5));
assert!(!tween.finished());

while !tween.advance(1.0 / 60.0) {}
assert_eq!(tween.value(), 10.0);
```

## Cross References

- [Spring.md](Spring.md) – spring curves are preferred for interactive motion
- [Animator.md](Animator.md) – springs drive scale/opacity inside items