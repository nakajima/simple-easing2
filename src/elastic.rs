use std::f32::consts::PI;

const C4: f32 = (2.0 * PI) / 3.0;
const C5: f32 = (2.0 * PI) / 4.5;

/// <https://easings.net/#easeInElastic>
#[must_use]
#[inline(always)]
pub fn elastic_in(t: f32) -> f32 {
    if t <= 0.0 {
        0.0
    } else if 1.0 <= t {
        1.0
    } else {
        -(10.0f32.mul_add(t, -10.0).exp2()) * (t.mul_add(10.0, -10.75) * C4).sin()
    }
}

/// <https://easings.net/#easeOutElastic>
#[must_use]
#[inline(always)]
pub fn elastic_out(t: f32) -> f32 {
    if t <= 0.0 {
        0.0
    } else if 1.0 <= t {
        1.0
    } else {
        (-10.0 * t)
            .exp2()
            .mul_add((t.mul_add(10.0, -0.75) * C4).sin(), 1.0)
    }
}

/// <https://easings.net/#easeInOutElastic>
#[must_use]
#[inline(always)]
pub fn elastic_in_out(t: f32) -> f32 {
    if t <= 0.0 {
        0.0
    } else if 1.0 <= t {
        1.0
    } else if t < 0.5 {
        -(20.0f32.mul_add(t, -10.0).exp2() * (20.0f32.mul_add(t, -11.125) * C5).sin()) / 2.0
    } else {
        ((-20.0f32).mul_add(t, 10.0).exp2() * (20.0f32.mul_add(t, -11.125) * C5).sin()) / 2.0 + 1.0
    }
}
