//! OverseerComposer modal component.
//! Specialized composer for Human Overseer interventions.
//!
//! Follows shadcn/ui Dialog anatomy with destructive theme variant.

use super::{Button, ButtonSize, ButtonVariant, Input, Select, SelectOption};
use crate::api::client::{self, Agent};
use leptos::prelude::*;

/// Props for OverseerComposer component.
#[derive(Clone)]
pub struct OverseerComposeProps {
    pub project_slug: String,
    pub agents: Vec<Agent>,
    // Reply context (optional)
    pub reply_to_thread_id: Option<String>,
    pub reply_to_recipient: Option<String>,
    pub reply_subject: Option<String>,
}

/// specialized composer for "Overseer" commands.
#[component]
pub fn OverseerComposer(
    props: OverseerComposeProps,
    on_close: Callback<()>,
    on_sent: Callback<()>,
) -> impl IntoView {
    // Form state
    let recipients = RwSignal::new(Vec::<String>::new());
    let subject = RwSignal::new(String::new());
    let body = RwSignal::new(String::new());
    let importance = RwSignal::new("high".to_string()); // Default to High for Overseer
    let ack_required = RwSignal::new(true); // Default to True for Overseer
    let thread_id = RwSignal::new(String::new());

    let sending = RwSignal::new(false);
    let error = RwSignal::new(Option::<String>::None);

    // Initialize from props
    if let Some(ref r) = props.reply_to_recipient {
        recipients.set(vec![r.clone()]);
    }
    if let Some(ref s) = props.reply_subject {
        subject.set(format!("OVERSEER: {}", s.trim_start_matches("re: ")));
    }
    if let Some(ref t) = props.reply_to_thread_id {
        thread_id.set(t.clone());
    }

    let project_slug = props.project_slug.clone();
    // Hardcoded sender for Overseer Mode
    let sender_name = "Overseer".to_string();

    let all_agents = props.agents.clone();

    // Toggle recipient selection
    let toggle_recipient = move |name: String| {
        let mut current = recipients.get();
        if current.contains(&name) {
            current.retain(|r| r != &name);
        } else {
            current.push(name);
        }
        recipients.set(current);
    };

    // Toggle All Candidates
    let all_agents_clone = all_agents.clone();
    let toggle_all = move |_| {
        let current_len = recipients.get().len();
        if current_len == all_agents_clone.len() {
            recipients.set(vec![]);
        } else {
            recipients.set(all_agents_clone.iter().map(|a| a.name.clone()).collect());
        }
    };

    // Send message handler
    let handle_submit = {
        let project_slug = project_slug.clone();
        let sender_name = sender_name.clone();
        move |_| {
            let recips = recipients.get();
            let subj = subject.get();
            let bod = body.get();

            if recips.is_empty() {
                error.set(Some("Target at least one agent.".to_string()));
                return;
            }
            if subj.trim().is_empty() {
                error.set(Some("Command subject required.".to_string()));
                return;
            }
            if bod.trim().is_empty() {
                error.set(Some("Command instructions required.".to_string()));
                return;
            }

            sending.set(true);
            error.set(None);

            let project = project_slug.clone();
            let sender = sender_name.clone();
            let tid = thread_id.get();
            let imp = importance.get();
            let ack = ack_required.get();
            let on_sent = on_sent;

            leptos::task::spawn_local(async move {
                match client::send_message(
                    &project,
                    &sender,
                    &recips,
                    &subj,
                    &bod,
                    if tid.is_empty() {
                        None
                    } else {
                        Some(tid.as_str())
                    },
                    &imp,
                    ack,
                )
                .await
                {
                    Ok(_) => {
                        on_sent.run(());
                    }
                    Err(e) => {
                        error.set(Some(e.message));
                        sending.set(false);
                    }
                }
            });
        }
    };

    view! {
        // Dialog Content - shadcn anatomy with destructive theme
        <div class="flex flex-col h-full max-h-[90vh] rounded-lg border border-destructive/50 bg-background shadow-lg overflow-hidden">
            // DialogHeader - shadcn pattern
            <div class="flex flex-col space-y-1.5 p-6 bg-destructive/10 border-b border-destructive/20">
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-3">
                        <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-destructive/20">
                            <i data-lucide="shield-alert" class="h-5 w-5 text-destructive"></i>
                        </div>
                        <div class="flex flex-col space-y-1">
                            <h2 class="text-lg font-semibold leading-none tracking-tight text-destructive">
                                "Overseer Intervention"
                            </h2>
                            <p class="text-sm text-muted-foreground">
                                "Issuing authoritative commands as 'Overseer'"
                            </p>
                        </div>
                    </div>
                    <Button
                        variant=ButtonVariant::Ghost
                        size=ButtonSize::Icon
                        on_click=Callback::new(move |_| on_close.run(()))
                        class="rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2".to_string()
                    >
                        <i data-lucide="x" class="h-4 w-4"></i>
                        <span class="sr-only">"Close"</span>
                    </Button>
                </div>
            </div>

            // DialogContent - Form area
            <div class="flex-1 overflow-y-auto p-6 space-y-6">
                // Target Agent Selection
                <div class="space-y-3">
                    <div class="flex items-center justify-between">
                        <label class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                            "Target Agents"
                        </label>
                        <button
                            type="button"
                            class="text-sm text-destructive hover:underline underline-offset-4 font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 rounded-sm"
                            on:click=toggle_all
                        >
                            {
                                let total_len = all_agents.len();
                                move || if recipients.get().len() == total_len { "Deselect All" } else { "Select All" }
                            }
                        </button>
                    </div>

                    {if all_agents.is_empty() {
                        view! { <p class="text-sm text-muted-foreground italic">"No agents available."</p> }.into_any()
                    } else {
                        view! {
                            <div class="flex flex-wrap gap-2">
                                {all_agents.iter().map(|agent| {
                                    let name = agent.name.clone();
                                    let name_display = name.clone();
                                    let toggle = toggle_recipient;
                                    // shadcn toggle button pattern - outline variant when unselected, destructive when selected
                                    view! {
                                        <button
                                            type="button"
                                            on:click=move |_| toggle(name.clone())
                                            class=move || {
                                                // Base classes from shadcn Button
                                                let base = "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 h-9 px-3";
                                                if recipients.get().contains(&name_display) {
                                                    // Selected: destructive variant
                                                    format!("{} bg-destructive text-destructive-foreground hover:bg-destructive/90 shadow-sm", base)
                                                } else {
                                                    // Unselected: outline variant
                                                    format!("{} border border-input bg-background hover:bg-accent hover:text-accent-foreground", base)
                                                }
                                            }
                                        >
                                            <i data-lucide="bot" class="h-4 w-4 shrink-0"></i>
                                            <span class="truncate max-w-[100px]">{name_display.clone()}</span>
                                        </button>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_any()
                    }}
                </div>

                // Subject / Directive - shadcn Input pattern
                <div class="space-y-2">
                    <label for="subject" class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                        "Directive / Subject"
                    </label>
                    <Input
                        id="subject".to_string()
                        value=subject
                        placeholder="e.g., STOP IMMEDIATELY, UPDATE PRIORITY...".to_string()
                    />
                </div>

                // Metadata Details - Grid layout
                <div class="grid grid-cols-2 gap-4">
                    <div class="space-y-2">
                        <label class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                            "Importance"
                        </label>
                        <Select
                            id="importance".to_string()
                            options=vec![
                                SelectOption::new("normal", "Normal"),
                                SelectOption::new("high", "High (Priority)"),
                            ]
                            value=importance
                            placeholder="Select...".to_string()
                            disabled=false
                        />
                    </div>
                    <div class="space-y-2">
                        <label class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                            "Thread Context"
                        </label>
                        <Input
                            id="threadId".to_string()
                            value=thread_id
                            placeholder="New Thread".to_string()
                        />
                    </div>
                </div>

                // Instructions - Textarea with explicit dark mode support
                <div class="space-y-2">
                    <label for="body" class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                        "Instructions"
                    </label>
                    <textarea
                        id="body"
                        prop:value=move || body.get()
                        on:input=move |ev| body.set(event_target_value(&ev))
                        rows="6"
                        placeholder="Detailed instructions for the agents..."
                        class="flex min-h-[120px] w-full rounded-md border border-input bg-white dark:bg-zinc-900 text-foreground px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 resize-none font-mono"
                    ></textarea>
                </div>

                // Acknowledgment Checkbox - with visible text on amber background
                <div class="flex items-start space-x-3 rounded-md border border-amber-300 dark:border-amber-700 bg-amber-50 dark:bg-amber-950/30 p-4">
                    <button
                        type="button"
                        role="checkbox"
                        aria-checked=move || ack_required.get().to_string()
                        on:click=move |_| ack_required.set(!ack_required.get())
                        class=move || {
                            let base = "peer h-5 w-5 shrink-0 rounded-sm border ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50";
                            if ack_required.get() {
                                format!("{} border-amber-600 bg-amber-600 text-white", base)
                            } else {
                                format!("{} border-amber-600 bg-white dark:bg-zinc-900", base)
                            }
                        }
                    >
                        {move || ack_required.get().then(|| view! {
                            <span class="flex items-center justify-center text-current">
                                <i data-lucide="check" class="h-4 w-4"></i>
                            </span>
                        })}
                    </button>
                    <div class="grid gap-1.5 leading-none">
                        <label class="text-sm font-semibold leading-none text-amber-900 dark:text-amber-100">
                            "Require Explicit Acknowledgment"
                        </label>
                        <p class="text-sm text-amber-700 dark:text-amber-300">
                            "Agents must confirm receipt of this directive."
                        </p>
                    </div>
                </div>

                // Error Alert - shadcn Alert destructive variant
                {move || {
                    error.get().map(|e| view! {
                        <div
                            role="alert"
                            class="relative w-full rounded-lg border border-destructive/50 p-4 text-destructive [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg]:text-destructive [&>svg~*]:pl-7"
                        >
                            <i data-lucide="alert-circle" class="h-4 w-4"></i>
                            <h5 class="mb-1 font-medium leading-none tracking-tight">"Error"</h5>
                            <div class="text-sm [&_p]:leading-relaxed">{e}</div>
                        </div>
                    })
                }}
            </div>

            // DialogFooter - fixed layout to prevent button cutoff
            <div class="p-6 border-t border-border bg-muted/50">
                <div class="flex items-center justify-end gap-3">
                    <Button
                        variant=ButtonVariant::Outline
                        on_click=Callback::new(move |_| on_close.run(()))
                    >
                        <span>"Cancel"</span>
                    </Button>
                    <Button
                        variant=ButtonVariant::Destructive
                        on_click=Callback::new(move |_| handle_submit(()))
                        disabled=Signal::derive(move || sending.get() || recipients.get().is_empty())
                    >
                        {move || {
                            if sending.get() {
                                view! {
                                    <i data-lucide="loader-2" class="mr-2 h-4 w-4 animate-spin"></i>
                                    <span>"Transmitting..."</span>
                                }.into_any()
                            } else {
                                view! {
                                    <i data-lucide="megaphone" class="mr-2 h-4 w-4"></i>
                                    <span>"Broadcast Directive"</span>
                                }.into_any()
                            }
                        }}
                    </Button>
                </div>
            </div>
        </div>
    }
}

fn event_target_value(ev: &web_sys::Event) -> String {
    use wasm_bindgen::JsCast;
    ev.target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlTextAreaElement>().ok())
        .map(|el| el.value())
        .unwrap_or_default()
}
