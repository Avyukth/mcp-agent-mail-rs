//! Project detail page - view project info and manage agents.
//! Will be implemented in task mcp-agent-mail-rs-m67.

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

/// Project detail page component.
#[component]
pub fn ProjectDetail() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.read().get("slug").unwrap_or_default();

    view! {
        <div class="space-y-6">
            <div class="flex items-center space-x-4">
                <a href="/projects" class="text-primary-600 hover:text-primary-500 dark:text-primary-400">
                    "← Back to Projects"
                </a>
            </div>

            <div class="flex items-center justify-between">
                <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
                    "Project: " {slug}
                </h1>
            </div>

            // Project info card
            <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6">
                <h2 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">"Project Details"</h2>
                <dl class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div>
                        <dt class="text-sm text-gray-500 dark:text-gray-400">"Slug"</dt>
                        <dd class="text-gray-900 dark:text-white font-mono">{slug}</dd>
                    </div>
                    <div>
                        <dt class="text-sm text-gray-500 dark:text-gray-400">"Agents"</dt>
                        <dd class="text-gray-900 dark:text-white">"—"</dd>
                    </div>
                </dl>
            </div>

            // Agents section
            <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6">
                <div class="flex items-center justify-between mb-4">
                    <h2 class="text-lg font-semibold text-gray-900 dark:text-white">"Registered Agents"</h2>
                    <button class="text-primary-600 hover:text-primary-500 dark:text-primary-400 text-sm font-medium">
                        "+ Register Agent"
                    </button>
                </div>
                <p class="text-gray-500 dark:text-gray-400 text-sm">"Loading agents..."</p>
            </div>

            <div class="bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg p-4">
                <p class="text-yellow-800 dark:text-yellow-200 text-sm">
                    "🚧 This is a placeholder page. Implementation coming in task "
                    <code class="font-mono bg-yellow-100 dark:bg-yellow-800 px-1 rounded">"mcp-agent-mail-rs-m67"</code>
                </p>
            </div>
        </div>
    }
}
