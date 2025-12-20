//! Tabs component for organizing content into panels.
//!
//! Follows shadcn/ui compound component pattern with accessibility support.
//! Uses CVA patterns from cva.rs for consistency.

use leptos::prelude::*;
use std::sync::Arc;

// Re-export CVA types for convenience
pub use super::cva::{TABS_CONTENT, TABS_LIST, TABS_TRIGGER};

/// Tab item definition.
#[derive(Clone, Debug)]
pub struct TabItem {
    /// Unique value identifier
    pub value: String,
    /// Display label
    pub label: String,
    /// Whether tab is disabled
    pub disabled: bool,
}

impl TabItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Context for sharing active tab state.
#[derive(Clone)]
pub struct TabsContext {
    pub active: RwSignal<String>,
    pub set_active: Arc<dyn Fn(String) + Send + Sync>,
}

/// Tabs container component.
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Tabs default_value="tab1">
///         <TabsList>
///             <TabsTrigger value="tab1">"Tab 1"</TabsTrigger>
///             <TabsTrigger value="tab2">"Tab 2"</TabsTrigger>
///         </TabsList>
///         <TabsContent value="tab1">"Content 1"</TabsContent>
///         <TabsContent value="tab2">"Content 2"</TabsContent>
///     </Tabs>
/// }
/// ```
#[component]
pub fn Tabs(
    /// Default active tab value
    #[prop(into)]
    default_value: String,
    /// Callback when active tab changes
    #[prop(optional)]
    on_change: Option<Callback<String>>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
    /// Tab content
    children: Children,
) -> impl IntoView {
    let active = RwSignal::new(default_value);

    let set_active = Arc::new(move |value: String| {
        active.set(value.clone());
        if let Some(cb) = on_change.as_ref() {
            cb.run(value);
        }
    });

    let context = TabsContext { active, set_active };

    provide_context(context);

    let final_class = class.unwrap_or_default();

    view! {
        <div class={format!("w-full {}", final_class)}>
            {children()}
        </div>
    }
}

/// Tabs list container (the row of tab triggers).
#[component]
pub fn TabsList(
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
    /// Tab triggers
    children: Children,
) -> impl IntoView {
    use super::cva::with_class;
    let final_class = with_class(TABS_LIST, class.as_deref());

    view! {
        <div class={final_class} role="tablist">
            {children()}
        </div>
    }
}

/// Individual tab trigger button.
#[component]
pub fn TabsTrigger(
    /// Tab value identifier
    #[prop(into)]
    value: String,
    /// Whether tab is disabled
    #[prop(default = false)]
    disabled: bool,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
    /// Tab label
    children: Children,
) -> impl IntoView {
    use super::cva::with_class;
    let base_class = with_class(TABS_TRIGGER, class.as_deref());

    let context = expect_context::<TabsContext>();
    let value_for_class = value.clone();
    let value_for_aria = value.clone();
    let value_for_click = value.clone();
    let active_signal = context.active;

    let final_class = move || {
        let is_active = active_signal.get() == value_for_class;
        let active_class = if is_active {
            "bg-background text-foreground shadow-sm"
        } else {
            "text-muted-foreground"
        };
        format!("{} {}", base_class, active_class)
    };

    let set_active = context.set_active.clone();

    view! {
        <button
            type="button"
            role="tab"
            class=final_class
            aria-selected=move || if active_signal.get() == value_for_aria { "true" } else { "false" }
            disabled={disabled}
            on:click=move |_| {
                if !disabled {
                    set_active(value_for_click.clone());
                }
            }
        >
            {children()}
        </button>
    }
}

/// Tab content panel.
#[component]
pub fn TabsContent(
    /// Tab value identifier (must match TabsTrigger value)
    #[prop(into)]
    value: String,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
    /// Panel content
    children: ChildrenFn,
) -> impl IntoView {
    use super::cva::with_class;
    let final_class = with_class(TABS_CONTENT, class.as_deref());

    let context = expect_context::<TabsContext>();
    let value_for_class = value.clone();
    let value_for_hidden = value.clone();
    let value_for_children = value.clone();
    let active_signal = context.active;

    view! {
        <div
            role="tabpanel"
            class=move || if active_signal.get() == value_for_class { final_class.clone() } else { "hidden".to_string() }
            hidden=move || active_signal.get() != value_for_hidden
        >
            {move || if active_signal.get() == value_for_children { Some(children()) } else { None }}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tabs_list_has_flex() {
        assert!(TABS_LIST.contains("inline-flex"));
    }

    #[test]
    fn test_tabs_list_has_background() {
        assert!(TABS_LIST.contains("bg-muted"));
    }

    #[test]
    fn test_tabs_trigger_has_padding() {
        assert!(TABS_TRIGGER.contains("px-3"));
        assert!(TABS_TRIGGER.contains("py-1.5"));
    }

    #[test]
    fn test_tabs_trigger_has_transition() {
        assert!(TABS_TRIGGER.contains("transition-all"));
    }

    #[test]
    fn test_tabs_content_has_margin() {
        assert!(TABS_CONTENT.contains("mt-2"));
    }

    #[test]
    fn test_tab_item_new() {
        let tab = TabItem::new("value1", "Label 1");
        assert_eq!(tab.value, "value1");
        assert_eq!(tab.label, "Label 1");
        assert!(!tab.disabled);
    }

    #[test]
    fn test_tab_item_disabled() {
        let tab = TabItem::new("value1", "Label 1").disabled();
        assert!(tab.disabled);
    }
}
