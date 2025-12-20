//! Alert component for displaying important messages.
//!
//! Follows shadcn/ui pattern with compound components.
//! Uses CVA patterns from cva.rs for consistency.

use leptos::prelude::*;

// Re-export CVA types for convenience
pub use super::cva::{ALERT_BASE, ALERT_DESCRIPTION, ALERT_TITLE, AlertVariant, alert_class};

/// Alert container component.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Alert variant=AlertVariant::Destructive>
///         <i data-lucide="alert-circle" class="h-4 w-4"></i>
///         <AlertTitle>"Error"</AlertTitle>
///         <AlertDescription>"Something went wrong"</AlertDescription>
///     </Alert>
/// }
/// ```
#[component]
pub fn Alert(
    #[prop(default = AlertVariant::Default)] variant: AlertVariant,
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    // Use CVA function for class merging
    let final_class = alert_class(variant, class.as_deref());

    view! {
        <div class={final_class} role="alert">
            {children()}
        </div>
    }
}

/// Alert title component.
#[component]
pub fn AlertTitle(
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    use super::cva::with_class;
    let final_class = with_class(ALERT_TITLE, class.as_deref());
    view! {
        <h5 class={final_class}>
            {children()}
        </h5>
    }
}

/// Alert description component.
#[component]
pub fn AlertDescription(
    #[prop(optional, into)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    use super::cva::with_class;
    let final_class = with_class(ALERT_DESCRIPTION, class.as_deref());
    view! {
        <div class={final_class}>
            {children()}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_base_class() {
        assert!(ALERT_BASE.contains("relative"));
        assert!(ALERT_BASE.contains("rounded-lg"));
        assert!(ALERT_BASE.contains("border"));
    }

    #[test]
    fn test_alert_variant_default() {
        let classes = AlertVariant::Default.class();
        assert!(classes.contains("bg-background"));
        assert!(classes.contains("text-foreground"));
    }

    #[test]
    fn test_alert_variant_destructive() {
        let classes = AlertVariant::Destructive.class();
        assert!(classes.contains("text-destructive"));
        assert!(classes.contains("border-destructive"));
    }

    #[test]
    fn test_alert_variant_success() {
        let classes = AlertVariant::Success.class();
        assert!(classes.contains("text-teal-600"));
        assert!(classes.contains("border-teal-500"));
    }

    #[test]
    fn test_alert_variant_warning() {
        let classes = AlertVariant::Warning.class();
        assert!(classes.contains("text-amber-600"));
        assert!(classes.contains("border-amber-500"));
    }

    #[test]
    fn test_alert_title_class() {
        assert!(ALERT_TITLE.contains("font-medium"));
        assert!(ALERT_TITLE.contains("tracking-tight"));
    }

    #[test]
    fn test_alert_description_class() {
        assert!(ALERT_DESCRIPTION.contains("text-sm"));
    }

    #[test]
    fn test_alert_class_function() {
        let class = alert_class(AlertVariant::Destructive, Some("custom-class"));
        assert!(class.contains("text-destructive"));
        assert!(class.contains("custom-class"));
        assert!(class.contains("rounded-lg"));
    }
}
