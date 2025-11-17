use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationDirection {
    FadeIn,
    FadeOut,
    None,
}

/// Calculate animation progress with ease-out cubic easing
pub fn get_animation_progress(start: Option<Instant>, duration_ms: f32) -> f32 {
    if let Some(start_time) = start {
        let elapsed = start_time.elapsed().as_millis() as f32;
        let progress = (elapsed / duration_ms).min(1.0);
        
        // Ease-out cubic for smooth deceleration
        1.0 - (1.0 - progress).powi(3)
    } else {
        1.0
    }
}

/// Calculate opacity based on animation direction and progress
pub fn get_opacity(direction: AnimationDirection, progress: f32) -> f32 {
    match direction {
        AnimationDirection::FadeIn => progress,
        AnimationDirection::FadeOut => 1.0 - progress,
        AnimationDirection::None => 1.0,
    }
}
