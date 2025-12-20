//! Switch component for boolean toggle inputs.
//!
//! Follows shadcn/ui pattern with accessibility and animation support.
//! Uses CVA patterns from cva.rs for consistency.

use leptos::prelude::*;

// Re-export CVA types for convenience
pub use super::cva::{SWITCH_BASE, SWITCH_THUMB};

/// Switch component for boolean toggle.
///
/// # Props
/// - `checked`: Whether switch is on (reactive)
/// - `on_change`: Callback when switch is toggled
/// - `disabled`: Whether switch is disabled
/// - `class`: Additional CSS classes
///
/// # Example
/// ```rust,ignore
/// let enabled = RwSignal::new(false);
/// view! {
///     <Switch
///         checked=enabled.into()
///         on_change=Callback::new(move |v| enabled.set(v))
///     />
/// }
/// ```
#[component]
pub fn Switch(
    /// Whether switch is checked/on
    #[prop(into)]
    checked: Signal<bool>,
    /// Callback when switch is toggled
    #[prop(optional)]
    on_change: Option<Callback<bool>>,
    /// Whether switch is disabled
    #[prop(into, default = Signal::derive(|| false))]
    disabled: Signal<bool>,
    /// ID for the switch
    #[prop(optional, into)]
    id: Option<String>,
    /// Name attribute
    #[prop(optional, into)]
    name: Option<String>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    use super::cva::with_class;
    let base_class = with_class(SWITCH_BASE, class.as_deref());

    // Dynamic classes based on checked state
    let final_class = move || {
        let checked_class = if checked.get() {
            "bg-primary"
        } else {
            "bg-input"
        };
        format!("{} {}", base_class, checked_class)
    };

    // Thumb position based on checked state
    let thumb_class = move || {
        let translate = if checked.get() {
            "translate-x-5"
        } else {
            "translate-x-0"
        };
        format!("{} {}", SWITCH_THUMB, translate)
    };

    view! {
        <button
            type="button"
            role="switch"
            id={id}
            name={name}
            class=final_class
            aria-checked=move || if checked.get() { "true" } else { "false" }
            disabled=move || disabled.get()
            on:click=move |_| {
                if !disabled.get() {
                    if let Some(cb) = on_change.as_ref() {
                        cb.run(!checked.get());
                    }
                }
            }
        >
            <span class=thumb_class />
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_base_has_dimensions() {
        assert!(SWITCH_BASE.contains("h-6"));
        assert!(SWITCH_BASE.contains("w-11"));
    }

    #[test]
    fn test_switch_base_has_rounded() {
        assert!(SWITCH_BASE.contains("rounded-full"));
    }

    #[test]
    fn test_switch_base_has_focus_ring() {
        assert!(SWITCH_BASE.contains("focus-visible:ring-2"));
    }

    #[test]
    fn test_switch_thumb_has_dimensions() {
        assert!(SWITCH_THUMB.contains("h-5"));
        assert!(SWITCH_THUMB.contains("w-5"));
    }

    #[test]
    fn test_switch_thumb_has_transition() {
        assert!(SWITCH_THUMB.contains("transition-transform"));
    }
}
