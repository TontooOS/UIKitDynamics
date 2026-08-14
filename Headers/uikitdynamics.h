/*
 * TontooUIKitDynamics - C Header
 * TontooOS Physics and Animation Engine
 *
 * This header provides C bindings for the uikitdynamics library.
 */

#ifndef TONTOO_UIKITDYNAMICS_H
#define TONTOO_UIKITDYNAMICS_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ======================== */
/* Math                     */
/* ======================== */

typedef struct {
    float x;
    float y;
} uikitdynamics_vec2_t;

typedef struct {
    float width;
    float height;
} uikitdynamics_size_t;

typedef struct {
    float x;
    float y;
    float width;
    float height;
} uikitdynamics_rect_t;

/**
 * Construct a 2D vector.
 */
uikitdynamics_vec2_t uikitdynamics_vec2(float x, float y);

/**
 * Euclidean length of a vector.
 */
float uikitdynamics_vec2_length(const uikitdynamics_vec2_t *v);

/**
 * Unit vector in the same direction. Returns zero vector when length is zero.
 */
uikitdynamics_vec2_t uikitdynamics_vec2_normalized(const uikitdynamics_vec2_t *v);

/**
 * Dot product of two vectors.
 */
float uikitdynamics_vec2_dot(const uikitdynamics_vec2_t *a, const uikitdynamics_vec2_t *b);

/**
 * Whether a rectangle contains a point (inclusive).
 *
 * @return 1 if contained, 0 otherwise
 */
bool uikitdynamics_rect_contains(const uikitdynamics_rect_t *r, const uikitdynamics_vec2_t *p);

/**
 * Clamp a point into a rectangle.
 */
uikitdynamics_vec2_t uikitdynamics_rect_clamp(const uikitdynamics_rect_t *r, const uikitdynamics_vec2_t *p);

/* ======================== */
/* Easing                   */
/* ======================== */

typedef enum {
    UIKITDYNAMICS_EASING_LINEAR = 0,
    UIKITDYNAMICS_EASING_QUAD_IN,
    UIKITDYNAMICS_EASING_QUAD_OUT,
    UIKITDYNAMICS_EASING_QUAD_IN_OUT,
    UIKITDYNAMICS_EASING_CUBIC_IN,
    UIKITDYNAMICS_EASING_CUBIC_OUT,
    UIKITDYNAMICS_EASING_CUBIC_IN_OUT,
    UIKITDYNAMICS_EASING_SINE_IN,
    UIKITDYNAMICS_EASING_SINE_OUT,
    UIKITDYNAMICS_EASING_SINE_IN_OUT,
    UIKITDYNAMICS_EASING_BACK_OUT,
    UIKITDYNAMICS_EASING_BACK_IN,
    UIKITDYNAMICS_EASING_BOUNCE_OUT,
    UIKITDYNAMICS_EASING_COUNT
} uikitdynamics_easing_t;

/**
 * Apply an easing curve to a progress value in [0, 1].
 *
 * Input is clamped to [0, 1]. Output is in [0, 1], except back/bounce curves
 * which overshoot slightly.
 */
float uikitdynamics_easing_apply(uikitdynamics_easing_t easing, float t);

/* ======================== */
/* Spring                   */
/* ======================== */

typedef struct {
    float stiffness;
    float damping;
    float mass;
    float settle_threshold;
} uikitdynamics_spring_t;

/**
 * Default snappy spring configuration.
 */
uikitdynamics_spring_t uikitdynamics_spring_default(void);

/**
 * Softer, slower spring configuration.
 */
uikitdynamics_spring_t uikitdynamics_spring_soft(void);

/**
 * Over/under-damped bouncy spring configuration.
 */
uikitdynamics_spring_t uikitdynamics_spring_bouncy(void);

/**
 * Advance the spring one integration step.
 *
 * @param spring Spring configuration
 * @param current Current value
 * @param target Target value
 * @param velocity In/out velocity, updated in place
 * @param dt Frame delta in seconds (clamped to [0, 1/20])
 * @return The new value
 */
float uikitdynamics_spring_advance(
    const uikitdynamics_spring_t *spring,
    float current,
    float target,
    float *velocity,
    float dt);

/**
 * Whether the spring is essentially at rest.
 *
 * @return 1 if at rest, 0 otherwise
 */
bool uikitdynamics_spring_at_rest(
    const uikitdynamics_spring_t *spring,
    float current,
    float target,
    float velocity);

/* ======================== */
/* Behaviors                */
/* ======================== */

typedef enum {
    UIKITDYNAMICS_BEHAVIOR_GRAVITY = 0,
    UIKITDYNAMICS_BEHAVIOR_COLLISION,
    UIKITDYNAMICS_BEHAVIOR_ATTACHMENT,
    UIKITDYNAMICS_BEHAVIOR_PUSH,
    UIKITDYNAMICS_BEHAVIOR_SNAP,
    UIKITDYNAMICS_BEHAVIOR_ITEM_PROPERTIES
} uikitdynamics_behavior_t;

/**
 * Create a gravity behavior: constant acceleration applied to every item.
 */
uikitdynamics_behavior_t uikitdynamics_behavior_gravity(uikitdynamics_vec2_t vector);

/**
 * Create a snap behavior: springs an item exactly onto a target point.
 *
 * @param item_index Index of the item this behavior affects
 * @param target Target position
 */
uikitdynamics_behavior_t uikitdynamics_behavior_snap(size_t item_index, uikitdynamics_vec2_t target);

/**
 * Create a push behavior: one-shot or continuous impulse.
 *
 * @param item_index Index of the item this behavior affects
 * @param direction Push direction (normalized internally)
 * @param magnitude Impulse magnitude
 */
uikitdynamics_behavior_t uikitdynamics_behavior_push(
    size_t item_index,
    uikitdynamics_vec2_t direction,
    float magnitude);

/**
 * Free a behavior created by the factory functions above.
 */
void uikitdynamics_behavior_free(uikitdynamics_behavior_t behavior);

/* ======================== */
/* Animator                 */
/* ======================== */

typedef struct uikitdynamics_animator_t uikitdynamics_animator_t;

/**
 * Create an empty animator (opaque handle).
 */
uikitdynamics_animator_t *uikitdynamics_animator_new(void);

/**
 * Free an animator.
 */
void uikitdynamics_animator_free(uikitdynamics_animator_t *animator);

/**
 * Set global boundary bounds for collision.
 *
 * @param bounds Bounds rectangle, or NULL to disable boundaries
 */
void uikitdynamics_animator_set_bounds(uikitdynamics_animator_t *animator, const uikitdynamics_rect_t *bounds);

/**
 * Add a physics item and return its stable index.
 */
size_t uikitdynamics_animator_add_item(
    uikitdynamics_animator_t *animator,
    uikitdynamics_vec2_t position,
    uikitdynamics_size_t size);

/**
 * Number of items in the animator.
 */
size_t uikitdynamics_animator_item_count(const uikitdynamics_animator_t *animator);

/**
 * Wake an item so integration (and behaviors) apply to it.
 */
void uikitdynamics_animator_wake(uikitdynamics_animator_t *animator, size_t index);

/**
 * Add a behavior to the animator. Ownership is transferred.
 */
void uikitdynamics_animator_add_behavior(uikitdynamics_animator_t *animator, uikitdynamics_behavior_t behavior);

/**
 * Advance the simulation by `dt` seconds.
 */
void uikitdynamics_animator_tick(uikitdynamics_animator_t *animator, float dt);

/**
 * Get the current position of an item.
 *
 * @return 1 on success, 0 when the index is out of range
 */
bool uikitdynamics_animator_item_position(
    const uikitdynamics_animator_t *animator,
    size_t index,
    uikitdynamics_vec2_t *out);

/**
 * Whether any item is moving or any spring is unsettled.
 *
 * @return 1 if running, 0 otherwise
 */
bool uikitdynamics_animator_is_running(const uikitdynamics_animator_t *animator);

#ifdef __cplusplus
}
#endif

#endif /* TONTOO_UIKITDYNAMICS_H */