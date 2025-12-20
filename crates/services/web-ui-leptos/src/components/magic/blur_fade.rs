//! Blur fade animation component.
//!
//! Creates a blur + fade entrance animation.

use leptos::prelude::*;

/// Direction of the fade animation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FadeDirection {
    /// Fade in from above
    Up,
    /// Fade in from below
    #[default]
    Down,
    /// Fade in from left
    Left,
    /// Fade in from right
    Right,
    /// Fade in from center (no translation)
    Center,
}

impl FadeDirection {
    fn animation_class(&self) -> &'static str {
        match self {
            FadeDirection::Up => "animate-blur-fade-up",
            FadeDirection::Down => "animate-blur-fade-down",
            FadeDirection::Left => "animate-blur-fade-left",
            FadeDirection::Right => "animate-blur-fade-right",
            FadeDirection::Center => "animate-blur-fade",
        }
    }
}

/// Blur fade entrance animation component.
///
/// # Props
/// - `direction`: Direction to fade from
/// - `delay`: Delay before animation in milliseconds
/// - `duration`: Animation duration in milliseconds
/// - `blur`: Initial blur amount in pixels
/// - `class`: Additional CSS classes
///
/// # Example
/// ```rust,ignore
/// view! {
///     <BlurFade direction=FadeDirection::Up delay=200>
///         <h1>"Welcome"</h1>
///     </BlurFade>
/// }
/// ```
#[component]
pub fn BlurFade(
    /// Direction to fade from
    #[prop(default = FadeDirection::Down)]
    direction: FadeDirection,
    /// Delay before animation in milliseconds
    #[prop(default = 0)]
    delay: u32,
    /// Animation duration in milliseconds
    #[prop(default = 500)]
    duration: u32,
    /// Initial blur amount in pixels
    #[prop(default = 6)]
    blur: u32,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
    /// Children content
    children: Children,
) -> impl IntoView {
    let extra = class.unwrap_or_default();
    let animation = direction.animation_class();

    let style = format!(
        "--blur-fade-blur: {}px; animation-delay: {}ms; animation-duration: {}ms;",
        blur, delay, duration
    );

    view! {
        <div
            class={format!("opacity-0 {} {}", animation, extra)}
            style={style}
        >
            {children()}
        </div>
    }
}

/// Stagger container for multiple blur fade items.
/// Automatically applies increasing delays to children.
#[component]
pub fn BlurFadeStagger(
    /// Base delay in milliseconds
    #[prop(default = 0)]
    base_delay: u32,
    /// Delay increment between items in milliseconds
    #[prop(default = 100)]
    stagger: u32,
    /// Direction for all children
    #[prop(default = FadeDirection::Down)]
    direction: FadeDirection,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
    /// Children content
    children: Children,
) -> impl IntoView {
    let extra = class.unwrap_or_default();
    let animation = direction.animation_class();

    // Use CSS custom properties for stagger
    let style = format!(
        "--stagger-delay: {}ms; --base-delay: {}ms;",
        stagger, base_delay
    );

    view! {
        <div
            class={format!("stagger {} {}", animation, extra)}
            style={style}
        >
            {children()}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fade_direction_up() {
        assert_eq!(FadeDirection::Up.animation_class(), "animate-blur-fade-up");
    }

    #[test]
    fn test_fade_direction_down() {
        assert_eq!(
            FadeDirection::Down.animation_class(),
            "animate-blur-fade-down"
        );
    }

    #[test]
    fn test_fade_direction_center() {
        assert_eq!(FadeDirection::Center.animation_class(), "animate-blur-fade");
    }
}
