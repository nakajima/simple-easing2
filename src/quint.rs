/// <https://easings.net/#easeInQuint>
#[must_use]
#[inline(always)]
pub fn quint_in(t: f32) -> f32 {
    t * t * t * t
}

/// <https://easings.net/#easeOutQuint>
#[must_use]
#[inline(always)]
pub fn quint_out(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(5)
}

/// <https://easings.net/#easeInOutQuint>
#[must_use]
#[inline(always)]
pub fn quint_in_out(t: f32) -> f32 {
    if t < 0.5 {
        16.0 * t * t * t * t * t
    } else {
        1.0 - (-2.0f32).mul_add(t, 2.0).powi(5) / 2.0
    }
}
