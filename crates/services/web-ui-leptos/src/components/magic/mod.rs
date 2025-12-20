//! Magic UI - Animated components for enhanced visual experiences.
//!
//! This module contains animated components inspired by Magic UI,
//! ported to Leptos with Tailwind CSS animations.

pub mod animated_gradient;
pub mod blur_fade;
pub mod grid_pattern;
pub mod number_counter;
pub mod shimmer_text;
pub mod typing_text;

pub use animated_gradient::{AnimatedGradient, AnimatedGradientText, GradientDirection};
pub use blur_fade::{BlurFade, BlurFadeStagger, FadeDirection};
pub use grid_pattern::{GridPattern, GridPatternMasked, GridType};
pub use number_counter::{FormattedNumber, NumberCounter};
pub use shimmer_text::{ShimmerBadge, ShimmerText};
pub use typing_text::{TypingText, TypingTextCss};
