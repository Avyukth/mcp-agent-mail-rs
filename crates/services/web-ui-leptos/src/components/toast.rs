//! Toast notification component with accessible stacking.
//!
//! Follows shadcn/ui toast patterns with mobile-first design.
//! Features: auto-dismiss, variant styling, stacking (max 3), reduced motion support.

use leptos::prelude::*;
use tailwind_fuse::tw_merge;

/// Maximum number of visible toasts at once.
pub const MAX_VISIBLE_TOASTS: usize = 3;

/// Default auto-dismiss duration in milliseconds.
pub const DEFAULT_DURATION_MS: u64 = 5000;

// ============================================================================
// Toast Variant
// ============================================================================

/// Toast visual variants following shadcn/ui patterns.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ToastVariant {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

impl ToastVariant {
    /// Get the tailwind classes for this variant.
    pub fn class(&self) -> &'static str {
        match self {
            Self::Info => "bg-primary text-primary-foreground border-primary/50",
            Self::Success => "bg-emerald-500 text-white border-emerald-600",
            Self::Warning => "bg-amber-500 text-white border-amber-600",
            Self::Error => "bg-destructive text-destructive-foreground border-destructive/50",
        }
    }

    /// Get the icon name for this variant (lucide icon).
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Success => "check-circle",
            Self::Warning => "alert-triangle",
            Self::Error => "x-circle",
        }
    }

    /// Get the ARIA role for this variant.
    /// Error toasts use "alert" for immediate announcement.
    /// Others use "status" for polite announcements.
    pub fn role(&self) -> &'static str {
        match self {
            Self::Error => "alert",
            _ => "status",
        }
    }

    /// Get aria-live value for this variant.
    pub fn aria_live(&self) -> &'static str {
        match self {
            Self::Error => "assertive",
            _ => "polite",
        }
    }
}

// ============================================================================
// Toast Data
// ============================================================================

/// A single toast notification.
#[derive(Debug, Clone, PartialEq)]
pub struct Toast {
    /// Unique identifier for this toast.
    pub id: u64,
    /// Toast message content.
    pub message: String,
    /// Optional title above the message.
    pub title: Option<String>,
    /// Visual variant (info, success, warning, error).
    pub variant: ToastVariant,
    /// Auto-dismiss duration in ms (0 = no auto-dismiss).
    pub duration_ms: u64,
}

impl Toast {
    /// Create a new toast with the given message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            id: 0, // Will be set by Toaster
            message: message.into(),
            title: None,
            variant: ToastVariant::Info,
            duration_ms: DEFAULT_DURATION_MS,
        }
    }

    /// Set the toast variant.
    pub fn variant(mut self, variant: ToastVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the toast title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set custom duration (0 = no auto-dismiss).
    pub fn duration(mut self, ms: u64) -> Self {
        self.duration_ms = ms;
        self
    }

    /// Create a success toast.
    pub fn success(message: impl Into<String>) -> Self {
        Self::new(message).variant(ToastVariant::Success)
    }

    /// Create an error toast.
    pub fn error(message: impl Into<String>) -> Self {
        Self::new(message).variant(ToastVariant::Error)
    }

    /// Create a warning toast.
    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(message).variant(ToastVariant::Warning)
    }

    /// Create an info toast.
    pub fn info(message: impl Into<String>) -> Self {
        Self::new(message).variant(ToastVariant::Info)
    }
}

// ============================================================================
// Toaster Context
// ============================================================================

/// Context for managing toast notifications globally.
#[derive(Clone, Copy)]
pub struct ToasterContext {
    /// Current toasts (reactive list).
    toasts: RwSignal<Vec<Toast>>,
    /// Counter for generating unique IDs.
    next_id: RwSignal<u64>,
}

impl ToasterContext {
    /// Create a new toaster context.
    fn new() -> Self {
        Self {
            toasts: RwSignal::new(Vec::new()),
            next_id: RwSignal::new(1),
        }
    }

    /// Show a new toast notification.
    pub fn toast(&self, mut toast: Toast) {
        // Assign unique ID
        let id = self.next_id.get();
        self.next_id.set(id + 1);
        toast.id = id;

        // Add to stack (limiting visible count)
        self.toasts.update(|toasts| {
            // Remove oldest if at capacity
            while toasts.len() >= MAX_VISIBLE_TOASTS {
                toasts.remove(0);
            }
            toasts.push(toast);
        });
    }

    /// Dismiss a specific toast by ID.
    pub fn dismiss(&self, id: u64) {
        self.toasts.update(|toasts| {
            toasts.retain(|t| t.id != id);
        });
    }

    /// Dismiss all toasts.
    pub fn dismiss_all(&self) {
        self.toasts.set(Vec::new());
    }

    /// Get the current toast count.
    pub fn count(&self) -> usize {
        self.toasts.get().len()
    }
}

/// Hook to access the toaster context.
/// Panics if used outside of a Toaster provider.
#[allow(clippy::expect_used)]
pub fn use_toaster() -> ToasterContext {
    use_context::<ToasterContext>().expect("use_toaster must be used within a Toaster component")
}

// ============================================================================
// Toaster Provider Component
// ============================================================================

/// CSS classes for toast container positioning.
const CONTAINER_BASE: &str = "fixed z-[100] flex flex-col gap-2 p-4 pointer-events-none";

/// Desktop: bottom-right; Mobile: bottom-center.
const CONTAINER_POSITION: &str =
    "bottom-0 right-0 sm:right-4 left-0 sm:left-auto items-center sm:items-end";

/// CSS classes for individual toast.
const TOAST_BASE: &str = "pointer-events-auto flex w-full max-w-sm items-start gap-3 rounded-lg border p-4 shadow-lg transition-all duration-200 motion-reduce:transition-none";

/// Animation classes for entering toasts.
const TOAST_ENTER: &str = "animate-in fade-in-0 slide-in-from-bottom-2";

/// Toaster component that provides toast context and renders toast stack.
#[component]
pub fn Toaster(children: Children) -> impl IntoView {
    let ctx = ToasterContext::new();
    provide_context(ctx);

    let container_class = tw_merge!(CONTAINER_BASE, CONTAINER_POSITION);

    view! {
        {children()}

        // Toast container portal
        <div class={container_class}>
            <For
                each=move || ctx.toasts.get()
                key=|toast| toast.id
                children=move |toast| {
                    view! { <ToastItem toast=toast ctx=ctx /> }
                }
            />
        </div>
    }
}

// ============================================================================
// Toast Item Component
// ============================================================================

/// Individual toast notification item.
#[component]
fn ToastItem(toast: Toast, ctx: ToasterContext) -> impl IntoView {
    let id = toast.id;

    // Auto-dismiss effect (WASM only - uses gloo-timers)
    #[cfg(target_arch = "wasm32")]
    {
        let duration_ms = toast.duration_ms;
        Effect::new(move |_| {
            if duration_ms > 0 {
                use gloo_timers::callback::Timeout;
                let timeout = Timeout::new(duration_ms as u32, move || {
                    ctx.dismiss(id);
                });
                timeout.forget(); // Let it run to completion
            }
        });
    }

    let toast_class = tw_merge!(TOAST_BASE, TOAST_ENTER, toast.variant.class());
    let role = toast.variant.role();
    let aria_live = toast.variant.aria_live();
    let icon = toast.variant.icon();

    view! {
        <div
            class={toast_class}
            role={role}
            aria-live={aria_live}
            aria-atomic="true"
        >
            // Icon
            <i data-lucide={icon} class="h-5 w-5 shrink-0 mt-0.5"></i>

            // Content
            <div class="flex-1 space-y-1">
                {toast.title.map(|title| view! {
                    <p class="text-sm font-semibold">{title}</p>
                })}
                <p class="text-sm opacity-90">{toast.message.clone()}</p>
            </div>

            // Close button
            <button
                type="button"
                class="shrink-0 rounded-md p-1 opacity-70 hover:opacity-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                on:click=move |_| ctx.dismiss(id)
                aria-label="Close notification"
            >
                <i data-lucide="x" class="h-4 w-4"></i>
            </button>
        </div>
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // === Constants ===

    #[test]
    fn test_max_visible_toasts() {
        assert_eq!(MAX_VISIBLE_TOASTS, 3);
    }

    #[test]
    fn test_default_duration() {
        assert_eq!(DEFAULT_DURATION_MS, 5000);
    }

    // === ToastVariant ===

    #[test]
    fn test_variant_default_is_info() {
        assert_eq!(ToastVariant::default(), ToastVariant::Info);
    }

    #[test]
    fn test_variant_info_class() {
        let class = ToastVariant::Info.class();
        assert!(class.contains("bg-primary"));
        assert!(class.contains("text-primary-foreground"));
    }

    #[test]
    fn test_variant_success_class() {
        let class = ToastVariant::Success.class();
        assert!(class.contains("bg-emerald-500"));
        assert!(class.contains("text-white"));
    }

    #[test]
    fn test_variant_warning_class() {
        let class = ToastVariant::Warning.class();
        assert!(class.contains("bg-amber-500"));
        assert!(class.contains("text-white"));
    }

    #[test]
    fn test_variant_error_class() {
        let class = ToastVariant::Error.class();
        assert!(class.contains("bg-destructive"));
        assert!(class.contains("text-destructive-foreground"));
    }

    #[test]
    fn test_variant_info_icon() {
        assert_eq!(ToastVariant::Info.icon(), "info");
    }

    #[test]
    fn test_variant_success_icon() {
        assert_eq!(ToastVariant::Success.icon(), "check-circle");
    }

    #[test]
    fn test_variant_warning_icon() {
        assert_eq!(ToastVariant::Warning.icon(), "alert-triangle");
    }

    #[test]
    fn test_variant_error_icon() {
        assert_eq!(ToastVariant::Error.icon(), "x-circle");
    }

    // === Accessibility Roles ===

    #[test]
    fn test_error_uses_alert_role() {
        assert_eq!(ToastVariant::Error.role(), "alert");
    }

    #[test]
    fn test_info_uses_status_role() {
        assert_eq!(ToastVariant::Info.role(), "status");
    }

    #[test]
    fn test_success_uses_status_role() {
        assert_eq!(ToastVariant::Success.role(), "status");
    }

    #[test]
    fn test_warning_uses_status_role() {
        assert_eq!(ToastVariant::Warning.role(), "status");
    }

    #[test]
    fn test_error_uses_assertive_aria_live() {
        assert_eq!(ToastVariant::Error.aria_live(), "assertive");
    }

    #[test]
    fn test_non_error_uses_polite_aria_live() {
        assert_eq!(ToastVariant::Info.aria_live(), "polite");
        assert_eq!(ToastVariant::Success.aria_live(), "polite");
        assert_eq!(ToastVariant::Warning.aria_live(), "polite");
    }

    // === Toast Builder ===

    #[test]
    fn test_toast_new() {
        let toast = Toast::new("Hello");
        assert_eq!(toast.message, "Hello");
        assert_eq!(toast.variant, ToastVariant::Info);
        assert_eq!(toast.duration_ms, DEFAULT_DURATION_MS);
        assert!(toast.title.is_none());
    }

    #[test]
    fn test_toast_with_variant() {
        let toast = Toast::new("Error occurred").variant(ToastVariant::Error);
        assert_eq!(toast.variant, ToastVariant::Error);
    }

    #[test]
    fn test_toast_with_title() {
        let toast = Toast::new("Details here").title("Alert");
        assert_eq!(toast.title, Some("Alert".to_string()));
    }

    #[test]
    fn test_toast_with_duration() {
        let toast = Toast::new("Quick message").duration(2000);
        assert_eq!(toast.duration_ms, 2000);
    }

    #[test]
    fn test_toast_no_auto_dismiss() {
        let toast = Toast::new("Persistent").duration(0);
        assert_eq!(toast.duration_ms, 0);
    }

    #[test]
    fn test_toast_success_helper() {
        let toast = Toast::success("Saved!");
        assert_eq!(toast.variant, ToastVariant::Success);
        assert_eq!(toast.message, "Saved!");
    }

    #[test]
    fn test_toast_error_helper() {
        let toast = Toast::error("Failed!");
        assert_eq!(toast.variant, ToastVariant::Error);
        assert_eq!(toast.message, "Failed!");
    }

    #[test]
    fn test_toast_warning_helper() {
        let toast = Toast::warning("Watch out!");
        assert_eq!(toast.variant, ToastVariant::Warning);
        assert_eq!(toast.message, "Watch out!");
    }

    #[test]
    fn test_toast_info_helper() {
        let toast = Toast::info("FYI");
        assert_eq!(toast.variant, ToastVariant::Info);
        assert_eq!(toast.message, "FYI");
    }

    #[test]
    fn test_toast_builder_chain() {
        let toast = Toast::new("Message")
            .variant(ToastVariant::Success)
            .title("Title")
            .duration(3000);

        assert_eq!(toast.message, "Message");
        assert_eq!(toast.variant, ToastVariant::Success);
        assert_eq!(toast.title, Some("Title".to_string()));
        assert_eq!(toast.duration_ms, 3000);
    }

    // === Toast Equality ===

    #[test]
    fn test_toast_partial_eq() {
        let a = Toast::new("Hello");
        let b = Toast::new("Hello");
        // Different IDs would make them different in practice
        assert_eq!(a.message, b.message);
    }

    #[test]
    fn test_toast_clone() {
        let original = Toast::new("Test")
            .title("Title")
            .variant(ToastVariant::Warning);
        let cloned = original.clone();
        assert_eq!(original.message, cloned.message);
        assert_eq!(original.title, cloned.title);
        assert_eq!(original.variant, cloned.variant);
    }

    // === CSS Classes ===

    #[test]
    fn test_container_has_fixed_position() {
        assert!(CONTAINER_BASE.contains("fixed"));
        assert!(CONTAINER_BASE.contains("z-[100]"));
    }

    #[test]
    fn test_container_has_responsive_positioning() {
        assert!(CONTAINER_POSITION.contains("bottom-0"));
        assert!(CONTAINER_POSITION.contains("sm:right-4"));
        assert!(CONTAINER_POSITION.contains("sm:items-end"));
    }

    #[test]
    fn test_toast_base_has_shadow() {
        assert!(TOAST_BASE.contains("shadow-lg"));
    }

    #[test]
    fn test_toast_base_has_rounded() {
        assert!(TOAST_BASE.contains("rounded-lg"));
    }

    #[test]
    fn test_toast_base_has_pointer_events() {
        assert!(TOAST_BASE.contains("pointer-events-auto"));
    }

    #[test]
    fn test_toast_has_reduced_motion_support() {
        assert!(TOAST_BASE.contains("motion-reduce:transition-none"));
    }

    #[test]
    fn test_toast_enter_animation() {
        assert!(TOAST_ENTER.contains("animate-in"));
        assert!(TOAST_ENTER.contains("fade-in"));
        assert!(TOAST_ENTER.contains("slide-in-from-bottom"));
    }

    // === Variant is Copy ===

    #[test]
    fn test_variant_is_copy() {
        fn assert_copy<T: Copy>() {}
        assert_copy::<ToastVariant>();
    }

    #[test]
    fn test_variant_is_clone() {
        fn assert_clone<T: Clone>() {}
        assert_clone::<ToastVariant>();
    }

    // === Touch Target ===

    #[test]
    fn test_toast_has_adequate_padding() {
        // p-4 = 1rem = 16px padding, adequate for touch
        assert!(TOAST_BASE.contains("p-4"));
    }
}
