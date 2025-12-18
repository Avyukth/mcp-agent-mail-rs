//! Badge component with CVA-style variants.
//!
//! Provides pill-shaped badges for status indicators and labels.

use leptos::prelude::*;

/// Badge variant styles
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum BadgeVariant {
    /// Primary/default badge (amber)
    #[default]
    Default,
    /// Secondary badge (charcoal)
    Secondary,
    /// Destructive/error badge (red)
    Destructive,
    /// Outline only badge
    Outline,
    /// Success badge (teal/green)
    Success,
    /// Warning badge (amber/yellow)
    Warning,
}

impl BadgeVariant {
    /// Get CSS classes for this variant
    pub fn classes(&self) -> &'static str {
        match self {
            BadgeVariant::Default => {
                "border-transparent bg-amber-500 text-white hover:bg-amber-600"
            }
            BadgeVariant::Secondary => {
                "border-transparent bg-charcoal-100 dark:bg-charcoal-700 text-charcoal-800 dark:text-charcoal-100"
            }
            BadgeVariant::Destructive => {
                "border-transparent bg-red-500 text-white hover:bg-red-600"
            }
            BadgeVariant::Outline => {
                "border-charcoal-300 dark:border-charcoal-600 text-charcoal-700 dark:text-charcoal-300 bg-transparent"
            }
            BadgeVariant::Success => "border-transparent bg-teal-500 text-white hover:bg-teal-600",
            BadgeVariant::Warning => {
                "border-transparent bg-yellow-500 text-charcoal-900 hover:bg-yellow-600"
            }
        }
    }
}

/// Base badge classes
const BADGE_BASE: &str = "inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-amber-500 focus:ring-offset-2";

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
    let final_class = format!(
        "{} {} {}",
        BADGE_BASE,
        variant.classes(),
        class.unwrap_or_default()
    );

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
        assert!(BadgeVariant::Default.classes().contains("amber"));
    }

    #[test]
    fn test_badge_variant_secondary() {
        assert!(BadgeVariant::Secondary.classes().contains("charcoal"));
    }

    #[test]
    fn test_badge_variant_destructive() {
        assert!(BadgeVariant::Destructive.classes().contains("red"));
    }

    #[test]
    fn test_badge_variant_outline() {
        assert!(BadgeVariant::Outline.classes().contains("border"));
        assert!(BadgeVariant::Outline.classes().contains("bg-transparent"));
    }

    #[test]
    fn test_badge_variant_success() {
        assert!(BadgeVariant::Success.classes().contains("teal"));
    }

    #[test]
    fn test_badge_variant_warning() {
        assert!(BadgeVariant::Warning.classes().contains("yellow"));
    }

    #[test]
    fn test_badge_dark_mode_support() {
        assert!(BadgeVariant::Secondary.classes().contains("dark:"));
        assert!(BadgeVariant::Outline.classes().contains("dark:"));
    }
}
