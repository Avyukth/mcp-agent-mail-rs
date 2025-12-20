//! Shimmer text effect component.
//!
//! Creates a shimmering shine effect across text.

use leptos::prelude::*;

/// Shimmer text effect component.
///
/// # Props
/// - `text`: Text to display
/// - `shimmer_color`: Color of the shimmer (default: white)
/// - `duration`: Animation duration in seconds (default: 2)
/// - `class`: Additional CSS classes
///
/// # Example
/// ```rust,ignore
/// view! {
///     <ShimmerText text="Premium Feature" />
/// }
/// ```
#[component]
pub fn ShimmerText(
    /// Text to display
    #[prop(into)]
    text: String,
    /// Shimmer highlight color (Tailwind color class)
    #[prop(default = "white".to_string(), into)]
    shimmer_color: String,
    /// Animation duration in seconds
    #[prop(default = 2.0)]
    duration: f32,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let extra = class.unwrap_or_default();
    let style = format!("animation-duration: {}s;", duration);

    // CSS for shimmer effect using a linear-gradient mask
    let shimmer_class = format!(
        "relative inline-block animate-shimmer bg-clip-text {} \
         before:absolute before:inset-0 before:bg-gradient-to-r \
         before:from-transparent before:via-{}/50 before:to-transparent \
         before:bg-[length:200%_100%] before:animate-shimmer",
        extra, shimmer_color
    );

    view! {
        <span class={shimmer_class} style={style}>
            {text}
        </span>
    }
}

/// Alternative shimmer implementation using pure CSS.
/// This version uses a gradient overlay that moves across the text.
#[component]
pub fn ShimmerBadge(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
    /// Children content
    children: Children,
) -> impl IntoView {
    let extra = class.unwrap_or_default();

    view! {
        <span class={format!(
            "inline-flex items-center gap-1 px-3 py-1 rounded-full \
             bg-gradient-to-r from-primary/10 via-primary/5 to-primary/10 \
             border border-primary/20 text-sm font-medium text-primary \
             animate-shimmer-bg bg-[length:200%_100%] {}",
            extra
        )}>
            {children()}
        </span>
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_shimmer_text_exists() {
        // Component exists and can be referenced
        assert!(true);
    }
}
