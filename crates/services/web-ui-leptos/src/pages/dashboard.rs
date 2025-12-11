//! Dashboard page - main landing page with health status and quick stats.

use leptos::prelude::*;
use crate::api::client::{self, Project};

/// Dashboard page component with health check and project stats.
#[component]
pub fn Dashboard() -> impl IntoView {
    // Use RwSignals for simple state management
    let health_status = RwSignal::new(String::from("checking..."));
    let health_error = RwSignal::new(Option::<String>::None);
    let projects = RwSignal::new(Vec::<Project>::new());
    let projects_loaded = RwSignal::new(false);

    // Load data on mount using spawn_local
    Effect::new(move |_| {
        // Spawn async task to load health
        leptos::task::spawn_local(async move {
            match client::check_health().await {
                Ok(h) => {
                    health_status.set(h.status);
                    health_error.set(None);
                }
                Err(e) => {
                    health_status.set("offline".to_string());
                    health_error.set(Some(e.message));
                }
            }
        });

        // Spawn async task to load projects
        leptos::task::spawn_local(async move {
            match client::get_projects().await {
                Ok(p) => {
                    projects.set(p);
                    projects_loaded.set(true);
                }
                Err(_) => {
                    projects_loaded.set(true);
                }
            }
        });
    });

    view! {
        <div class="space-y-6">
            <div>
                <h1 class="text-2xl font-bold text-gray-900 dark:text-white">"Dashboard"</h1>
                <p class="text-gray-600 dark:text-gray-400">"Welcome to MCP Agent Mail"</p>
            </div>

            // Status Cards
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                // Backend Status Card
                <div class="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
                    <div class="flex items-center gap-3">
                        <div class={move || {
                            let status = health_status.get();
                            match status.as_str() {
                                "ok" => "w-3 h-3 rounded-full bg-green-500",
                                "checking..." => "w-3 h-3 rounded-full bg-yellow-500 animate-pulse",
                                _ => "w-3 h-3 rounded-full bg-red-500",
                            }
                        }}></div>
                        <h3 class="font-semibold text-gray-900 dark:text-white">"Backend Status"</h3>
                    </div>
                    <p class="mt-2 text-2xl font-bold text-gray-700 dark:text-gray-300 capitalize">
                        {move || health_status.get()}
                    </p>
                </div>

                // Projects Count Card
                <div class="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
                    <div class="flex items-center gap-3">
                        <span class="text-2xl">"📁"</span>
                        <h3 class="font-semibold text-gray-900 dark:text-white">"Projects"</h3>
                    </div>
                    <p class="mt-2 text-2xl font-bold text-primary-600 dark:text-primary-400">
                        {move || {
                            if projects_loaded.get() {
                                projects.get().len().to_string()
                            } else {
                                "—".to_string()
                            }
                        }}
                    </p>
                </div>

                // Quick Actions Card
                <div class="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
                    <h3 class="font-semibold text-gray-900 dark:text-white mb-3">"Quick Actions"</h3>
                    <div class="space-y-2">
                        <a
                            href="/projects"
                            class="block px-4 py-2 bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300 rounded-lg hover:bg-primary-200 dark:hover:bg-primary-800 transition-colors"
                        >
                            "View Projects →"
                        </a>
                        <a
                            href="/inbox"
                            class="block px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
                        >
                            "Check Inbox →"
                        </a>
                    </div>
                </div>
            </div>

            // Error display
            {move || {
                health_error.get().map(|e| view! {
                    <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-xl p-4">
                        <p class="text-red-700 dark:text-red-400">
                            <strong>"Error: "</strong> {e}
                        </p>
                        <p class="text-sm text-red-600 dark:text-red-500 mt-1">
                            "Make sure the backend is running on port 8765"
                        </p>
                    </div>
                })
            }}

            // Recent Projects List
            {move || {
                let project_list = projects.get();
                if project_list.is_empty() {
                    None
                } else {
                    Some(view! {
                        <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700">
                            <div class="p-4 border-b border-gray-200 dark:border-gray-700">
                                <h2 class="text-lg font-semibold text-gray-900 dark:text-white">"Recent Projects"</h2>
                            </div>
                            <ul class="divide-y divide-gray-200 dark:divide-gray-700">
                                {project_list.into_iter().take(5).map(|project| {
                                    let href = format!("/projects/{}", project.slug);
                                    let slug = project.slug.clone();
                                    let name = project.name.as_ref().unwrap_or(&project.slug).clone();
                                    view! {
                                        <li class="p-4 hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors">
                                            <a href=href class="block">
                                                <p class="font-medium text-gray-900 dark:text-white">{slug}</p>
                                                <p class="text-sm text-gray-500 dark:text-gray-400 truncate">{name}</p>
                                            </a>
                                        </li>
                                    }
                                }).collect::<Vec<_>>()}
                            </ul>
                        </div>
                    })
                }
            }}
        </div>
    }
}
