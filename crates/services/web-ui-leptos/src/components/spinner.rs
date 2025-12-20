//! Spinner component for loading states.
//!
//! Provides an accessible loading indicator with size variants.
//! Uses CVA patterns from cva.rs for consistency.

use leptos::prelude::*;

// Re-export CVA types for convenience
pub use super::cva::{SPINNER_BASE, SpinnerSize, spinner_class};

/// Spinner component for loading states.
///
/// # Props
/// - `size`: Spinner size (Sm, Default, Lg)
/// - `class`: Additional CSS classes
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Spinner />
///     <Spinner size=SpinnerSize::Lg />
///     <Spinner size=SpinnerSize::Sm class="text-primary" />
/// }
/// ```
#[component]
pub fn Spinner(
    /// Spinner size variant
    #[prop(default = SpinnerSize::Default)]
    size: SpinnerSize,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
    /// Screen reader label
    #[prop(default = "Loading...".to_string(), into)]
    label: String,
) -> impl IntoView {
    // Use CVA function for class merging
    let final_class = spinner_class(size, class.as_deref());
    let label_clone = label.clone();

    view! {
        <div role="status" aria-label={label}>
            <svg
                class={final_class}
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
            >
                <circle
                    class="opacity-25"
                    cx="12"
                    cy="12"
                    r="10"
                    stroke="currentColor"
                    stroke-width="4"
                />
                <path
                    class="opacity-75"
                    fill="currentColor"
                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                />
            </svg>
            <span class="sr-only">{label_clone}</span>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner_base_has_animate_spin() {
        assert!(SPINNER_BASE.contains("animate-spin"));
    }

    #[test]
    fn test_spinner_size_sm() {
        assert!(SpinnerSize::Sm.class().contains("h-4"));
        assert!(SpinnerSize::Sm.class().contains("w-4"));
    }

    #[test]
    fn test_spinner_size_default() {
        assert!(SpinnerSize::Default.class().contains("h-6"));
        assert!(SpinnerSize::Default.class().contains("w-6"));
    }

    #[test]
    fn test_spinner_size_lg() {
        assert!(SpinnerSize::Lg.class().contains("h-8"));
        assert!(SpinnerSize::Lg.class().contains("w-8"));
    }

    #[test]
    fn test_spinner_class_function() {
        let class = spinner_class(SpinnerSize::Default, Some("text-primary"));
        assert!(class.contains("animate-spin"));
        assert!(class.contains("h-6"));
        assert!(class.contains("text-primary"));
    }
}
