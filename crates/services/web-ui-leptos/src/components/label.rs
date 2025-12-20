//! Label component for form fields.
//!
//! Follows shadcn/ui pattern with required indicator support.
//! Uses CVA patterns from cva.rs for consistency.

use leptos::prelude::*;

// Re-export CVA types for convenience
pub use super::cva::LABEL_BASE;

/// Label component for form fields.
///
/// # Props
/// - `for_id`: ID of the associated form element
/// - `required`: Show required indicator (*)
/// - `optional`: Show optional text
/// - `class`: Additional CSS classes
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Label for_id="email" required=true>"Email"</Label>
///     <Input id="email" />
/// }
/// ```
#[component]
pub fn Label(
    /// ID of the associated form element
    #[prop(optional, into)]
    for_id: Option<String>,
    /// Show required indicator (*)
    #[prop(default = false)]
    required: bool,
    /// Show optional text
    #[prop(default = false)]
    optional: bool,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
    /// Label content
    children: Children,
) -> impl IntoView {
    use super::cva::with_class;
    let final_class = with_class(LABEL_BASE, class.as_deref());

    view! {
        <label class={final_class} for={for_id}>
            {children()}
            {if required {
                Some(view! { <span class="text-destructive ml-1">"*"</span> })
            } else {
                None
            }}
            {if optional {
                Some(view! { <span class="text-muted-foreground ml-1 text-xs">"(optional)"</span> })
            } else {
                None
            }}
        </label>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_base_has_text_sm() {
        assert!(LABEL_BASE.contains("text-sm"));
    }

    #[test]
    fn test_label_base_has_font_medium() {
        assert!(LABEL_BASE.contains("font-medium"));
    }

    #[test]
    fn test_label_base_has_peer_disabled() {
        assert!(LABEL_BASE.contains("peer-disabled:cursor-not-allowed"));
    }
}
