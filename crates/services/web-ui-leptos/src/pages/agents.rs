//! Agents page - list all agents across projects.
//! Will be implemented in task mcp-agent-mail-rs-drh.

use leptos::prelude::*;

/// Agents page component.
#[component]
pub fn Agents() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div class="flex items-center justify-between">
                <h1 class="text-2xl font-bold text-gray-900 dark:text-white">"Agents"</h1>
                <div class="flex items-center space-x-4">
                    <input
                        type="search"
                        placeholder="Search agents..."
                        class="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white placeholder-gray-500 focus:ring-2 focus:ring-primary-500 focus:border-transparent"
                    />
                </div>
            </div>

            // Agent cards grid
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                <AgentCardPlaceholder />
                <AgentCardPlaceholder />
                <AgentCardPlaceholder />
            </div>

            <div class="bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg p-4">
                <p class="text-yellow-800 dark:text-yellow-200 text-sm">
                    "🚧 This is a placeholder page. Implementation coming in task "
                    <code class="font-mono bg-yellow-100 dark:bg-yellow-800 px-1 rounded">"mcp-agent-mail-rs-drh"</code>
                </p>
            </div>
        </div>
    }
}

/// Placeholder agent card.
#[component]
fn AgentCardPlaceholder() -> impl IntoView {
    view! {
        <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6 animate-pulse">
            <div class="flex items-center space-x-4">
                <div class="w-12 h-12 bg-gray-200 dark:bg-gray-700 rounded-full"></div>
                <div class="flex-1">
                    <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-3/4 mb-2"></div>
                    <div class="h-3 bg-gray-200 dark:bg-gray-700 rounded w-1/2"></div>
                </div>
            </div>
        </div>
    }
}
