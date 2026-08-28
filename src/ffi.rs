//! C FFI exports for UIKitDynamics.

use std::ffi::CString;
use std::os::raw::{c_char, c_int};

use crate::easing::{Easing, Tween};
use crate::spring::SpringPreset;

fn easing_from(kind: c_int) -> Easing {
    match kind {
        1 => Easing::QuadIn,
        2 => Easing::QuadOut,
        3 => Easing::QuadInOut,
        4 => Easing::CubicIn,
        5 => Easing::CubicOut,
        6 => Easing::CubicInOut,
        7 => Easing::SineIn,
        8 => Easing::SineOut,
        9 => Easing::SineInOut,
        10 => Easing::BackOut,
        11 => Easing::BackIn,
        12 => Easing::BounceOut,
        _ => Easing::Linear,
    }
}

fn preset_from(kind: c_int) -> SpringPreset {
    match kind {
        1 => SpringPreset::Snappy,
        2 => SpringPreset::Bouncy,
        3 => SpringPreset::Soft,
        _ => SpringPreset::Default,
    }
}

/// The framework version as a static C string.
#[no_mangle]
pub extern "C" fn tontoo_uikitdynamics_version() -> *const c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr() as *const c_char
}

/// Apply an easing curve to a progress value `t` (0.0..=1.0).
///
/// `kind`: 0 linear, 1 quad-in, 2 quad-out, 3 quad-in-out, 4 cubic-in,
/// 5 cubic-out, 6 cubic-in-out, 7 sine-in, 8 sine-out, 9 sine-in-out,
/// 10 back-out, 11 back-in, 12 bounce-out.
#[no_mangle]
pub extern "C" fn tontoo_uikitdynamics_easing_apply(kind: c_int, t: f32) -> f32 {
    easing_from(kind).apply(t)
}

/// Advance a spring simulation one step.
///
/// `preset`: 0 default, 1 snappy, 2 bouncy, 3 soft. Reads the velocity from
/// `velocity`, writes the updated velocity back and returns the new value.
#[no_mangle]
pub unsafe extern "C" fn tontoo_uikitdynamics_spring_advance(
    preset: c_int,
    current: f32,
    target: f32,
    velocity: *mut f32,
    dt: f32,
) -> f32 {
    if velocity.is_null() {
        return current;
    }
    let mut vel = *velocity;
    let spring = preset_from(preset).spring();
    let next = spring.advance(current, target, &mut vel, dt);
    *velocity = vel;
    next
}

/// Whether a spring simulation has come to rest.
#[no_mangle]
pub unsafe extern "C" fn tontoo_uikitdynamics_spring_at_rest(
    preset: c_int,
    current: f32,
    target: f32,
    velocity: f32,
) -> c_int {
    let spring = preset_from(preset).spring();
    spring.at_rest(current, target, velocity) as c_int
}

/// Create an animation tween. Free with `tontoo_uikitdynamics_tween_free`.
///
/// See [`tontoo_uikitdynamics_easing_apply`] for easing `kind` values.
#[no_mangle]
pub extern "C" fn tontoo_uikitdynamics_tween_new(
    from: f32,
    to: f32,
    duration: f32,
    easing_kind: c_int,
) -> *mut Tween {
    let tween = Tween::new(from, to, duration).easing(easing_from(easing_kind));
    Box::into_raw(Box::new(tween))
}

/// The tween's current interpolated value.
///
/// # Safety
///
/// `tween` must be a handle returned by `tontoo_uikitdynamics_tween_new`.
#[no_mangle]
pub unsafe extern "C" fn tontoo_uikitdynamics_tween_value(tween: *mut Tween) -> f32 {
    if tween.is_null() {
        return 0.0;
    }
    (*tween).value()
}

/// The tween's progress between 0.0 and 1.0.
///
/// # Safety
///
/// `tween` must be a handle returned by `tontoo_uikitdynamics_tween_new`.
#[no_mangle]
pub unsafe extern "C" fn tontoo_uikitdynamics_tween_progress(tween: *mut Tween) -> f32 {
    if tween.is_null() {
        return 0.0;
    }
    (*tween).progress()
}

/// Advance a tween by `dt` seconds. Returns nonzero when finished.
///
/// # Safety
///
/// `tween` must be a handle returned by `tontoo_uikitdynamics_tween_new`.
#[no_mangle]
pub unsafe extern "C" fn tontoo_uikitdynamics_tween_advance(
    tween: *mut Tween,
    dt: f32,
) -> c_int {
    if tween.is_null() {
        return 1;
    }
    (*tween).advance(dt) as c_int
}

/// Destroy a tween handle.
///
/// # Safety
///
/// `tween` must be a handle returned by `tontoo_uikitdynamics_tween_new`
/// and must not be used afterwards.
#[no_mangle]
pub unsafe extern "C" fn tontoo_uikitdynamics_tween_free(tween: *mut Tween) {
    if !tween.is_null() {
        drop(Box::from_raw(tween));
    }
}

/// Free a string returned by this library.
///
/// # Safety
///
/// `s` must be a pointer returned by this API or null.
#[no_mangle]
pub unsafe extern "C" fn tontoo_uikitdynamics_string_free(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}
