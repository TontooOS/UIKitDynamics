# Math

Primitive 2D geometry used across the physics engine: `Vec2`, `Size` and
`Rect`. These are plain `Copy` data types with the arithmetic operators and
normalization helpers the physics code needs.

## Vec2

A two-dimensional vector with `x` and `y` fields.

```rust
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
```

### Constructor

```rust
pub const fn new(x: f32, y: f32) -> Self
```

### Operators

`Vec2` implements `Add`, `Sub` and `Mul<f32>`.

```rust
let a = v(1.0, 2.0);
let b = a + v(3.0, 4.0); // v(4.0, 6.0)
let c = a * 2.0;         // v(2.0, 4.0)
```

### Methods

| Method | Behavior |
|---|---|
| `length(&self) -> f32` | Euclidean length. |
| `normalized(&self) -> Vec2` | Unit vector; returns `Vec2::ZERO` when length is near zero. |
| `dot(&self, other: Vec2) -> f32` | Dot product. |
| `scale(&self, scalar: f32) -> Vec2` | Scales both components. |

### Convenience Constructor

```rust
pub fn v(x: f32, y: f32) -> Vec2
```

## Size

A 2D size used for collision bounds of physics items.

```rust
pub struct Size {
    pub width: f32,
    pub height: f32,
}
```

### Constructors

```rust
pub const fn new(width: f32, height: f32) -> Self
```

## Rect

An axis-aligned rectangle.

```rust
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
```

### Methods

| Method | Behavior |
|---|---|
| `new(x, y, width, height) -> Self` | Constructs a rectangle. |
| `min(&self) -> Vec2` | Minimum corner (top-left). |
| `max(&self) -> Vec2` | Maximum corner (bottom-right). |
| `contains(&self, p: Vec2) -> bool` | True when `p` lies on or inside. |
| `clamp(&self, p: Vec2) -> Vec2` | Clamps `p` to the rectangle. |

## Usage / Example

```rust
use uikitdynamics::math::{Rect, Vec2, v};

let r = Rect::new(0.0, 0.0, 100.0, 100.0);
let p = v(150.0, 150.0);
let clamped = r.clamp(p); // v(100.0, 100.0)
assert!(!r.contains(p));
assert_eq!(clamped, r.max());
```

## Cross References

- [Animator.md](Animator.md) – how positions and collisions use these types
- [Behaviors.md](Behaviors.md) – colliders operate on `Rect` bounds and `Vec2` velocities