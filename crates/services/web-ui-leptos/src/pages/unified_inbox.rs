//! Unified Inbox page - Gmail-style view of ALL messages across ALL projects.
//! Uses the /api/unified-inbox endpoint.
//!
//! Features:
//! - SplitViewLayout for Gmail-style two-column view on desktop
//! - FilterBar with search, project, sender, importance filters
//! - InlineMessageDetail for viewing messages without navigation
//! - Mobile fallback with card-based list

use crate::api::client::{self, Agent, UnifiedInboxMessage};
use crate::components::{
    Alert, AlertDescription, AlertTitle, AlertVariant, Badge, BadgeVariant, Button, ButtonSize,
    ButtonVariant, FilterBar, FilterState, InlineMessageDetail, MessageListItem,
    OverseerComposeProps, OverseerComposer, SplitViewLayout,
};
use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

/// Unified Inbox page component.
#[component]
pub fn UnifiedInbox() -> impl IntoView {
    let query = use_query_map();

    // State
    let messages = RwSignal::new(Vec::<UnifiedInboxMessage>::new());
    let all_messages = RwSignal::new(Vec::<UnifiedInboxMessage>::new()); // Unfiltered for extracting options
    let loading = RwSignal::new(true);
    let error = RwSignal::new(Option::<String>::None);
    let filter_state = RwSignal::new(query.with_untracked(FilterState::from_params_map));
    let selected_id = RwSignal::new(Option::<i64>::None);

    // Overseer Composer state
    let show_overseer = RwSignal::new(false);
    let overseer_agents = RwSignal::new(Vec::<Agent>::new());
    let overseer_project = RwSignal::new(String::new());

    // Load all messages once on mount
    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            loading.set(true);
            error.set(None);

            match client::get_unified_inbox(None, Some(100)).await {
                Ok(msgs) => {
                    // Auto-select first message if nothing selected
                    if selected_id.get_untracked().is_none() {
                        if let Some(first) = msgs.first() {
                            selected_id.set(Some(first.id));
                        }
                    }
                    all_messages.set(msgs.clone());
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

    // Apply filters reactively
    Effect::new(move |_| {
        let filter = filter_state.get();
        let all = all_messages.get();

        let filtered: Vec<UnifiedInboxMessage> = all
            .into_iter()
            .filter(|msg| {
                // Search query filter
                if !filter.query.is_empty() {
                    let q = filter.query.to_lowercase();
                    let matches = msg.subject.to_lowercase().contains(&q)
                        || msg.sender_name.to_lowercase().contains(&q)
                        || msg
                            .thread_id
                            .as_ref()
                            .is_some_and(|t| t.to_lowercase().contains(&q));
                    if !matches {
                        return false;
                    }
                }

                // Importance filter
                if let Some(ref imp) = filter.importance {
                    if msg.importance != *imp {
                        return false;
                    }
                }

                // Sender filter
                if let Some(ref sender) = filter.sender {
                    if msg.sender_name != *sender {
                        return false;
                    }
                }

                // Project filter (uses project_slug for display-friendly matching)
                if let Some(ref project) = filter.project {
                    if msg.project_slug != *project {
                        return false;
                    }
                }

                true
            })
            .collect();

        // If current selection is no longer visible, select first filtered message
        if let Some(current_id) = selected_id.get_untracked() {
            let still_visible = filtered.iter().any(|m| m.id == current_id);
            if !still_visible {
                if let Some(first) = filtered.first() {
                    selected_id.set(Some(first.id));
                }
            }
        } else if let Some(first) = filtered.first() {
            // No selection, select first
            selected_id.set(Some(first.id));
        }

        messages.set(filtered);
    });

    // Extract unique senders for filter dropdown
    let senders = Signal::derive(move || {
        let mut senders: Vec<String> = all_messages
            .get()
            .iter()
            .map(|m| m.sender_name.clone())
            .collect();
        senders.sort();
        senders.dedup();
        leptos::logging::log!("UnifiedInbox: Found {} unique senders", senders.len());
        senders
    });

    // Extract unique project slugs for filter dropdown
    let projects = Signal::derive(move || {
        let mut projects: Vec<String> = all_messages
            .get()
            .iter()
            .map(|m| m.project_slug.clone())
            .collect();
        projects.sort();
        projects.dedup();
        leptos::logging::log!("UnifiedInbox: Found {} unique projects", projects.len());
        projects
    });

    // Message count for FilterBar
    let message_count = Signal::derive(move || messages.get().len());

    // Convert messages to MessageListItem format for SplitViewLayout
    let message_list_items = Signal::derive(move || {
        messages
            .get()
            .iter()
            .map(|msg| MessageListItem {
                id: msg.id,
                sender: msg.sender_name.clone(),
                subject: msg.subject.clone(),
                timestamp: format_date(&msg.created_ts),
                unread: false, // Read state not yet tracked.
                importance: msg.importance.clone(),
                project_slug: msg.project_slug.clone(),
            })
            .collect::<Vec<_>>()
    });

    // Get project slug for selected message (used for InlineMessageDetail)
    let selected_project = Signal::derive(move || {
        if let Some(id) = selected_id.get() {
            messages
                .get()
                .iter()
                .find(|m| m.id == id)
                .map(|m| m.project_slug.clone())
                .unwrap_or_default()
        } else {
            String::new()
        }
    });

    // Handle message selection
    let on_select = Callback::new(move |id: i64| {
        selected_id.set(Some(id));
    });

    // Handle Overseer button click
    let open_overseer = move |_| {
        let project_slug = selected_project.get();
        if project_slug.is_empty() {
            error.set(Some(
                "Select a message first to use Overseer mode.".to_string(),
            ));
            return;
        }
        // Fetch agents for the selected project
        overseer_project.set(project_slug.clone());
        leptos::task::spawn_local(async move {
            match client::get_agents(&project_slug).await {
                Ok(agents) => {
                    overseer_agents.set(agents);
                    show_overseer.set(true);
                }
                Err(e) => {
                    error.set(Some(format!("Failed to load agents: {}", e.message)));
                }
            }
        });
    };

    // Refresh messages after sending
    let refresh_messages = move || {
        leptos::task::spawn_local(async move {
            if let Ok(msgs) = client::get_unified_inbox(None, Some(100)).await {
                all_messages.set(msgs.clone());
                messages.set(msgs);
            }
        });
    };

    view! {
        <div class="space-y-6">
            // Overseer Composer Modal - shadcn Dialog pattern with proper z-index layering
            {move || {
                if show_overseer.get() {
                    let agents = overseer_agents.get();
                    let project = overseer_project.get();
                    Some(view! {
                        // Dialog portal container - modal layer
                        <div
                            class="fixed inset-0 z-[1050]"
                            role="dialog"
                            aria-modal="true"
                            aria-labelledby="overseer-dialog-title"
                        >
                            // Backdrop overlay - completely opaque to block all page content
                            <div
                                class="fixed inset-0 z-[1050] bg-black/95 backdrop-blur-sm animate-fade-in"
                                on:click=move |_| show_overseer.set(false)
                                aria-hidden="true"
                            ></div>
                            // Dialog content container - centered with proper positioning
                            <div class="fixed inset-0 z-[1100] flex items-center justify-center p-4 p-6 overflow-y-auto">
                                <OverseerComposer
                                props=OverseerComposeProps {
                                    project_slug: project,
                                    agents,
                                    reply_to_thread_id: None,
                                    reply_to_recipient: None,
                                    reply_subject: None,
                                }
                                on_close=Callback::new(move |_| show_overseer.set(false))
                                on_sent=Callback::new(move |_| {
                                    show_overseer.set(false);
                                    refresh_messages();
                                })
                            />
                            </div>
                        </div>
                    })
                } else {
                    None
                }
            }}

            // Header - 2025 Magic UI Design with animated gradient
            <div class="inbox-header-gradient rounded-xl p-6 mb-4 animate-fade-in">
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-4">
                        // Animated icon container - properly scaled
                        <div class="relative">
                            <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-primary/20 to-violet-500/20 flex items-center justify-center animate-float">
                                <svg class="h-4 w-4 text-primary" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <polyline points="22 12 16 12 14 15 10 15 8 12 2 12"></polyline>
                                    <path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"></path>
                                </svg>
                            </div>
                            // Subtle glow effect
                            <div class="absolute inset-0 rounded-lg bg-primary/10 blur-md -z-10"></div>
                        </div>
                        <div>
                            <h1 class="text-2xl font-bold tracking-tight text-foreground">
                                <span class="text-gradient-animated">"Unified Inbox"</span>
                            </h1>
                            <p class="text-sm text-muted-foreground mt-1 flex items-center gap-2">
                                <span class="inline-flex items-center">
                                    <span class="w-2 h-2 rounded-full bg-teal-500 animate-pulse-gentle mr-2"></span>
                                    "All messages across all projects"
                                </span>
                            </p>
                        </div>
                    </div>
                    <div class="flex items-center gap-6">
                        // Message counter badge - properly scaled with inline SVG
                        <div class="hidden sm:flex items-center gap-2 px-3 py-1.5 rounded-full bg-secondary/50 border border-border">
                            <svg class="h-3 w-3 text-muted-foreground" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <path d="m22 7-8.991 5.727a2 2 0 0 1-2.009 0L2 7"></path>
                                <rect x="2" y="4" width="20" height="16" rx="2"></rect>
                            </svg>
                            <Badge
                                variant=BadgeVariant::Default
                                class="h-6 min-w-[2.5rem] rounded-full font-mono tabular-nums px-2".to_string()
                            >
                                {move || message_count.get()}
                            </Badge>
                            <span class="text-xs text-muted-foreground font-medium">"messages"</span>
                        </div>
                        // Overseer button - professional amber styling with proper padding
                        <Button
                            variant=ButtonVariant::Secondary
                            size=ButtonSize::Default
                            on_click=Callback::new(open_overseer)
                            class="bg-amber-500 hover:bg-amber-600 text-white border-0 shadow-md hover:shadow-lg font-medium px-4".to_string()
                        >
                            <svg class="h-4 w-4 mr-2" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"></path>
                                <path d="M12 8v4"></path>
                                <path d="M12 16h.01"></path>
                            </svg>
                            <span>"Overseer Mode"</span>
                        </Button>
                    </div>
                </div>
            </div>

            // Filter Bar with glass morphism
            <div class="filter-bar-glass p-1 mb-4 animate-slide-up stagger-delay-1">
                {move || {
                    view! {
                        <FilterBar
                            filter_state=filter_state
                            message_count=message_count
                            projects=projects.get()
                            senders=senders.get()
                        />
                    }
                }}
            </div>

            // Error
            {move || {
                error.get().map(|e| view! {
                    <Alert variant=AlertVariant::Destructive>
                        <AlertTitle>"Error loading messages"</AlertTitle>
                        <AlertDescription>{e}</AlertDescription>
                    </Alert>
                })
            }}

            // Loading with shimmer effect
            {move || {
                if loading.get() {
                    Some(view! {
                        <div class="space-y-4 animate-fade-in">
                            // Shimmer loading skeletons
                            <div class="flex flex-col lg:flex-row gap-6 h-[calc(100vh-18rem)] rounded-xl border bg-card overflow-hidden">
                                // Message list skeleton
                                <div class="flex-none w-full lg:w-[35%] border-r border-border p-4 space-y-3">
                                    {(0..8).map(|i| view! {
                                        <div class="flex items-start gap-3 p-3 rounded-lg" style={format!("animation-delay: {}ms", i * 100)}>
                                            <div class="w-10 h-10 rounded-full shimmer-pro"></div>
                                            <div class="flex-1 space-y-2">
                                                <div class="h-4 w-24 rounded shimmer-pro"></div>
                                                <div class="h-3 w-full rounded shimmer-pro"></div>
                                            </div>
                                        </div>
                                    }).collect::<Vec<_>>()}
                                </div>
                                // Detail panel skeleton
                                <div class="hidden lg:block flex-1 p-6 space-y-4">
                                    <div class="h-8 w-64 rounded shimmer-pro"></div>
                                    <div class="space-y-2 mt-6">
                                        <div class="h-4 w-full rounded shimmer-pro"></div>
                                        <div class="h-4 w-5/6 rounded shimmer-pro"></div>
                                        <div class="h-4 w-4/6 rounded shimmer-pro"></div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    })
                } else {
                    None
                }
            }}

            // SplitViewLayout - Gmail-style two-column view with 2025 design
            {move || {
                if !loading.get() {
                    let items = message_list_items.get();
                    let selected_signal: Signal<Option<i64>> = selected_id.into();
                    Some(view! {
                        <div class="animate-scale-in">
                            <SplitViewLayout
                                messages=items
                                selected_id=selected_signal
                                on_select=on_select
                            >
                                {move || {
                                    if let Some(id) = selected_id.get() {
                                        view! {
                                            <div class="animate-blur-fade">
                                                <InlineMessageDetail
                                                    message_id=Signal::derive(move || id)
                                                    project_slug=selected_project
                                                />
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div class="h-full flex flex-col items-center justify-center text-muted-foreground">
                                                <div class="w-20 h-20 rounded-full bg-gradient-to-br from-primary/10 to-violet-500/10 flex items-center justify-center mb-4 animate-float">
                                                    <i data-lucide="mail-open" class="w-10 h-10 opacity-60"></i>
                                                </div>
                                                <p class="text-lg font-medium">"Select a message"</p>
                                                <p class="text-sm mt-1 text-muted-foreground/80">
                                                    "Choose a message from the list to view its contents"
                                                </p>
                                                <div class="mt-4 flex gap-2 text-xs text-muted-foreground/60">
                                                    <kbd class="kbd">"↑"</kbd>
                                                    <kbd class="kbd">"↓"</kbd>
                                                    <span>"to navigate"</span>
                                                </div>
                                            </div>
                                        }.into_any()
                                    }
                                }}
                            </SplitViewLayout>
                        </div>
                    })
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
