//! Shadcn-style Select component.
//! Custom dropdown with button trigger and floating options panel.

use leptos::prelude::*;

/// Option for the Select component.
#[derive(Clone, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

impl SelectOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

/// Shadcn-style Select component with full keyboard navigation.
#[component]
pub fn Select(
    /// Unique ID for the select.
    id: String,
    /// Available options.
    options: Vec<SelectOption>,
    /// Current selected value signal.
    value: RwSignal<String>,
    /// Placeholder text when nothing selected.
    placeholder: String,
    /// Whether the select is disabled.
    #[prop(default = false)]
    disabled: bool,
    /// Optional icon name (lucide).
    #[prop(optional)]
    icon: Option<&'static str>,
) -> impl IntoView {
    let is_open = RwSignal::new(false);
    let focused_index = RwSignal::new(-1i32);
    let options_for_display = options.clone();
    let options_for_nav = options.clone();
    let option_count = options.len() as i32;

    // Generate unique IDs
    let listbox_id = format!("{}-listbox", id);
    let listbox_id_clone = listbox_id.clone();

    // Get current label
    let get_label = {
        let options = options.clone();
        let placeholder = placeholder.clone();
        move || {
            let val = value.get();
            if val.is_empty() {
                placeholder.clone()
            } else {
                options
                    .iter()
                    .find(|o| o.value == val)
                    .map(|o| o.label.clone())
                    .unwrap_or(val)
            }
        }
    };

    // Check if placeholder is showing
    let is_placeholder = move || value.get().is_empty();

    // Toggle dropdown
    let toggle = move |_| {
        if !disabled {
            is_open.update(|v| *v = !*v);
        }
    };

    // Select option
    let select_option = move |val: String| {
        value.set(val);
        is_open.set(false);
        focused_index.set(-1);
    };

    // Close on click outside
    let close_dropdown = move |_| {
        is_open.set(false);
        focused_index.set(-1);
    };

    // Keyboard navigation handler
    let handle_keydown = {
        let options = options_for_nav.clone();
        move |ev: web_sys::KeyboardEvent| {
            let key = ev.key();
            match key.as_str() {
                "Escape" => {
                    is_open.set(false);
                    focused_index.set(-1);
                }
                "ArrowDown" => {
                    ev.prevent_default();
                    if !is_open.get() {
                        is_open.set(true);
                    }
                    focused_index.update(|i| {
                        *i = (*i + 1).min(option_count - 1);
                    });
                }
                "ArrowUp" => {
                    ev.prevent_default();
                    focused_index.update(|i| {
                        *i = (*i - 1).max(0);
                    });
                }
                "Home" => {
                    ev.prevent_default();
                    focused_index.set(0);
                }
                "End" => {
                    ev.prevent_default();
                    focused_index.set(option_count - 1);
                }
                "Enter" | " " => {
                    ev.prevent_default();
                    if is_open.get() {
                        let idx = focused_index.get();
                        if idx >= 0 && (idx as usize) < options.len() {
                            value.set(options[idx as usize].value.clone());
                            is_open.set(false);
                            focused_index.set(-1);
                        }
                    } else {
                        is_open.set(true);
                    }
                }
                _ => {
                    // Type-ahead: find first option starting with typed character
                    if key.len() == 1 && is_open.get() {
                        let char_lower = key.to_lowercase();
                        for (i, opt) in options.iter().enumerate() {
                            if opt.label.to_lowercase().starts_with(&char_lower) {
                                focused_index.set(i as i32);
                                break;
                            }
                        }
                    }
                }
            }
        }
    };

    view! {
        <div class="relative">
            // Trigger Button
            <button
                type="button"
                id=id.clone()
                role="combobox"
                aria-expanded=move || is_open.get()
                aria-haspopup="listbox"
                aria-controls=listbox_id_clone.clone()
                aria-activedescendant={
                    let id_for_aria = id.clone();
                    move || {
                        if focused_index.get() >= 0 {
                            format!("{}-option-{}", id_for_aria, focused_index.get())
                        } else {
                            String::new()
                        }
                    }
                }
                disabled=disabled
                on:click=toggle
                on:keydown=handle_keydown
                class=move || {
                    let base = "flex h-10 w-full items-center justify-between gap-2 whitespace-nowrap rounded-lg border bg-white dark:bg-charcoal-800 px-3 py-2 text-sm shadow-sm ring-offset-white transition-colors focus:outline-none focus:ring-2 focus:ring-amber-500 focus:ring-offset-2";
                    let state = if disabled {
                        "cursor-not-allowed opacity-50 border-cream-200 dark:border-charcoal-700"
                    } else if is_open.get() {
                        "border-amber-400 ring-2 ring-amber-400/20"
                    } else {
                        "border-cream-200 dark:border-charcoal-700 hover:border-amber-300 dark:hover:border-amber-700"
                    };
                    format!("{} {}", base, state)
                }
            >
                <span class=move || {
                    if is_placeholder() {
                        "text-charcoal-400 dark:text-charcoal-500 flex items-center gap-2"
                    } else {
                        "text-charcoal-800 dark:text-cream-100 flex items-center gap-2"
                    }
                }>
                    {icon.map(|icon| view! {
                        <i data-lucide=icon class="icon-sm text-charcoal-400"></i>
                    })}
                    {get_label}
                </span>
                <i
                    data-lucide="chevron-down"
                    class=move || {
                        let base = "icon-sm text-charcoal-400 transition-transform duration-200";
                        if is_open.get() {
                            format!("{} rotate-180", base)
                        } else {
                            base.to_string()
                        }
                    }
                ></i>
            </button>

            // Dropdown Panel
            {move || {
                if is_open.get() {
                    let opts = options_for_display.clone();
                    let current_value = value.get();
                    let current_focus = focused_index.get();
                    let listbox_id = listbox_id.clone();
                    let id_for_opts = id.clone();
                    Some(view! {
                        // Backdrop for click-outside
                        <div
                            class="fixed inset-0 z-40"
                            on:click=close_dropdown
                        ></div>

                        // Options Panel
                        <div
                            id=listbox_id
                            role="listbox"
                            class="absolute z-50 mt-1 w-full min-w-[8rem] overflow-hidden rounded-lg border border-cream-200 dark:border-charcoal-700 bg-white dark:bg-charcoal-800 shadow-lg animate-slide-up"
                        >
                            <div class="max-h-60 overflow-auto py-1">
                                {opts.into_iter().enumerate().map(|(i, opt)| {
                                    let val = opt.value.clone();
                                    let label = opt.label.clone();
                                    let is_selected = val == current_value;
                                    let is_focused = i as i32 == current_focus;
                                    let select = select_option;
                                    let val_clone = val.clone();
                                    let option_id = format!("{}-option-{}", id_for_opts, i);

                                    view! {
                                        <button
                                            type="button"
                                            id=option_id
                                            role="option"
                                            aria-selected=is_selected
                                            on:click=move |_| select(val_clone.clone())
                                            class=move || {
                                                let base = "relative flex w-full cursor-pointer items-center px-3 py-2 text-sm outline-none transition-colors";
                                                if is_selected {
                                                    format!("{} bg-amber-50 dark:bg-amber-900/20 text-amber-700 dark:text-amber-300", base)
                                                } else if is_focused {
                                                    format!("{} bg-cream-100 dark:bg-charcoal-700 text-charcoal-800 dark:text-cream-100", base)
                                                } else {
                                                    format!("{} text-charcoal-700 dark:text-charcoal-300 hover:bg-cream-100 dark:hover:bg-charcoal-700", base)
                                                }
                                            }
                                        >
                                            <span class="flex-1 text-left">{label}</span>
                                            {if is_selected {
                                                Some(view! {
                                                    <i data-lucide="check" class="icon-sm text-amber-600 dark:text-amber-400 ml-2"></i>
                                                })
                                            } else {
                                                None
                                            }}
                                        </button>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_option_new() {
        let opt = SelectOption::new("val", "Label");
        assert_eq!(opt.value, "val");
        assert_eq!(opt.label, "Label");
    }

    #[test]
    fn test_listbox_id_format() {
        let id = "my-select";
        let listbox_id = format!("{}-listbox", id);
        assert_eq!(listbox_id, "my-select-listbox");
    }

    #[test]
    fn test_option_id_format() {
        let id = "my-select";
        let option_id = format!("{}-option-{}", id, 0);
        assert_eq!(option_id, "my-select-option-0");
    }
}
