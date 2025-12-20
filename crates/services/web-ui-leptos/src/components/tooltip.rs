//! Tooltip component for contextual information.
//!
//! Follows shadcn/ui pattern with delayed show and keyboard accessibility.
//! Uses CVA patterns from cva.rs for consistency.

use leptos::prelude::*;

// Re-export CVA types for convenience
pub use super::cva::TOOLTIP_CONTENT;

/// Tooltip position relative to trigger.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TooltipSide {
    #[default]
    Top,
    Right,
    Bottom,
    Left,
}

impl TooltipSide {
    pub fn position_class(&self) -> &'static str {
        match self {
            TooltipSide::Top => "bottom-full left-1/2 -translate-x-1/2 mb-2",
            TooltipSide::Right => "left-full top-1/2 -translate-y-1/2 ml-2",
            TooltipSide::Bottom => "top-full left-1/2 -translate-x-1/2 mt-2",
            TooltipSide::Left => "right-full top-1/2 -translate-y-1/2 mr-2",
        }
    }
}

/// Tooltip component with hover/focus triggers.
///
/// # Props
/// - `content`: Tooltip text content
/// - `side`: Position relative to trigger (Top, Right, Bottom, Left)
/// - `delay_ms`: Delay before showing (default 700ms)
/// - `class`: Additional CSS classes
///
/// # Example
/// ```rust,ignore
/// view! {
///     <Tooltip content="More information">
///         <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon>
///             <i data-lucide="info" class="h-4 w-4" />
///         </Button>
///     </Tooltip>
/// }
/// ```
#[component]
pub fn Tooltip(
    /// Tooltip text content
    #[prop(into)]
    content: String,
    /// Position relative to trigger
    #[prop(default = TooltipSide::Top)]
    side: TooltipSide,
    /// Delay before showing in milliseconds
    #[prop(default = 700)]
    delay_ms: u32,
    /// Additional CSS classes for tooltip content
    #[prop(optional, into)]
    class: Option<String>,
    /// Trigger element
    children: Children,
) -> impl IntoView {
    use super::cva::with_class;
    let base_class = with_class(TOOLTIP_CONTENT, class.as_deref());
    let position_class = side.position_class();

    let is_visible = RwSignal::new(false);
    let timeout_handle = StoredValue::new(None::<i32>);

    let show = move || {
        // Set a timeout to show the tooltip
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use wasm_bindgen::closure::Closure;

            let window = web_sys::window().unwrap();
            let closure = Closure::once(Box::new(move || {
                is_visible.set(true);
            }) as Box<dyn FnOnce()>);

            let id = window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    delay_ms as i32,
                )
                .unwrap();

            timeout_handle.set_value(Some(id));
            closure.forget();
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = delay_ms;
            is_visible.set(true);
        }
    };

    let hide = move || {
        // Clear any pending timeout
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(id) = timeout_handle.get_value() {
                if let Some(window) = web_sys::window() {
                    window.clear_timeout_with_handle(id);
                }
            }
            timeout_handle.set_value(None);
        }

        is_visible.set(false);
    };

    view! {
        <div
            class="relative inline-flex"
            on:mouseenter=move |_| show()
            on:mouseleave=move |_| hide()
            on:focusin=move |_| show()
            on:focusout=move |_| hide()
        >
            {children()}
            <div
                class=move || {
                    if is_visible.get() {
                        format!("absolute z-50 {} {} animate-in fade-in-0 zoom-in-95", position_class, base_class)
                    } else {
                        "hidden".to_string()
                    }
                }
                role="tooltip"
            >
                {content.clone()}
            </div>
        </div>
    }
}

/// Simple tooltip using title attribute (native browser tooltip).
/// Use this for simple cases where custom styling isn't needed.
#[component]
pub fn SimpleTooltip(
    /// Tooltip text content
    #[prop(into)]
    content: String,
    /// Trigger element
    children: Children,
) -> impl IntoView {
    view! {
        <span title={content}>
            {children()}
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tooltip_content_has_background() {
        assert!(TOOLTIP_CONTENT.contains("bg-popover"));
    }

    #[test]
    fn test_tooltip_content_has_text_color() {
        assert!(TOOLTIP_CONTENT.contains("text-popover-foreground"));
    }

    #[test]
    fn test_tooltip_content_has_padding() {
        assert!(TOOLTIP_CONTENT.contains("px-3"));
        assert!(TOOLTIP_CONTENT.contains("py-1.5"));
    }

    #[test]
    fn test_tooltip_content_has_shadow() {
        assert!(TOOLTIP_CONTENT.contains("shadow-md"));
    }

    #[test]
    fn test_tooltip_side_position_top() {
        assert!(TooltipSide::Top.position_class().contains("bottom-full"));
    }

    #[test]
    fn test_tooltip_side_position_right() {
        assert!(TooltipSide::Right.position_class().contains("left-full"));
    }

    #[test]
    fn test_tooltip_side_position_bottom() {
        assert!(TooltipSide::Bottom.position_class().contains("top-full"));
    }

    #[test]
    fn test_tooltip_side_position_left() {
        assert!(TooltipSide::Left.position_class().contains("right-full"));
    }
}
