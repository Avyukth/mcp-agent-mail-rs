//! Main layout component with navigation.

use leptos::prelude::*;
use leptos_router::components::Outlet;

/// Main layout wrapper with navigation and content outlet.
/// Will be expanded in task mcp-agent-mail-rs-ldr.
#[component]
pub fn Layout() -> impl IntoView {
    // Dark mode signal - will be persisted to localStorage later
    let (dark_mode, set_dark_mode) = signal(false);

    // Toggle dark mode class on document
    Effect::new(move |_| {
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if let Some(html) = document.document_element() {
                if dark_mode.get() {
                    let _ = html.class_list().add_1("dark");
                } else {
                    let _ = html.class_list().remove_1("dark");
                }
            }
        }
    });

    view! {
        <div class="min-h-screen bg-gray-50 dark:bg-gray-900 transition-colors">
            // Navigation header
            <nav class="bg-white dark:bg-gray-800 shadow-sm border-b border-gray-200 dark:border-gray-700">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="flex justify-between h-16 items-center">
                        // Logo / Brand
                        <div class="flex items-center space-x-8">
                            <a href="/" class="text-xl font-bold text-primary-600 dark:text-primary-400">
                                "📧 MCP Agent Mail"
                            </a>
                            // Navigation links
                            <div class="hidden md:flex space-x-4">
                                <NavLink href="/" label="Dashboard" />
                                <NavLink href="/projects" label="Projects" />
                                <NavLink href="/agents" label="Agents" />
                                <NavLink href="/inbox" label="Inbox" />
                            </div>
                        </div>
                        
                        // Dark mode toggle
                        <button
                            on:click=move |_| set_dark_mode.update(|v| *v = !*v)
                            class="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
                            title="Toggle dark mode"
                        >
                            <span class="text-xl">
                                {move || if dark_mode.get() { "☀️" } else { "🌙" }}
                            </span>
                        </button>
                    </div>
                </div>
            </nav>

            // Main content area
            <main class="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
                <Outlet />
            </main>

            // Footer
            <footer class="border-t border-gray-200 dark:border-gray-700 mt-auto">
                <div class="max-w-7xl mx-auto py-4 px-4 text-center text-sm text-gray-500 dark:text-gray-400">
                    "MCP Agent Mail • Rust/WASM Edition"
                </div>
            </footer>
        </div>
    }
}

/// Navigation link component.
#[component]
fn NavLink(href: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <a
            href=href
            class="text-gray-600 dark:text-gray-300 hover:text-primary-600 dark:hover:text-primary-400 px-3 py-2 rounded-md text-sm font-medium transition-colors"
        >
            {label}
        </a>
    }
}
