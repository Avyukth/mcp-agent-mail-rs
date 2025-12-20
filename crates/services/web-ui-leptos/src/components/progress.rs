//! Progress component for displaying completion status.
//!
//! Follows shadcn/ui pattern with accessibility and animation support.
//! Uses CVA patterns from cva.rs for consistency.

use leptos::prelude::*;

// Re-export CVA types for convenience
pub use super::cva::{PROGRESS_BASE, PROGRESS_INDICATOR};

/// Progress component for displaying completion percentage.
///
/// # Props
/// - `value`: Progress percentage (0.0 to 100.0)
/// - `max`: Maximum value (default 100.0)
/// - `class`: Additional CSS classes
///
/// # Example
/// ```rust,ignore
/// let progress = RwSignal::new(45.0);
/// view! {
///     <Progress value=progress />
/// }
/// ```
#[component]
pub fn Progress(
    /// Progress value (0.0 to max)
    #[prop(into)]
    value: Signal<f64>,
    /// Maximum value (default 100.0)
    #[prop(default = 100.0)]
    max: f64,
    /// Additional CSS classes for container
    #[prop(optional, into)]
    class: Option<String>,
    /// Additional CSS classes for indicator
    #[prop(optional, into)]
    indicator_class: Option<String>,
) -> impl IntoView {
    use super::cva::with_class;
    let final_class = with_class(PROGRESS_BASE, class.as_deref());
    let final_indicator_class = with_class(PROGRESS_INDICATOR, indicator_class.as_deref());

    // Calculate percentage and clamp between 0-100
    let percentage = move || {
        let v = value.get();
        let pct = (v / max) * 100.0;
        pct.clamp(0.0, 100.0)
    };

    view! {
        <div
            class={final_class}
            role="progressbar"
            aria-valuenow=move || value.get() as i32
            aria-valuemin=0
            aria-valuemax=max as i32
        >
            <div
                class={final_indicator_class}
                style=move || format!("width: {}%", percentage())
            />
        </div>
    }
}

/// Indeterminate progress (loading bar that animates continuously).
#[component]
pub fn ProgressIndeterminate(
    /// Additional CSS classes for container
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    use super::cva::with_class;
    let final_class = with_class(PROGRESS_BASE, class.as_deref());

    view! {
        <div
            class={format!("{} overflow-hidden", final_class)}
            role="progressbar"
            aria-busy="true"
        >
            <div class="h-full w-1/3 bg-primary rounded-full animate-progress-indeterminate" />
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_base_has_height() {
        assert!(PROGRESS_BASE.contains("h-4"));
    }

    #[test]
    fn test_progress_base_has_rounded() {
        assert!(PROGRESS_BASE.contains("rounded-full"));
    }

    #[test]
    fn test_progress_base_has_background() {
        assert!(PROGRESS_BASE.contains("bg-secondary"));
    }

    #[test]
    fn test_progress_indicator_has_transition() {
        assert!(PROGRESS_INDICATOR.contains("transition-all"));
    }

    #[test]
    fn test_progress_indicator_has_primary_color() {
        assert!(PROGRESS_INDICATOR.contains("bg-primary"));
    }
}
