//! Alert component for displaying important messages.
//!
//! Follows shadcn/ui pattern with compound components.

use leptos::prelude::*;

const ALERT_BASE: &str = "relative w-full rounded-lg border p-4 [&>svg~*]:pl-7 [&>svg+div]:translate-y-[-3px] [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg]:text-foreground";
const TITLE_CLASS: &str = "mb-1 font-medium leading-none tracking-tight";
const DESCRIPTION_CLASS: &str = "text-sm [&_p]:leading-relaxed";

/// Alert variant styles
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum AlertVariant {
    /// Default style (background/foreground)
    #[default]
    Default,
    /// Destructive style (red)
    Destructive,
    /// Success style (green/emerald)
    Success,
    /// Warning style (amber)
    Warning,
}

impl AlertVariant {
    pub fn classes(&self) -> &'static str {
        match self {
            AlertVariant::Default => {
                "bg-white dark:bg-charcoal-800 text-charcoal-900 dark:text-cream-50 border-charcoal-200 dark:border-charcoal-700"
            }
            AlertVariant::Destructive => {
                "border-red-500/50 text-red-600 dark:border-red-500 [&>svg]:text-red-600 dark:text-red-500"
            }
            AlertVariant::Success => {
                "border-teal-500/50 text-teal-600 dark:border-teal-500 [&>svg]:text-teal-600 dark:text-teal-500"
            }
            AlertVariant::Warning => {
                "border-amber-500/50 text-amber-600 dark:border-amber-500 [&>svg]:text-amber-600 dark:text-amber-500"
            }
        }
    }
}

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
    let final_class = format!(
        "{} {} {}",
        ALERT_BASE,
        variant.classes(),
        class.unwrap_or_default()
    );

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
    let final_class = format!("{} {}", TITLE_CLASS, class.unwrap_or_default());
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
    let final_class = format!("{} {}", DESCRIPTION_CLASS, class.unwrap_or_default());
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
        let classes = AlertVariant::Default.classes();
        assert!(classes.contains("bg-white"));
        assert!(classes.contains("text-charcoal-900"));
    }

    #[test]
    fn test_alert_variant_destructive() {
        let classes = AlertVariant::Destructive.classes();
        assert!(classes.contains("text-red-600"));
        assert!(classes.contains("border-red-500/50"));
    }

    #[test]
    fn test_alert_title_class() {
        assert!(TITLE_CLASS.contains("font-medium"));
        assert!(TITLE_CLASS.contains("tracking-tight"));
    }

    #[test]
    fn test_alert_description_class() {
        assert!(DESCRIPTION_CLASS.contains("text-sm"));
    }
}
