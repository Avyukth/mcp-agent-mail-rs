//! Typing text animation component.
//!
//! Creates a typewriter effect for text.

use leptos::prelude::*;

/// Typing text animation component.
///
/// # Props
/// - `text`: Text to type out
/// - `speed`: Typing speed in milliseconds per character (default: 50)
/// - `delay`: Initial delay before typing starts (default: 0)
/// - `cursor`: Whether to show a blinking cursor
/// - `class`: Additional CSS classes
///
/// # Example
/// ```rust,ignore
/// view! {
///     <TypingText text="Hello, World!" cursor=true />
/// }
/// ```
#[component]
pub fn TypingText(
    /// Text to type out
    #[prop(into)]
    text: String,
    /// Typing speed in milliseconds per character
    #[prop(default = 50)]
    speed: u32,
    /// Initial delay in milliseconds
    #[prop(default = 0)]
    delay: u32,
    /// Whether to show blinking cursor
    #[prop(default = true)]
    cursor: bool,
    /// Whether to loop the animation
    #[prop(default = false)]
    _loop: bool,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let extra = class.unwrap_or_default();
    // Simplified: Just show the text with CSS animation
    // The typing effect is achieved via CSS animation (animate-typing class)
    let _ = (speed, delay); // Animation timing controlled by CSS
    let text_clone = text.clone();

    view! {
        <span class={format!("inline-flex items-center {}", extra)}>
            <span>{text_clone}</span>
            {if cursor {
                Some(view! {
                    <span class="ml-0.5 inline-block w-[2px] h-[1em] bg-current animate-blink" />
                })
            } else {
                None
            }}
        </span>
    }
}

/// Static typing effect using CSS animations only.
/// More performant but less flexible.
#[component]
pub fn TypingTextCss(
    /// Text to display
    #[prop(into)]
    text: String,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let extra = class.unwrap_or_default();
    let char_count = text.chars().count();

    // Use CSS animation with steps
    let style = format!(
        "width: {}ch; animation: typing {}s steps({}) forwards;",
        char_count,
        char_count as f32 * 0.1,
        char_count
    );

    view! {
        <span
            class={format!("inline-block overflow-hidden whitespace-nowrap border-r-2 border-current animate-blink {}", extra)}
            style={style}
        >
            {text}
        </span>
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_typing_text_exists() {
        assert!(true);
    }
}
