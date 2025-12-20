//! Badge component with CVA-style variants.
//!
//! Provides pill-shaped badges for status indicators and labels.
//! Uses CVA patterns from cva.rs for consistency.

use leptos::prelude::*;

// Re-export CVA types for convenience
pub use super::cva::{BADGE_BASE, BadgeVariant, badge_class};

/// Badge component with variants.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Badge variant=BadgeVariant::Success>"Active"</Badge>
///     <Badge variant=BadgeVariant::Destructive>"Error"</Badge>
/// }
/// ```
#[component]
pub fn Badge(
    /// Badge variant
    #[prop(default = BadgeVariant::Default)]
    variant: BadgeVariant,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
    /// Badge content
    children: Children,
) -> impl IntoView {
    // Use CVA function for class merging
    let final_class = badge_class(variant, class.as_deref());

    view! {
        <span class={final_class}>
            {children()}
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_base_has_rounded_full() {
        assert!(BADGE_BASE.contains("rounded-full"));
    }

    #[test]
    fn test_badge_base_has_focus_ring() {
        assert!(BADGE_BASE.contains("focus:ring-2"));
    }

    #[test]
    fn test_badge_base_has_pill_padding() {
        assert!(BADGE_BASE.contains("px-2.5"));
        assert!(BADGE_BASE.contains("py-0.5"));
    }

    #[test]
    fn test_badge_variant_default() {
        // Now uses primary (indigo) instead of amber
        assert!(BadgeVariant::Default.class().contains("bg-primary"));
    }

    #[test]
    fn test_badge_variant_secondary() {
        assert!(BadgeVariant::Secondary.class().contains("bg-secondary"));
    }

    #[test]
    fn test_badge_variant_destructive() {
        assert!(BadgeVariant::Destructive.class().contains("bg-destructive"));
    }

    #[test]
    fn test_badge_variant_outline() {
        assert!(BadgeVariant::Outline.class().contains("text-foreground"));
    }

    #[test]
    fn test_badge_variant_success() {
        assert!(BadgeVariant::Success.class().contains("teal"));
    }

    #[test]
    fn test_badge_variant_warning() {
        assert!(BadgeVariant::Warning.class().contains("amber"));
    }

    #[test]
    fn test_badge_class_function() {
        let class = badge_class(BadgeVariant::Success, Some("custom-class"));
        assert!(class.contains("bg-teal-500"));
        assert!(class.contains("custom-class"));
        assert!(class.contains("rounded-full"));
    }
}
