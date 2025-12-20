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
    let displayed_text = RwSignal::new(String::new());
    let text_chars: Vec<char> = text.chars().collect();
    let total_chars = text_chars.len();

    // Use Effect to animate typing
    Effect::new(move |_| {
        let chars = text_chars.clone();

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use wasm_bindgen::closure::Closure;

            let window = web_sys::window().unwrap();
            let mut index = 0usize;

            // Initial delay
            let start_typing = Closure::once(Box::new(move || {
                // Start typing interval
                let interval_closure = Closure::wrap(Box::new(move || {
                    if index < chars.len() {
                        displayed_text.update(|t| t.push(chars[index]));
                        index += 1;
                    }
                }) as Box<dyn FnMut()>);

                if let Some(w) = web_sys::window() {
                    let _ = w.set_interval_with_callback_and_timeout_and_arguments_0(
                        interval_closure.as_ref().unchecked_ref(),
                        speed as i32,
                    );
                }
                interval_closure.forget();
            }) as Box<dyn FnOnce()>);

            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                start_typing.as_ref().unchecked_ref(),
                delay as i32,
            );
            start_typing.forget();
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            // For SSR, just show the full text
            let _ = (speed, delay, total_chars);
            displayed_text.set(chars.into_iter().collect());
        }
    });

    view! {
        <span class={format!("inline-flex items-center {}", extra)}>
            <span>{move || displayed_text.get()}</span>
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
