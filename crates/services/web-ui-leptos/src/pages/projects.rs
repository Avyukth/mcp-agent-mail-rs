//! Projects page - list and create projects.
//! Will be implemented in task mcp-agent-mail-rs-cfu.

use leptos::prelude::*;

/// Projects page component.
#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div class="flex items-center justify-between">
                <h1 class="text-2xl font-bold text-gray-900 dark:text-white">"Projects"</h1>
                <button class="bg-primary-600 hover:bg-primary-700 text-white px-4 py-2 rounded-lg font-medium transition-colors">
                    "+ New Project"
                </button>
            </div>

            // Placeholder table
            <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden">
                <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
                    <thead class="bg-gray-50 dark:bg-gray-700">
                        <tr>
                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">"Name"</th>
                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">"Slug"</th>
                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">"Agents"</th>
                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase tracking-wider">"Created"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-gray-200 dark:divide-gray-700">
                        <tr>
                            <td class="px-6 py-4 text-gray-500 dark:text-gray-400 text-sm" colspan="4">
                                "Loading projects..."
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>

            <PlaceholderNotice task_id="mcp-agent-mail-rs-cfu" />
        </div>
    }
}

/// Placeholder notice component.
#[component]
fn PlaceholderNotice(task_id: &'static str) -> impl IntoView {
    view! {
        <div class="bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg p-4">
            <p class="text-yellow-800 dark:text-yellow-200 text-sm">
                "🚧 This is a placeholder page. Implementation coming in task "
                <code class="font-mono bg-yellow-100 dark:bg-yellow-800 px-1 rounded">{task_id}</code>
            </p>
        </div>
    }
}
