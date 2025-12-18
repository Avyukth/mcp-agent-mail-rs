//! Skeleton component for loading states.
//!
//! Renders an animated pulsing placeholder.

use leptos::prelude::*;

#[component]
pub fn Skeleton(#[prop(optional, into)] class: Option<String>) -> impl IntoView {
    let base_class = "animate-pulse rounded-md bg-cream-200 dark:bg-charcoal-700";
    let final_class = match class {
        Some(c) => format!("{} {}", base_class, c),
        None => base_class.to_string(),
    };

    view! {
        <div class={final_class}></div>
    }
}
