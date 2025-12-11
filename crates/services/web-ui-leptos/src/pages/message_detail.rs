//! Message detail page - view message thread and reply.
//! Will be implemented in task mcp-agent-mail-rs-mi4.

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

/// Message detail page component.
#[component]
pub fn MessageDetail() -> impl IntoView {
    let params = use_params_map();
    let message_id = move || params.read().get("id").unwrap_or_default();

    view! {
        <div class="space-y-6">
            <div class="flex items-center space-x-4">
                <a href="/inbox" class="text-primary-600 hover:text-primary-500 dark:text-primary-400">
                    "← Back to Inbox"
                </a>
            </div>

            // Message header
            <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6">
                <div class="flex items-start justify-between">
                    <div>
                        <h1 class="text-xl font-bold text-gray-900 dark:text-white">"Message Thread"</h1>
                        <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
                            "ID: " <code class="font-mono">{message_id}</code>
                        </p>
                    </div>
                    <div class="flex space-x-2">
                        <button class="px-3 py-1 text-sm bg-green-100 dark:bg-green-900 text-green-700 dark:text-green-300 rounded-lg">
                            "✓ ACK"
                        </button>
                        <button class="px-3 py-1 text-sm bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300 rounded-lg">
                            "↩ Reply"
                        </button>
                    </div>
                </div>
            </div>

            // Message content
            <div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 p-6">
                <div class="flex items-center space-x-4 mb-4">
                    <div class="w-10 h-10 bg-gray-200 dark:bg-gray-700 rounded-full flex items-center justify-center">
                        <span class="text-lg">"🤖"</span>
                    </div>
                    <div>
                        <p class="font-medium text-gray-900 dark:text-white">"sender-agent"</p>
                        <p class="text-sm text-gray-500 dark:text-gray-400">"to: recipient-agent"</p>
                    </div>
                </div>
                <div class="prose dark:prose-invert max-w-none">
                    <p class="text-gray-600 dark:text-gray-300">"Loading message content..."</p>
                </div>
            </div>

            <div class="bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg p-4">
                <p class="text-yellow-800 dark:text-yellow-200 text-sm">
                    "🚧 This is a placeholder page. Implementation coming in task "
                    <code class="font-mono bg-yellow-100 dark:bg-yellow-800 px-1 rounded">"mcp-agent-mail-rs-mi4"</code>
                </p>
            </div>
        </div>
    }
}
