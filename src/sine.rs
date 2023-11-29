use std::f32::consts::PI;

/// <https://easings.net/#easeInSine>
#[must_use]
#[inline(always)]
pub fn sine_in(t: f32) -> f32 {
    1.0 - (t * PI / 2.0).cos()
}

/// <https://easings.net/#easeOutSine>
#[must_use]
#[inline(always)]
pub fn sine_out(t: f32) -> f32 {
    (t * PI / 2.0).sin()
}

/// <https://easings.net/#easeInOutSine>
#[must_use]
#[inline(always)]
pub fn sine_in_out(t: f32) -> f32 {
    -((PI * t).cos() - 1.0) / 2.0
}
