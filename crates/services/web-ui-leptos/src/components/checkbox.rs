//! Checkbox component for boolean selection.
//!
//! Follows shadcn/ui pattern with accessibility and indeterminate state support.
//! Uses CVA patterns from cva.rs for consistency.

use leptos::prelude::*;

// Re-export CVA types for convenience
pub use super::cva::CHECKBOX_BASE;

/// Checkbox state for tri-state support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CheckboxState {
    #[default]
    Unchecked,
    Checked,
    Indeterminate,
}

impl CheckboxState {
    pub fn is_checked(&self) -> bool {
        matches!(self, CheckboxState::Checked)
    }

    pub fn is_indeterminate(&self) -> bool {
        matches!(self, CheckboxState::Indeterminate)
    }
}

/// Checkbox component for boolean selection.
///
/// # Props
/// - `checked`: Whether checkbox is checked (reactive)
/// - `on_change`: Callback when checkbox is toggled
/// - `disabled`: Whether checkbox is disabled
/// - `class`: Additional CSS classes
///
/// # Example
/// ```rust,ignore
/// let agreed = RwSignal::new(false);
/// view! {
///     <Checkbox
///         checked=agreed.into()
///         on_change=Callback::new(move |v| agreed.set(v))
///     />
/// }
/// ```
#[component]
pub fn Checkbox(
    /// Whether checkbox is checked
    #[prop(into)]
    checked: Signal<bool>,
    /// Callback when checkbox is toggled
    #[prop(optional)]
    on_change: Option<Callback<bool>>,
    /// Whether checkbox is disabled
    #[prop(into, default = Signal::derive(|| false))]
    disabled: Signal<bool>,
    /// Indeterminate state (overrides checked visual)
    #[prop(into, default = Signal::derive(|| false))]
    indeterminate: Signal<bool>,
    /// ID for the checkbox
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
    let base_class = with_class(CHECKBOX_BASE, class.as_deref());

    // Dynamic classes based on checked/indeterminate state
    let final_class = move || {
        let state_class = if checked.get() || indeterminate.get() {
            "bg-primary border-primary text-primary-foreground"
        } else {
            "border-input"
        };
        format!("{} {}", base_class, state_class)
    };

    view! {
        <button
            type="button"
            role="checkbox"
            id={id}
            name={name}
            class=final_class
            aria-checked=move || {
                if indeterminate.get() {
                    "mixed"
                } else if checked.get() {
                    "true"
                } else {
                    "false"
                }
            }
            disabled=move || disabled.get()
            on:click=move |_| {
                if !disabled.get() {
                    if let Some(cb) = on_change.as_ref() {
                        cb.run(!checked.get());
                    }
                }
            }
        >
            // Check icon
            <svg
                class=move || if checked.get() && !indeterminate.get() { "h-4 w-4" } else { "h-4 w-4 opacity-0" }
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="3"
                stroke-linecap="round"
                stroke-linejoin="round"
            >
                <polyline points="20 6 9 17 4 12" />
            </svg>
            // Indeterminate icon (minus)
            <svg
                class=move || if indeterminate.get() { "h-4 w-4 absolute" } else { "h-4 w-4 opacity-0 absolute" }
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="3"
                stroke-linecap="round"
            >
                <line x1="5" y1="12" x2="19" y2="12" />
            </svg>
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkbox_base_has_dimensions() {
        assert!(CHECKBOX_BASE.contains("h-4"));
        assert!(CHECKBOX_BASE.contains("w-4"));
    }

    #[test]
    fn test_checkbox_base_has_rounded() {
        assert!(CHECKBOX_BASE.contains("rounded-sm"));
    }

    #[test]
    fn test_checkbox_base_has_focus_ring() {
        assert!(CHECKBOX_BASE.contains("focus-visible:ring-2"));
    }

    #[test]
    fn test_checkbox_base_has_border() {
        assert!(CHECKBOX_BASE.contains("border"));
    }

    #[test]
    fn test_checkbox_state_is_checked() {
        assert!(!CheckboxState::Unchecked.is_checked());
        assert!(CheckboxState::Checked.is_checked());
        assert!(!CheckboxState::Indeterminate.is_checked());
    }

    #[test]
    fn test_checkbox_state_is_indeterminate() {
        assert!(!CheckboxState::Unchecked.is_indeterminate());
        assert!(!CheckboxState::Checked.is_indeterminate());
        assert!(CheckboxState::Indeterminate.is_indeterminate());
    }
}
