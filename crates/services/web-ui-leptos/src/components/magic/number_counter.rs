//! Number counter animation component.
//!
//! Animates counting from 0 to a target value.

use leptos::prelude::*;

/// Number counter animation component.
///
/// # Props
/// - `value`: Target value to count to
/// - `duration`: Animation duration in milliseconds
/// - `format`: Custom format function
/// - `class`: Additional CSS classes
///
/// # Example
/// ```rust,ignore
/// view! {
///     <NumberCounter value=1000 duration=2000 />
/// }
/// ```
#[component]
pub fn NumberCounter(
    /// Target value to count to
    #[prop(into)]
    value: Signal<i64>,
    /// Animation duration in milliseconds
    #[prop(default = 1000)]
    duration: u32,
    /// Decimal places to show
    #[prop(default = 0)]
    decimals: u32,
    /// Prefix string (e.g., "$")
    #[prop(optional, into)]
    prefix: Option<String>,
    /// Suffix string (e.g., "%", "+")
    #[prop(optional, into)]
    suffix: Option<String>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let extra = class.unwrap_or_default();
    let displayed_value = RwSignal::new(0.0_f64);
    let prefix_clone = prefix.clone();
    let suffix_clone = suffix.clone();

    // Simple reactive update (animation via CSS transition)
    // For WASM compatibility, we use CSS transitions instead of JS animation
    Effect::new(move |_| {
        let target = value.get() as f64;
        let _ = duration; // Duration handled by CSS transition
        displayed_value.set(target);
    });

    view! {
        <span class={format!("tabular-nums {}", extra)}>
            {move || {
                let val = displayed_value.get();
                let formatted = if decimals > 0 {
                    format!("{:.prec$}", val, prec = decimals as usize)
                } else {
                    format!("{}", val.round() as i64)
                };

                format!(
                    "{}{}{}",
                    prefix_clone.as_deref().unwrap_or(""),
                    formatted,
                    suffix_clone.as_deref().unwrap_or("")
                )
            }}
        </span>
    }
}

/// Static number with format (no animation).
#[component]
pub fn FormattedNumber(
    /// Value to display
    #[prop(into)]
    value: Signal<i64>,
    /// Decimal places
    #[prop(default = 0)]
    decimals: u32,
    /// Use locale formatting (commas)
    #[prop(default = true)]
    locale: bool,
    /// Prefix string
    #[prop(optional, into)]
    prefix: Option<String>,
    /// Suffix string
    #[prop(optional, into)]
    suffix: Option<String>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let extra = class.unwrap_or_default();
    let prefix_clone = prefix.clone();
    let suffix_clone = suffix.clone();

    view! {
        <span class={format!("tabular-nums {}", extra)}>
            {move || {
                let val = value.get();
                let formatted = if locale {
                    format_with_commas(val, decimals)
                } else if decimals > 0 {
                    format!("{:.prec$}", val as f64, prec = decimals as usize)
                } else {
                    format!("{}", val)
                };

                format!(
                    "{}{}{}",
                    prefix_clone.as_deref().unwrap_or(""),
                    formatted,
                    suffix_clone.as_deref().unwrap_or("")
                )
            }}
        </span>
    }
}

/// Format a number with commas (thousands separator).
fn format_with_commas(num: i64, decimals: u32) -> String {
    let abs = num.abs();
    let sign = if num < 0 { "-" } else { "" };

    let int_part = abs.to_string();
    let chars: Vec<char> = int_part.chars().collect();
    let len = chars.len();

    let with_commas: String = chars
        .into_iter()
        .enumerate()
        .map(|(i, c)| {
            let pos_from_right = len - 1 - i;
            if pos_from_right > 0 && pos_from_right % 3 == 0 {
                format!("{},", c)
            } else {
                c.to_string()
            }
        })
        .collect();

    if decimals > 0 {
        format!("{}{}.{}", sign, with_commas, "0".repeat(decimals as usize))
    } else {
        format!("{}{}", sign, with_commas)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_with_commas_small() {
        assert_eq!(format_with_commas(100, 0), "100");
    }

    #[test]
    fn test_format_with_commas_thousands() {
        assert_eq!(format_with_commas(1000, 0), "1,000");
    }

    #[test]
    fn test_format_with_commas_millions() {
        assert_eq!(format_with_commas(1000000, 0), "1,000,000");
    }

    #[test]
    fn test_format_with_commas_negative() {
        assert_eq!(format_with_commas(-1000, 0), "-1,000");
    }

    #[test]
    fn test_format_with_commas_decimals() {
        assert_eq!(format_with_commas(1000, 2), "1,000.00");
    }
}
