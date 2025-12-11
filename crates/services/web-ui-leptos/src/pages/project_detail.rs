//! Project detail page - view project info and manage agents.

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use crate::api::client::{self, Agent};

/// Project detail page component.
#[component]
pub fn ProjectDetail() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.read().get("slug").unwrap_or_default();

    // State
    let agents = RwSignal::new(Vec::<Agent>::new());
    let loading = RwSignal::new(true);
    let error = RwSignal::new(Option::<String>::None);
    let show_new_form = RwSignal::new(false);
    let creating = RwSignal::new(false);
    
    // Form fields
    let new_name = RwSignal::new(String::new());
    let new_program = RwSignal::new(String::new());
    let new_model = RwSignal::new(String::new());
    let new_task = RwSignal::new(String::new());

    // Load agents
    let load_agents = {
        let slug = slug.clone();
        move || {
            let project_slug = slug();
            loading.set(true);
            error.set(None);
            leptos::task::spawn_local(async move {
                match client::get_agents(&project_slug).await {
                    Ok(a) => {
                        agents.set(a);
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(e.message));
                        loading.set(false);
                    }
                }
            });
        }
    };

    // Initial load
    Effect::new({
        let load = load_agents.clone();
        move |_| { load(); }
    });

    // Create agent handler
    let create_agent = {
        let slug = slug.clone();
        let load_agents = load_agents.clone();
        move |_| {
            let name = new_name.get();
            if name.trim().is_empty() {
                return;
            }

            let project_slug = slug();
            let program = new_program.get();
            let model = new_model.get();
            let task = new_task.get();

            creating.set(true);
            error.set(None);

            leptos::task::spawn_local(async move {
                match client::register_agent(
                    &project_slug,
                    &name,
                    if program.is_empty() { "unknown" } else { &program },
                    if model.is_empty() { "unknown" } else { &model },
                    if task.is_empty() { None } else { Some(task.as_str()) },
                ).await {
                    Ok(_) => {
                        // Reload agents
                        match client::get_agents(&project_slug).await {
                            Ok(a) => agents.set(a),
                            Err(e) => error.set(Some(e.message)),
                        }
                        new_name.set(String::new());
                        new_program.set(String::new());
                        new_model.set(String::new());
                        new_task.set(String::new());
                        show_new_form.set(false);
                        creating.set(false);
                    }
                    Err(e) => {
                        error.set(Some(e.message));
                        creating.set(false);
                    }
                }
            });
        }
    };

    view! {
        <div class="space-y-6">
            // Breadcrumb
            <nav class="flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400">
                <a href="/projects" class="hover:text-primary-600 dark:hover:text-primary-400">"Projects"</a>
                <span>"/"</span>
                <span class="text-gray-900 dark:text-white font-medium">{slug}</span>
            </nav>

            // Header
            <div class="flex items-center justify-between">
                <div>
                    <h1 class="text-2xl font-bold text-gray-900 dark:text-white">{slug}</h1>
                    <p class="text-gray-600 dark:text-gray-400">"Agents in this project"</p>
                </div>
                <button
                    on:click=move |_| show_new_form.update(|v| *v = !*v)
                    class="px-4 py-2 bg-primary-600 text-white rounded-lg hover:bg-primary-700 transition-colors flex items-center gap-2"
                >
                    <span class="text-lg">"+"</span>
                    <span>"Register Agent"</span>
                </button>
            </div>

            // New Agent Form
            {move || {
                if show_new_form.get() {
                    Some(view! {
                        <div class="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
                            <h2 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">"Register New Agent"</h2>
                            <form on:submit=move |ev| { ev.prevent_default(); create_agent(()); } class="space-y-4">
                                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                    <div>
                                        <label for="agentName" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                            "Agent Name *"
                                        </label>
                                        <input
                                            id="agentName"
                                            type="text"
                                            prop:value=move || new_name.get()
                                            on:input=move |ev| new_name.set(event_target_value(&ev))
                                            placeholder="BlueStone"
                                            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary-500 focus:border-transparent"
                                        />
                                    </div>
                                    <div>
                                        <label for="agentProgram" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                            "Program"
                                        </label>
                                        <input
                                            id="agentProgram"
                                            type="text"
                                            prop:value=move || new_program.get()
                                            on:input=move |ev| new_program.set(event_target_value(&ev))
                                            placeholder="claude-code"
                                            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary-500 focus:border-transparent"
                                        />
                                    </div>
                                    <div>
                                        <label for="agentModel" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                            "Model"
                                        </label>
                                        <input
                                            id="agentModel"
                                            type="text"
                                            prop:value=move || new_model.get()
                                            on:input=move |ev| new_model.set(event_target_value(&ev))
                                            placeholder="claude-3-opus"
                                            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary-500 focus:border-transparent"
                                        />
                                    </div>
                                    <div>
                                        <label for="agentTask" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                            "Task Description"
                                        </label>
                                        <input
                                            id="agentTask"
                                            type="text"
                                            prop:value=move || new_task.get()
                                            on:input=move |ev| new_task.set(event_target_value(&ev))
                                            placeholder="Research and implement features"
                                            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary-500 focus:border-transparent"
                                        />
                                    </div>
                                </div>
                                <div class="flex gap-3">
                                    <button
                                        type="submit"
                                        disabled=move || creating.get() || new_name.get().trim().is_empty()
                                        class="px-4 py-2 bg-primary-600 text-white rounded-lg hover:bg-primary-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                                    >
                                        {move || if creating.get() { "Registering..." } else { "Register Agent" }}
                                    </button>
                                    <button
                                        type="button"
                                        on:click=move |_| {
                                            show_new_form.set(false);
                                            new_name.set(String::new());
                                            new_program.set(String::new());
                                            new_model.set(String::new());
                                            new_task.set(String::new());
                                        }
                                        class="px-4 py-2 bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors"
                                    >
                                        "Cancel"
                                    </button>
                                </div>
                            </form>
                        </div>
                    })
                } else {
                    None
                }
            }}

            // Error Message
            {move || {
                error.get().map(|e| view! {
                    <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-xl p-4">
                        <p class="text-red-700 dark:text-red-400">{e}</p>
                    </div>
                })
            }}

            // Content: Loading / Empty / Grid
            {move || {
                if loading.get() {
                    view! {
                        <div class="flex items-center justify-center py-12">
                            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
                        </div>
                    }.into_any()
                } else {
                    let agent_list = agents.get();
                    if agent_list.is_empty() {
                        view! {
                            <div class="bg-white dark:bg-gray-800 rounded-xl p-12 text-center shadow-sm border border-gray-200 dark:border-gray-700">
                                <div class="text-4xl mb-4">"🤖"</div>
                                <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-2">"No agents yet"</h3>
                                <p class="text-gray-600 dark:text-gray-400 mb-4">
                                    "Register your first agent to start sending and receiving messages."
                                </p>
                                <button
                                    on:click=move |_| show_new_form.set(true)
                                    class="px-4 py-2 bg-primary-600 text-white rounded-lg hover:bg-primary-700 transition-colors"
                                >
                                    "Register Agent"
                                </button>
                            </div>
                        }.into_any()
                    } else {
                        let project_slug = slug();
                        view! {
                            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                                {agent_list.into_iter().map(|agent| {
                                    let name = agent.name.clone();
                                    let program = agent.program.clone().unwrap_or_else(|| "unknown".to_string());
                                    let model = agent.model.clone().unwrap_or_else(|| "unknown".to_string());
                                    let task = agent.task_description.clone();
                                    let last_active = agent.last_active_ts.clone().unwrap_or_default();
                                    let inbox_href = format!("/inbox?project={}&agent={}", project_slug, name);
                                    
                                    view! {
                                        <div class="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700 hover:shadow-md transition-shadow">
                                            <div class="flex items-start justify-between mb-4">
                                                <div class="flex items-center gap-3">
                                                    <div class="w-10 h-10 bg-primary-100 dark:bg-primary-900 rounded-full flex items-center justify-center">
                                                        <span class="text-lg">"🤖"</span>
                                                    </div>
                                                    <div>
                                                        <h3 class="font-semibold text-gray-900 dark:text-white">{name.clone()}</h3>
                                                        <p class="text-sm text-gray-500 dark:text-gray-400">{program}</p>
                                                    </div>
                                                </div>
                                            </div>
                                            
                                            <div class="space-y-2 text-sm">
                                                <div class="flex justify-between">
                                                    <span class="text-gray-500 dark:text-gray-400">"Model"</span>
                                                    <span class="text-gray-900 dark:text-white font-mono">{model}</span>
                                                </div>
                                                {task.map(|t| view! {
                                                    <div>
                                                        <span class="text-gray-500 dark:text-gray-400">"Task"</span>
                                                        <p class="text-gray-700 dark:text-gray-300 mt-1 line-clamp-2">{t}</p>
                                                    </div>
                                                })}
                                                <div class="flex justify-between pt-2 border-t border-gray-200 dark:border-gray-700">
                                                    <span class="text-gray-500 dark:text-gray-400">"Last Active"</span>
                                                    <span class="text-gray-600 dark:text-gray-400">{format_date(&last_active)}</span>
                                                </div>
                                            </div>

                                            <div class="mt-4 pt-4 border-t border-gray-200 dark:border-gray-700">
                                                <a
                                                    href=inbox_href
                                                    class="text-primary-600 dark:text-primary-400 hover:text-primary-800 dark:hover:text-primary-300 text-sm font-medium"
                                                >
                                                    "View Inbox →"
                                                </a>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }.into_any()
                    }
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
