//! Unified Inbox page - Gmail-style view of ALL messages across ALL projects.
//! Uses the /mail/api/unified-inbox API from s0j task.

use crate::api::client::{self, UnifiedInboxMessage};
use crate::components::{Select, SelectOption};
use leptos::prelude::*;

/// Unified Inbox page component.
#[component]
pub fn UnifiedInbox() -> impl IntoView {
    // State
    let messages = RwSignal::new(Vec::<UnifiedInboxMessage>::new());
    let loading = RwSignal::new(true);
    let error = RwSignal::new(Option::<String>::None);
    let importance_filter = RwSignal::new(String::new()); // "", "high", "normal"

    // Load messages effect
    Effect::new(move |_| {
        let importance = importance_filter.get();

        leptos::task::spawn_local(async move {
            loading.set(true);
            error.set(None);

            let imp = if importance.is_empty() {
                None
            } else {
                Some(importance.as_str())
            };

            match client::get_unified_inbox(imp, Some(100)).await {
                Ok(msgs) => {
                    messages.set(msgs);
                    loading.set(false);
                }
                Err(e) => {
                    error.set(Some(e.message));
                    loading.set(false);
                }
            }
        });
    });

    // Importance filter options
    let importance_options = vec![
        SelectOption {
            value: "".to_string(),
            label: "All Messages".to_string(),
        },
        SelectOption {
            value: "high".to_string(),
            label: "High Priority".to_string(),
        },
        SelectOption {
            value: "normal".to_string(),
            label: "Normal Priority".to_string(),
        },
    ];

    view! {
        <div class="space-y-6">
            // Header
            <div class="flex items-center justify-between">
                <div>
                    <h1 class="font-display text-2xl font-bold text-charcoal-800 dark:text-cream-100 flex items-center gap-3">
                        <i data-lucide="inbox" class="icon-xl text-amber-500"></i>
                        "Unified Inbox"
                    </h1>
                    <p class="text-sm text-charcoal-500 dark:text-charcoal-400 mt-1">
                        "All messages across all projects"
                    </p>
                </div>

                // Filter
                <div class="flex items-center gap-3">
                    <Select
                        id="importance-filter".to_string()
                        value=importance_filter
                        options=importance_options
                        placeholder="Filter by importance".to_string()
                    />
                </div>
            </div>

            // Error
            {move || {
                error.get().map(|e| view! {
                    <div class="rounded-xl border border-red-200 dark:border-red-800 bg-red-50 dark:bg-red-900/20 p-4">
                        <div class="flex items-start gap-3">
                            <i data-lucide="triangle-alert" class="icon-lg text-red-500"></i>
                            <p class="text-red-700 dark:text-red-400">{e}</p>
                        </div>
                    </div>
                })
            }}

            // Loading
            {move || {
                if loading.get() {
                    Some(view! {
                        <div class="flex items-center justify-center py-16">
                            <div class="flex flex-col items-center gap-4">
                                <i data-lucide="loader-2" class="icon-2xl text-amber-500 animate-spin"></i>
                                <p class="text-charcoal-500 dark:text-charcoal-400 text-sm">"Loading messages..."</p>
                            </div>
                        </div>
                    })
                } else {
                    None
                }
            }}

            // Messages List
            {move || {
                let msgs = messages.get();
                if !loading.get() && msgs.is_empty() {
                    Some(view! {
                        <div class="card-elevated p-12 text-center">
                            <div class="inline-flex items-center justify-center w-16 h-16 rounded-2xl bg-cream-200 dark:bg-charcoal-700 mb-6">
                                <i data-lucide="inbox" class="icon-2xl text-charcoal-400"></i>
                            </div>
                            <h3 class="font-display text-xl font-semibold text-charcoal-800 dark:text-cream-100 mb-2">"No messages"</h3>
                            <p class="text-charcoal-500 dark:text-charcoal-400">
                                "Your unified inbox is empty."
                            </p>
                        </div>
                    }.into_any())
                } else if !msgs.is_empty() {
                    Some(view! {
                        <div class="space-y-3">
                            {msgs.into_iter().map(|msg| {
                                let id = msg.id;
                                let subject = msg.subject.clone();
                                let sender = msg.sender_name.clone();
                                let importance = msg.importance.clone();
                                let created = msg.created_ts.clone();
                                let thread_id = msg.thread_id.clone();

                                view! {
                                    <a
                                        href={format!("/inbox/{}?project={}", id, msg.project_id)}
                                        class="block card-elevated p-4 hover:shadow-lg transition-all duration-200 group"
                                    >
                                        <div class="flex items-start gap-4">
                                            // Icon
                                            <div class="flex-shrink-0">
                                                <div class={format!(
                                                    "w-10 h-10 rounded-xl flex items-center justify-center {}",
                                                    if importance == "high" {
                                                        "bg-red-100 dark:bg-red-900/30"
                                                    } else {
                                                        "bg-cream-200 dark:bg-charcoal-700"
                                                    }
                                                )}>
                                                    <i data-lucide="mail" class={format!(
                                                        "icon-lg {}",
                                                        if importance == "high" {
                                                            "text-red-500"
                                                        } else {
                                                            "text-charcoal-400"
                                                        }
                                                    )}></i>
                                                </div>
                                            </div>

                                            // Content
                                            <div class="flex-1 min-w-0">
                                                <div class="flex items-center justify-between gap-2 mb-1">
                                                    <span class="font-medium text-charcoal-800 dark:text-cream-100 truncate group-hover:text-amber-600">
                                                        {subject}
                                                    </span>
                                                    {if importance == "high" {
                                                        Some(view! {
                                                            <span class="badge badge-red flex-shrink-0">
                                                                <i data-lucide="alert-circle" class="icon-xs"></i>
                                                                "High"
                                                            </span>
                                                        })
                                                    } else {
                                                        None
                                                    }}
                                                </div>
                                                <div class="flex items-center gap-3 text-sm text-charcoal-500 dark:text-charcoal-400">
                                                    <span class="flex items-center gap-1">
                                                        <i data-lucide="user" class="icon-xs"></i>
                                                        {sender}
                                                    </span>
                                                    <span class="flex items-center gap-1">
                                                        <i data-lucide="calendar" class="icon-xs"></i>
                                                        {format_date(&created)}
                                                    </span>
                                                    {thread_id.map(|tid| view! {
                                                        <span class="flex items-center gap-1">
                                                            <i data-lucide="git-branch" class="icon-xs"></i>
                                                            {tid}
                                                        </span>
                                                    })}
                                                </div>
                                            </div>

                                            // Arrow
                                            <i data-lucide="chevron-right" class="icon-lg text-charcoal-300 group-hover:text-amber-500 transition-colors"></i>
                                        </div>
                                    </a>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_any())
                } else {
                    None
                }
            }}
        </div>
    }
}

fn format_date(date_str: &str) -> String {
    if date_str.is_empty() {
        return "—".to_string();
    }
    date_str.split('T').next().unwrap_or(date_str).to_string()
}
