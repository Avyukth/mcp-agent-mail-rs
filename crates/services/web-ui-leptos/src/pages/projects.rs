//! Projects page - list and create projects.

use leptos::prelude::*;
use crate::api::client::{self, Project};

/// Projects page component.
#[component]
pub fn Projects() -> impl IntoView {
    // State
    let projects = RwSignal::new(Vec::<Project>::new());
    let loading = RwSignal::new(true);
    let error = RwSignal::new(Option::<String>::None);
    let show_new_form = RwSignal::new(false);
    let new_project_path = RwSignal::new(String::new());
    let creating = RwSignal::new(false);

    // Load projects on mount
    let load_projects = move || {
        loading.set(true);
        error.set(None);
        leptos::task::spawn_local(async move {
            match client::get_projects().await {
                Ok(p) => {
                    projects.set(p);
                    loading.set(false);
                }
                Err(e) => {
                    error.set(Some(e.message));
                    loading.set(false);
                }
            }
        });
    };

    // Initial load
    Effect::new(move |_| {
        load_projects();
    });

    // Create project handler
    let create_project = move |_| {
        let path = new_project_path.get();
        if path.trim().is_empty() {
            return;
        }

        creating.set(true);
        error.set(None);
        
        leptos::task::spawn_local(async move {
            match client::ensure_project(&path).await {
                Ok(_) => {
                    // Reload projects
                    match client::get_projects().await {
                        Ok(p) => {
                            projects.set(p);
                        }
                        Err(e) => {
                            error.set(Some(e.message));
                        }
                    }
                    new_project_path.set(String::new());
                    show_new_form.set(false);
                    creating.set(false);
                }
                Err(e) => {
                    error.set(Some(e.message));
                    creating.set(false);
                }
            }
        });
    };

    view! {
        <div class="space-y-6">
            // Header
            <div class="flex items-center justify-between">
                <div>
                    <h1 class="text-2xl font-bold text-gray-900 dark:text-white">"Projects"</h1>
                    <p class="text-gray-600 dark:text-gray-400">"Manage your agent mail projects"</p>
                </div>
                <button
                    on:click=move |_| show_new_form.update(|v| *v = !*v)
                    class="px-4 py-2 bg-primary-600 text-white rounded-lg hover:bg-primary-700 transition-colors flex items-center gap-2"
                >
                    <span class="text-lg">"+"</span>
                    <span>"New Project"</span>
                </button>
            </div>

            // New Project Form
            {move || {
                if show_new_form.get() {
                    Some(view! {
                        <div class="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
                            <h2 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">"Create New Project"</h2>
                            <form on:submit=move |ev| { ev.prevent_default(); create_project(()); } class="space-y-4">
                                <div>
                                    <label for="projectPath" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                        "Project Path (human_key)"
                                    </label>
                                    <input
                                        id="projectPath"
                                        type="text"
                                        prop:value=move || new_project_path.get()
                                        on:input=move |ev| new_project_path.set(event_target_value(&ev))
                                        placeholder="/path/to/your/project"
                                        class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary-500 focus:border-transparent"
                                    />
                                    <p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
                                        "The absolute path to your project directory"
                                    </p>
                                </div>
                                <div class="flex gap-3">
                                    <button
                                        type="submit"
                                        disabled=move || creating.get() || new_project_path.get().trim().is_empty()
                                        class="px-4 py-2 bg-primary-600 text-white rounded-lg hover:bg-primary-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                                    >
                                        {move || if creating.get() { "Creating..." } else { "Create Project" }}
                                    </button>
                                    <button
                                        type="button"
                                        on:click=move |_| { show_new_form.set(false); new_project_path.set(String::new()); }
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

            // Content: Loading / Empty / List
            {move || {
                if loading.get() {
                    // Loading State
                    view! {
                        <div class="flex items-center justify-center py-12">
                            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
                        </div>
                    }.into_any()
                } else {
                    let project_list = projects.get();
                    if project_list.is_empty() {
                        // Empty State
                        view! {
                            <div class="bg-white dark:bg-gray-800 rounded-xl p-12 text-center shadow-sm border border-gray-200 dark:border-gray-700">
                                <div class="text-4xl mb-4">"📁"</div>
                                <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-2">"No projects yet"</h3>
                                <p class="text-gray-600 dark:text-gray-400 mb-4">
                                    "Create your first project to start sending messages between agents."
                                </p>
                                <button
                                    on:click=move |_| show_new_form.set(true)
                                    class="px-4 py-2 bg-primary-600 text-white rounded-lg hover:bg-primary-700 transition-colors"
                                >
                                    "Create Project"
                                </button>
                            </div>
                        }.into_any()
                    } else {
                        // Projects Table
                        view! {
                            <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden">
                                <table class="w-full">
                                    <thead class="bg-gray-50 dark:bg-gray-700">
                                        <tr>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                                                "Slug"
                                            </th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                                                "Path"
                                            </th>
                                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                                                "Created"
                                            </th>
                                            <th class="px-6 py-3 text-right text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                                                "Actions"
                                            </th>
                                        </tr>
                                    </thead>
                                    <tbody class="divide-y divide-gray-200 dark:divide-gray-700">
                                        {project_list.into_iter().map(|project| {
                                            let slug = project.slug.clone();
                                            let href = format!("/projects/{}", slug);
                                            let href2 = href.clone();
                                            let human_key = project.human_key.clone().unwrap_or_default();
                                            let created = project.created_at.clone().unwrap_or_default();
                                            view! {
                                                <tr class="hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors">
                                                    <td class="px-6 py-4 whitespace-nowrap">
                                                        <a
                                                            href=href
                                                            class="text-primary-600 dark:text-primary-400 font-medium hover:underline"
                                                        >
                                                            {slug}
                                                        </a>
                                                    </td>
                                                    <td class="px-6 py-4">
                                                        <span class="text-gray-600 dark:text-gray-400 text-sm font-mono truncate block max-w-md">
                                                            {human_key}
                                                        </span>
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                                                        {format_date(&created)}
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-right">
                                                        <a
                                                            href=href2
                                                            class="text-primary-600 dark:text-primary-400 hover:text-primary-800 dark:hover:text-primary-300 text-sm font-medium"
                                                        >
                                                            "View Agents →"
                                                        </a>
                                                    </td>
                                                </tr>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </tbody>
                                </table>
                            </div>
                        }.into_any()
                    }
                }
            }}
        </div>
    }
}

/// Format date string for display.
fn format_date(date_str: &str) -> String {
    // Simple date formatting - just show the date part
    if date_str.is_empty() {
        return "—".to_string();
    }
    // Try to extract just the date part (YYYY-MM-DD)
    date_str.split('T').next().unwrap_or(date_str).to_string()
}
