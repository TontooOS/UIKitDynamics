# Animator

The physics simulation core: owns items and behaviors and advances them each
frame. The AST is pure data; items know nothing about GTK. Real widgets are
driven by `uikit::animation::AnimatedWidget`.

## DynamicItem

A physics body inside an `Animator`.

```rust
pub struct DynamicItem {
    pub position: Vec2,
    pub velocity: Vec2,
    pub size: Size,
    pub mass: f32,
    pub elasticity: f32,
    pub resting: bool,
    pub scale: f32,
    pub scale_target: f32,
    pub scale_velocity: f32,
    pub scale_spring: Option<Spring>,
    pub opacity: f32,
    pub opacity_target: f32,
    pub opacity_velocity: f32,
    pub opacity_spring: Option<Spring>,
}
```

### Constructors

```rust
pub fn new(position: Vec2, size: Size) -> Self
```

Creates a resting item; both scale and opacity springs are `None`.

### Methods

| Method | Behavior |
|---|---|
| `animate_scale(&mut self, target: f32)` | Spring-animates scale to target. |
| `animate_opacity(&mut self, target: f32)` | Spring-animates opacity to target. |
| `set_scale(&mut self, f32)` | Set scale instantly. |
| `set_opacity(&mut self, f32)` | Set opacity instantly. |
| `is_settled(&self) -> bool` | True when velocity length is below `0.01`. |
| `wake(&mut self)` | Re-enables integration (`resting = false`). |

## Animator

Owns items and behaviors and advances them each frame.

```rust
pub struct Animator {
    items: Vec<DynamicItem>,
    behaviors: Vec<Box<dyn Behavior>>,
    pub bounds: Option<Rect>,
}
```

### Constructors

```rust
pub fn new() -> Self
```

### Methods

| Method | Behavior |
|---|---|
| `set_bounds(&mut self, Option<Rect>)` | Sets boundary bounds and adds a `CollisionBehavior`. |
| `add_item(&mut self, DynamicItem) -> usize` | Adds an item and returns its stable index. |
| `item_count(&self) -> usize` | Number of items. |
| `item(&self, usize) -> Option<&DynamicItem>` | Borrows an item by index. |
| `item_mut(&mut self, usize) -> Option<&mut DynamicItem>` | Mutably borrows an item. |
| `add_behavior(&mut self, impl Behavior + 'static)` | Adds a composable behavior. |
| `tick(&mut self, dt: f32)` | Advances the simulation by `dt` seconds. |
| `is_running(&self) -> bool` | True when any item moves or a spring is unsettled. |

### Tick Pipeline

`tick` runs three phases per frame:

1. Apply each active behavior to all items; remove finished one-shots.
2. Integrate velocities into positions (resting items are skipped).
3. Advance the scale and opacity springs; clear springs that settled.

## Usage / Example

```rust
use uikitdynamics::prelude::*;

let mut animator = Animator::new();
let item = animator.add_item(DynamicItem::new(v(0.0, 0.0), Size::new(20.0, 20.0)));
animator.item_mut(item).unwrap().wake();

animator.add_behavior(GravityBehavior::default());
animator.set_bounds(Some(Rect::new(0.0, 0.0, 100.0, 100.0)));
animator.tick(1.0 / 60.0);
```

## Driving a GTK App

Use `uikit::App::set_tick` to advance the animator every frame and
`AnimatedWidget::apply_from` to push item state into widgets:

```rust
app.set_tick(move |dt| {
    animator.tick(dt);
    for gate in animated.iter() {
        gate.apply_from(&animator);
    }
});
```

## Cross References

- [Behaviors.md](Behaviors.md) – composable forces applied each tick
- [Spring.md](Spring.md) – scale/opacity springs on items
- [Math.md](Math.md) – `Vec2` positions and `Rect` bounds