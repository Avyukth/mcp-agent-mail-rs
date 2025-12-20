//! Grid pattern background component.
//!
//! Creates an animated dot or line grid pattern background.

use leptos::prelude::*;

/// Grid pattern type.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum GridType {
    /// Dot pattern grid
    #[default]
    Dots,
    /// Line pattern grid
    Lines,
    /// Small dot pattern
    SmallDots,
}

impl GridType {
    fn pattern_class(&self) -> &'static str {
        match self {
            GridType::Dots => {
                "bg-[radial-gradient(circle,currentColor_1px,transparent_1px)] bg-[length:24px_24px]"
            }
            GridType::Lines => {
                "bg-[linear-gradient(to_right,currentColor_1px,transparent_1px),linear-gradient(to_bottom,currentColor_1px,transparent_1px)] bg-[length:24px_24px]"
            }
            GridType::SmallDots => {
                "bg-[radial-gradient(circle,currentColor_0.5px,transparent_0.5px)] bg-[length:16px_16px]"
            }
        }
    }
}

/// Grid pattern background component.
///
/// # Props
/// - `pattern`: Type of grid pattern (Dots, Lines, SmallDots)
/// - `color`: Grid color (Tailwind color class)
/// - `opacity`: Grid opacity (0.0 to 1.0)
/// - `animated`: Whether to animate the pattern
/// - `class`: Additional CSS classes
///
/// # Example
/// ```rust,ignore
/// view! {
///     <GridPattern pattern=GridType::Dots opacity=0.1>
///         <div class="relative z-10">"Content"</div>
///     </GridPattern>
/// }
/// ```
#[component]
pub fn GridPattern(
    /// Grid pattern type
    #[prop(default = GridType::Dots)]
    pattern: GridType,
    /// Grid color
    #[prop(default = "text-muted-foreground".to_string(), into)]
    color: String,
    /// Grid opacity (0.0 to 1.0)
    #[prop(default = 0.1)]
    opacity: f32,
    /// Whether to animate the pattern
    #[prop(default = false)]
    animated: bool,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
    /// Children content
    children: Children,
) -> impl IntoView {
    let extra = class.unwrap_or_default();
    let pattern_class = pattern.pattern_class();
    let animation = if animated { "animate-grid-pulse" } else { "" };

    let opacity_style = format!("opacity: {};", opacity);

    view! {
        <div class={format!("relative {}", extra)}>
            <div
                class={format!("absolute inset-0 {} {} {}", pattern_class, color, animation)}
                style={opacity_style}
                aria-hidden="true"
            />
            <div class="relative z-10">
                {children()}
            </div>
        </div>
    }
}

/// Gradient mask grid pattern - fades at edges.
#[component]
pub fn GridPatternMasked(
    /// Grid pattern type
    #[prop(default = GridType::Dots)]
    pattern: GridType,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
    /// Children content
    children: Children,
) -> impl IntoView {
    let extra = class.unwrap_or_default();
    let pattern_class = pattern.pattern_class();

    view! {
        <div class={format!("relative overflow-hidden {}", extra)}>
            <div
                class={format!(
                    "absolute inset-0 {} text-muted-foreground/10 \
                     [mask-image:radial-gradient(ellipse_at_center,black_30%,transparent_70%)]",
                    pattern_class
                )}
                aria-hidden="true"
            />
            <div class="relative z-10">
                {children()}
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_type_dots() {
        assert!(GridType::Dots.pattern_class().contains("radial-gradient"));
    }

    #[test]
    fn test_grid_type_lines() {
        assert!(GridType::Lines.pattern_class().contains("linear-gradient"));
    }

    #[test]
    fn test_grid_type_small_dots() {
        assert!(GridType::SmallDots.pattern_class().contains("0.5px"));
    }
}
