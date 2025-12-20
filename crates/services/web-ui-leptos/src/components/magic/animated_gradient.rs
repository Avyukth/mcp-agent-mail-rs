//! Animated gradient background component.
//!
//! Creates a smoothly transitioning gradient background effect.

use leptos::prelude::*;

/// Direction of the gradient animation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum GradientDirection {
    /// Horizontal movement
    #[default]
    Horizontal,
    /// Vertical movement
    Vertical,
    /// Diagonal movement
    Diagonal,
}

impl GradientDirection {
    fn animation_class(&self) -> &'static str {
        match self {
            GradientDirection::Horizontal => "animate-gradient-x",
            GradientDirection::Vertical => "animate-gradient-y",
            GradientDirection::Diagonal => "animate-gradient-xy",
        }
    }
}

/// Animated gradient background component.
///
/// # Props
/// - `from_color`: Starting gradient color
/// - `via_color`: Middle gradient color (optional)
/// - `to_color`: Ending gradient color
/// - `direction`: Animation direction
/// - `duration`: Animation duration in seconds (default: 3)
/// - `class`: Additional CSS classes
///
/// # Example
/// ```rust,ignore
/// view! {
///     <AnimatedGradient
///         from_color="from-indigo-500"
///         via_color="via-purple-500"
///         to_color="to-pink-500"
///     >
///         <h1 class="text-white">"Welcome"</h1>
///     </AnimatedGradient>
/// }
/// ```
#[component]
pub fn AnimatedGradient(
    /// Starting gradient color class (e.g., "from-indigo-500")
    #[prop(into)]
    from_color: String,
    /// Middle gradient color class (optional)
    #[prop(optional, into)]
    via_color: Option<String>,
    /// Ending gradient color class (e.g., "to-pink-500")
    #[prop(into)]
    to_color: String,
    /// Animation direction
    #[prop(default = GradientDirection::Horizontal)]
    direction: GradientDirection,
    /// Animation duration in seconds
    #[prop(default = 3.0)]
    duration: f32,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
    /// Children content
    children: Children,
) -> impl IntoView {
    let via = via_color.unwrap_or_default();
    let extra = class.unwrap_or_default();
    let animation = direction.animation_class();

    // Build gradient classes
    let gradient_classes = format!(
        "bg-gradient-to-r {} {} {} bg-[length:200%_200%] {} {}",
        from_color, via, to_color, animation, extra
    );

    let style = format!("animation-duration: {}s;", duration);

    view! {
        <div class={gradient_classes} style={style}>
            {children()}
        </div>
    }
}

/// Animated gradient text effect.
///
/// Makes text have a moving gradient color effect.
#[component]
pub fn AnimatedGradientText(
    /// Starting gradient color class
    #[prop(into)]
    from_color: String,
    /// Middle gradient color class (optional)
    #[prop(optional, into)]
    via_color: Option<String>,
    /// Ending gradient color class
    #[prop(into)]
    to_color: String,
    /// Animation duration in seconds
    #[prop(default = 3.0)]
    duration: f32,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
    /// Text content
    children: Children,
) -> impl IntoView {
    let via = via_color.unwrap_or_default();
    let extra = class.unwrap_or_default();

    let gradient_classes = format!(
        "inline-block bg-gradient-to-r {} {} {} bg-[length:200%_200%] bg-clip-text text-transparent animate-gradient-x {}",
        from_color, via, to_color, extra
    );

    let style = format!("animation-duration: {}s;", duration);

    view! {
        <span class={gradient_classes} style={style}>
            {children()}
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradient_direction_horizontal() {
        assert_eq!(
            GradientDirection::Horizontal.animation_class(),
            "animate-gradient-x"
        );
    }

    #[test]
    fn test_gradient_direction_vertical() {
        assert_eq!(
            GradientDirection::Vertical.animation_class(),
            "animate-gradient-y"
        );
    }

    #[test]
    fn test_gradient_direction_diagonal() {
        assert_eq!(
            GradientDirection::Diagonal.animation_class(),
            "animate-gradient-xy"
        );
    }
}
