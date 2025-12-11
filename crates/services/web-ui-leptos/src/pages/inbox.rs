//! Inbox page - view messages with cascading project/agent selects.
//! Will be implemented in task mcp-agent-mail-rs-ezy.

use leptos::prelude::*;

/// Inbox page component.
#[component]
pub fn Inbox() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div class="flex items-center justify-between">
                <h1 class="text-2xl font-bold text-gray-900 dark:text-white">"Inbox"</h1>
                <button class="bg-primary-600 hover:bg-primary-700 text-white px-4 py-2 rounded-lg font-medium transition-colors">
                    "✉️ Compose"
                </button>
            </div>

            // Filters
            <div class="flex items-center space-x-4">
                <select class="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white">
                    <option>"All Projects"</option>
                </select>
                <select class="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white">
                    <option>"All Agents"</option>
                </select>
            </div>

            // Message list
            <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 divide-y divide-gray-200 dark:divide-gray-700">
                <MessageRowPlaceholder />
                <MessageRowPlaceholder />
                <MessageRowPlaceholder />
            </div>

            <div class="bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg p-4">
                <p class="text-yellow-800 dark:text-yellow-200 text-sm">
                    "🚧 This is a placeholder page. Implementation coming in task "
                    <code class="font-mono bg-yellow-100 dark:bg-yellow-800 px-1 rounded">"mcp-agent-mail-rs-ezy"</code>
                </p>
            </div>
        </div>
    }
}

/// Placeholder message row.
#[component]
fn MessageRowPlaceholder() -> impl IntoView {
    view! {
        <div class="p-4 hover:bg-gray-50 dark:hover:bg-gray-700 cursor-pointer animate-pulse">
            <div class="flex items-center space-x-4">
                <div class="w-10 h-10 bg-gray-200 dark:bg-gray-700 rounded-full"></div>
                <div class="flex-1">
                    <div class="h-4 bg-gray-200 dark:bg-gray-700 rounded w-1/4 mb-2"></div>
                    <div class="h-3 bg-gray-200 dark:bg-gray-700 rounded w-3/4"></div>
                </div>
                <div class="h-3 bg-gray-200 dark:bg-gray-700 rounded w-16"></div>
            </div>
        </div>
    }
}
