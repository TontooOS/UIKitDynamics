//! # uikitdynamics
//!
//! A UIKit-Dynamics-style physics and animation engine for TontooOS.
//!
//! Pure physics — no GTK dependency. Provides spring-physics timing, easing
//! curves, composable behaviors (gravity, collisions, attachments, pushes,
//! snaps) and a frame-driven [`Animator`].
//!
//! The GTK binding layer (driving real widgets from items) lives in the
//! `uikit` crate, in its `animation` module.
//!
//! ```
//! use uikitdynamics::prelude::*;
//!
//! let mut animator = Animator::new();
//! let item = animator.add_item(DynamicItem::new(v(0.0, 0.0), Size::new(60.0, 60.0)));
//! let mut item = animator.item_mut(item).unwrap();
//! item.wake();
//! item.velocity = v(0.0, 0.0);
//! drop(item);
//!
//! animator.add_behavior(GravityBehavior::new(v(0.0, 980.0)));
//! animator.set_bounds(Some(Rect::new(0.0, 0.0, 800.0, 600.0)));
//!
//! for _ in 0..120 {
//!     animator.tick(1.0 / 60.0);
//! }
//! assert!(animator.is_running());
//! ```

pub mod animator;
pub mod behaviors;
pub mod easing;
pub mod math;
pub mod spring;

pub use animator::{Animator, DynamicItem};
pub use behaviors::{
    AttachmentBehavior, Behavior, CollisionBehavior, GravityBehavior, ItemProperties,
    PushBehavior, SnapBehavior,
};
pub use easing::{Easing, Tween};
pub use math::{Rect, Size, Vec2, v};
pub use spring::{Spring, SpringPreset};

/// Re-exports for convenient glob imports.
pub mod prelude {
    pub use super::{
        Animator, AttachmentBehavior, Behavior, CollisionBehavior, DynamicItem, Easing,
        GravityBehavior, ItemProperties, PushBehavior, Rect, SnapBehavior, Size, Spring,
        SpringPreset, Tween, Vec2, v,
    };
}