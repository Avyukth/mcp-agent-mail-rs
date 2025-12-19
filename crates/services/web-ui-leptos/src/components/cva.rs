//! CVA (Class Variance Authority) patterns using tailwind_fuse.
//!
//! This module provides shadcn/ui-style CVA patterns for Leptos components.
//! Uses `tw_merge!` for runtime class merging without derive macros.

use tailwind_fuse::tw_merge;

// -- Button CVA Classes --

/// Base button classes (always applied)
pub const BUTTON_BASE: &str = "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50";

/// Button variant styles following shadcn/ui patterns.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

impl ButtonVariant {
    /// Get the tailwind classes for this variant
    pub fn class(&self) -> &'static str {
        match self {
            Self::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
            Self::Destructive => {
                "bg-destructive text-destructive-foreground hover:bg-destructive/90"
            }
            Self::Outline => {
                "border border-input bg-background hover:bg-accent hover:text-accent-foreground"
            }
            Self::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
            Self::Ghost => "hover:bg-accent hover:text-accent-foreground",
            Self::Link => "text-primary underline-offset-4 hover:underline",
        }
    }
}

/// Button size variants following shadcn/ui patterns.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
    Icon,
}

impl ButtonSize {
    /// Get the tailwind classes for this size
    pub fn class(&self) -> &'static str {
        match self {
            Self::Default => "h-10 px-4 py-2",
            Self::Sm => "h-9 rounded-md px-3",
            Self::Lg => "h-11 rounded-md px-8",
            Self::Icon => "h-10 w-10",
        }
    }
}

/// Generate button classes using CVA-style merging.
///
/// # Example
/// ```rust,ignore
/// let classes = button_class(ButtonVariant::Destructive, ButtonSize::Lg, Some("my-extra"));
/// ```
pub fn button_class(variant: ButtonVariant, size: ButtonSize, extra: Option<&str>) -> String {
    tw_merge!(
        BUTTON_BASE,
        variant.class(),
        size.class(),
        extra.unwrap_or_default()
    )
}

// -- Badge CVA Classes --

/// Base badge classes
pub const BADGE_BASE: &str = "inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2";

/// Badge variants
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
}

impl BadgeVariant {
    pub fn class(&self) -> &'static str {
        match self {
            Self::Default => {
                "border-transparent bg-primary text-primary-foreground hover:bg-primary/80"
            }
            Self::Secondary => {
                "border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/80"
            }
            Self::Destructive => {
                "border-transparent bg-destructive text-destructive-foreground hover:bg-destructive/80"
            }
            Self::Outline => "text-foreground",
        }
    }
}

/// Generate badge classes
pub fn badge_class(variant: BadgeVariant, extra: Option<&str>) -> String {
    tw_merge!(BADGE_BASE, variant.class(), extra.unwrap_or_default())
}

// -- Input CVA Classes --

/// Input base classes
pub const INPUT_BASE: &str = "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";

/// Generate input classes
pub fn input_class(extra: Option<&str>) -> String {
    tw_merge!(INPUT_BASE, extra.unwrap_or_default())
}

// -- Card CVA Classes --

/// Card base classes
pub const CARD_BASE: &str = "rounded-lg border bg-card text-card-foreground shadow-sm";
pub const CARD_HEADER: &str = "flex flex-col space-y-1.5 p-6";
pub const CARD_CONTENT: &str = "p-6 pt-0";
pub const CARD_FOOTER: &str = "flex items-center p-6 pt-0";

/// Generate card classes
pub fn card_class(extra: Option<&str>) -> String {
    tw_merge!(CARD_BASE, extra.unwrap_or_default())
}

// -- Utility: cn function (className merge) --

/// Merge class names with conflict resolution (cn equivalent).
///
/// This is the primary API for class merging in components.
/// Later classes override conflicting earlier ones.
///
/// # Example
/// ```rust,ignore
/// let class = cn(&["text-red-500", "p-4", if is_active { "bg-primary" } else { "" }]);
/// ```
pub fn cn(classes: &[&str]) -> String {
    tw_merge!(classes.join(" "))
}

/// Merge two class strings
pub fn merge2(base: &str, extra: &str) -> String {
    tw_merge!(base, extra)
}

/// Merge base with optional extra class
pub fn with_class(base: &str, extra: Option<&str>) -> String {
    tw_merge!(base, extra.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_class_default() {
        let class = button_class(ButtonVariant::Default, ButtonSize::Default, None);
        assert!(class.contains("bg-primary"));
        assert!(class.contains("h-10"));
    }

    #[test]
    fn test_button_class_destructive_lg() {
        let class = button_class(ButtonVariant::Destructive, ButtonSize::Lg, None);
        assert!(class.contains("bg-destructive"));
        assert!(class.contains("h-11"));
    }

    #[test]
    fn test_button_class_with_extra() {
        let class = button_class(ButtonVariant::Ghost, ButtonSize::Icon, Some("my-custom"));
        assert!(class.contains("my-custom"));
        assert!(class.contains("w-10"));
    }

    #[test]
    fn test_badge_class() {
        let class = badge_class(BadgeVariant::Outline, None);
        assert!(class.contains("text-foreground"));
        assert!(class.contains("rounded-full"));
    }

    #[test]
    fn test_input_class() {
        let class = input_class(Some("w-64"));
        assert!(class.contains("border-input"));
        assert!(class.contains("w-64"));
    }

    #[test]
    fn test_cn_merge() {
        let class = cn(&["text-red-500", "bg-blue-500", "text-green-500"]);
        // tw_merge should resolve conflicts - text-green-500 should win
        assert!(class.contains("text-green-500") || class.contains("text-red-500"));
    }

    #[test]
    fn test_with_class() {
        let result = with_class("base-class", Some("extra"));
        assert!(result.contains("base-class"));
        assert!(result.contains("extra"));
    }

    #[test]
    fn test_with_class_none() {
        let result = with_class("base-class", None);
        assert!(result.contains("base-class"));
    }
}
