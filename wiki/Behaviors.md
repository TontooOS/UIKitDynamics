# Behaviors

Composable force modifiers applied to the items of an [`Animator`](Animator.md),
modeled after Apple's UIKit Dynamics.

## Behavior Trait

```rust
pub trait Behavior: Send {
    fn apply(&mut self, items: &mut [DynamicItem], dt: f32);
    fn active(&self) -> bool { true }
}
```

`apply` runs every animation frame. Behaviors are `Send` so the animator can
run inside GTK's `Send` tick callback. `active() -> false` marks a finished
one-shot behavior; the animator removes it.

## GravityBehavior

Applies a constant acceleration to every item.

```rust
pub struct GravityBehavior { pub vector: Vec2, pub strength: f32 }
```

### Constructors

```rust
pub fn new(vector: Vec2) -> Self
```

`Default` is gravity pulling straight down: `v(0.0, 980.0)`.

### Methods

| Method | Effect |
|---|---|
| `strength(mut self, f32) -> Self` | Scales the gravitational force per frame. |

## CollisionBehavior

Resolves collisions against an optional boundary rectangle and between items.

```rust
pub struct CollisionBehavior { pub bounds: Option<Rect>, pub collide_items: bool }
```

### Constructors

```rust
pub fn new(bounds: Option<Rect>) -> Self
```

### Methods

| Method | Effect |
|---|---|
| `bounds(mut self, Rect) -> Self` | Confines items to a rectangle. |
| `collide_items(mut self, bool) -> Self` | Enables/disables item-to-item collision. |

Items bounce per `elasticity`; collisions use simple circle/circle resolution
with fractional mass.

## AttachmentBehavior

Springs an item toward an anchor point.

```rust
pub struct AttachmentBehavior { pub item_index: Option<usize>, pub anchor: Vec2, pub length: f32, pub spring: Spring }
```

### Constructors

```rust
pub fn new(item_index: usize, anchor: Vec2) -> Self
```

### Methods

| Method | Effect |
|---|---|
| `length(mut self, f32) -> Self` | Rest distance ("rubber band"); `0` springs to the anchor. |
| `spring(mut self, Spring) -> Self` | Overrides spring tuning. |

## PushBehavior

Applies a one-shot impulse or a continuous acceleration.

```rust
pub struct PushBehavior { pub item_index: Option<usize>, pub magnitude: f32, pub continuous: bool, pub direction: Vec2 }
```

### Constructors

```rust
pub fn new(item_index: usize, direction: Vec2, magnitude: f32) -> Self
```

### Methods

| Method | Effect |
|---|---|
| `continuous(mut self, bool) -> Self` | `true` re-applies the push every frame. |

One-shot pushes return `active() == false` after being consumed and are removed
by the animator.

## SnapBehavior

Springs an item exactly onto a target point, mirroring `UISnapBehavior`.

```rust
pub struct SnapBehavior { pub item_index: Option<usize>, pub target: Vec2, pub spring: Spring }
```

### Constructors

```rust
pub fn new(item_index: usize, target: Vec2) -> Self
```

### Methods

| Method | Effect |
|---|---|
| `spring(mut self, Spring) -> Self` | Overrides spring tuning. |

When the spring settles, the item is snapped exactly to the target and its
velocity zeroed.

## ItemProperties

A non-physics "behavior" that tunes per-item mass and elasticity.

```rust
pub struct ItemProperties { pub mass: f32, pub elasticity: f32, pub resistance: f32 }
```

`Default` is `mass 1.0`, `elasticity 0.6`, `resistance 0.0`. Applied each
frame, `resistance` damps velocity by `velocity *= (1 - resistance * dt)`.

## Usage / Example

```rust
use uikitdynamics::prelude::*;

let mut animator = Animator::new();
let index = animator.add_item(DynamicItem::new(v(0.0, 0.0), Size::new(30.0, 30.0)));
animator.add_behavior(GravityBehavior::default());
animator.add_behavior(SnapBehavior::new(index, v(200.0, 150.0)));
animator.set_bounds(Some(Rect::new(0.0, 0.0, 800.0, 600.0)));
for _ in 0..300 {
    animator.tick(1.0 / 60.0);
}
```

## Cross References

- [Animator.md](Animator.md) – how behaviors are stored and evaluated
- [Spring.md](Spring.md) – spring tuning used by attachments and snaps
- [Math.md](Math.md) – `Rect` bounds and `Vec2` forces