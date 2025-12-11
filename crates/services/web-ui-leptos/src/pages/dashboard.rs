//! Dashboard page - main landing page with health status and quick stats.
//! Will be implemented in task mcp-agent-mail-rs-d8j.

use leptos::prelude::*;

/// Dashboard page component.
#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <div class="flex items-center justify-between">
                <h1 class="text-2xl font-bold text-gray-900 dark:text-white">"Dashboard"</h1>
            </div>

            // Placeholder cards - will be replaced with real data
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                <PlaceholderCard title="Health Status" icon="🏥" value="Loading..." />
                <PlaceholderCard title="Projects" icon="📁" value="—" />
                <PlaceholderCard title="Agents" icon="🤖" value="—" />
                <PlaceholderCard title="Messages" icon="✉️" value="—" />
                <PlaceholderCard title="Unread" icon="📬" value="—" />
                <PlaceholderCard title="Active Reservations" icon="🔒" value="—" />
            </div>

            <div class="bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg p-4">
                <p class="text-yellow-800 dark:text-yellow-200 text-sm">
                    "🚧 This is a placeholder page. Implementation coming in task "
                    <code class="font-mono bg-yellow-100 dark:bg-yellow-800 px-1 rounded">"mcp-agent-mail-rs-d8j"</code>
                </p>
            </div>
        </div>
    }
}

/// Placeholder card component for dashboard stats.
#[component]
fn PlaceholderCard(title: &'static str, icon: &'static str, value: &'static str) -> impl IntoView {
    view! {
        <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6">
            <div class="flex items-center justify-between">
                <span class="text-2xl">{icon}</span>
                <span class="text-xs text-gray-500 dark:text-gray-400 uppercase tracking-wider">{title}</span>
            </div>
            <div class="mt-4">
                <span class="text-3xl font-bold text-gray-900 dark:text-white">{value}</span>
            </div>
        </div>
    }
}
