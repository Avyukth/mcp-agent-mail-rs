//! Textarea component for multi-line text input.
//!
//! Follows shadcn/ui pattern with accessibility and validation support.
//! Uses CVA patterns from cva.rs for consistency.

use leptos::prelude::*;

// Re-export CVA types for convenience
pub use super::cva::TEXTAREA_BASE;

/// Textarea component for multi-line text input.
///
/// # Props
/// - `value`: Bound text value (reactive)
/// - `placeholder`: Placeholder text
/// - `disabled`: Whether textarea is disabled
/// - `invalid`: Show invalid state styling
/// - `rows`: Number of visible rows
/// - `max_length`: Maximum character count
/// - `class`: Additional CSS classes
///
/// # Example
/// ```rust,ignore
/// let content = RwSignal::new(String::new());
/// view! {
///     <Textarea
///         value=content
///         placeholder="Enter your message..."
///         rows=4
///     />
/// }
/// ```
#[component]
pub fn Textarea(
    /// Bound text value
    #[prop(into)]
    value: RwSignal<String>,
    /// Placeholder text
    #[prop(optional, into)]
    placeholder: Option<String>,
    /// Whether textarea is disabled (reactive)
    #[prop(into, default = Signal::derive(|| false))]
    disabled: Signal<bool>,
    /// Show invalid state styling
    #[prop(into, default = Signal::derive(|| false))]
    invalid: Signal<bool>,
    /// Number of visible rows
    #[prop(default = 3)]
    rows: u32,
    /// Maximum character count
    #[prop(optional)]
    max_length: Option<u32>,
    /// Show character count
    #[prop(default = false)]
    show_count: bool,
    /// ID for the textarea
    #[prop(optional, into)]
    id: Option<String>,
    /// Name attribute
    #[prop(optional, into)]
    name: Option<String>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    use super::cva::textarea_class;

    // Create a derived signal for invalid state
    let invalid_value = move || invalid.get();
    let final_class = textarea_class(invalid_value(), class.as_deref());

    let char_count = move || value.get().len();

    view! {
        <div class="relative">
            <textarea
                id={id}
                name={name}
                class={final_class}
                placeholder={placeholder}
                disabled=move || disabled.get()
                rows={rows}
                maxlength={max_length}
                aria-invalid=move || if invalid.get() { "true" } else { "false" }
                prop:value=move || value.get()
                on:input=move |ev| {
                    let new_value = event_target_value(&ev);
                    value.set(new_value);
                }
            />
            {if show_count {
                Some(view! {
                    <div class="absolute bottom-2 right-3 text-xs text-muted-foreground">
                        {move || {
                            if let Some(max) = max_length {
                                format!("{}/{}", char_count(), max)
                            } else {
                                format!("{}", char_count())
                            }
                        }}
                    </div>
                })
            } else {
                None
            }}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_textarea_base_has_min_height() {
        assert!(TEXTAREA_BASE.contains("min-h-[80px]"));
    }

    #[test]
    fn test_textarea_base_has_resize() {
        assert!(TEXTAREA_BASE.contains("resize-y"));
    }

    #[test]
    fn test_textarea_base_has_focus_ring() {
        assert!(TEXTAREA_BASE.contains("focus-visible:ring-2"));
    }

    #[test]
    fn test_textarea_base_has_disabled_state() {
        assert!(TEXTAREA_BASE.contains("disabled:cursor-not-allowed"));
    }
}
