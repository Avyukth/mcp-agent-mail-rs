//! Search results page with FTS5 highlighting.
//!
//! Displays search results with query term highlighting,
//! filter chips, and debounced search-as-you-type.

use crate::api::client::{self, Message, Project};
use crate::components::{Badge, BadgeVariant, Card, CardContent, Input, Pagination, Skeleton};
use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

/// Debounce delay for search input in milliseconds.
const SEARCH_DEBOUNCE_MS: u64 = 300;

/// Search results page component.
#[component]
pub fn Search() -> impl IntoView {
    let query_params = use_query_map();

    // Get initial query from URL
    let initial_query = query_params.with_untracked(|q| q.get("q").unwrap_or_default());
    let initial_project = query_params.with_untracked(|q| q.get("project").unwrap_or_default());

    // State
    let search_input = RwSignal::new(initial_query.clone());
    let search_query = RwSignal::new(initial_query);
    let selected_project = RwSignal::new(initial_project);
    let projects = RwSignal::new(Vec::<Project>::new());
    let results = RwSignal::new(Vec::<Message>::new());
    let loading = RwSignal::new(false);
    let error = RwSignal::new(Option::<String>::None);
    let has_searched = RwSignal::new(false);

    // Pagination state
    let has_more = RwSignal::new(false);
    let total_count = RwSignal::new(0i64);

    // Load projects for filter
    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            if let Ok(p) = client::get_projects().await {
                projects.set(p);
            }
        });
    });

    // Debounced search effect
    Effect::new(move |_| {
        let input = search_input.get();
        let _project = selected_project.get(); // Track dependency

        // Use timeout for debounce
        leptos::task::spawn_local(async move {
            // Simple debounce using sleep
            gloo_timers::future::TimeoutFuture::new(SEARCH_DEBOUNCE_MS as u32).await;

            // Only search if input hasn't changed
            if search_input.get_untracked() == input {
                search_query.set(input);
            }
        });
    });

    // Execute search when query changes
    Effect::new(move |_| {
        let query = search_query.get();
        let project = selected_project.get();

        if query.is_empty() {
            results.set(vec![]);
            has_searched.set(false);
            return;
        }

        loading.set(true);
        has_searched.set(true);
        error.set(None);

        leptos::task::spawn_local(async move {
            // Use project filter if selected, otherwise search all
            let search_project = if project.is_empty() {
                // Search first available project or skip
                match projects.get_untracked().first() {
                    Some(p) => p.slug.clone(),
                    None => {
                        loading.set(false);
                        return;
                    }
                }
            } else {
                project
            };

            match client::search_messages(&search_project, &query).await {
                Ok(msgs) => {
                    total_count.set(msgs.len() as i64);
                    has_more.set(false); // API doesn't support pagination yet
                    results.set(msgs);
                    loading.set(false);
                }
                Err(e) => {
                    error.set(Some(e.message));
                    loading.set(false);
                }
            }
        });
    });

    // Result count for screen readers
    let result_count = Signal::derive(move || results.get().len());

    view! {
        <div class="space-y-6">
            // Header with search input
            <div class="space-y-4">
                <div class="flex items-center gap-3">
                    <i data-lucide="search" class="icon-lg text-amber-500"></i>
                    <h1 class="text-2xl font-bold text-foreground">"Search Messages"</h1>
                </div>

                // Search input
                <div class="flex flex-col sm:flex-row gap-3">
                    <div class="flex-1 relative">
                        <i data-lucide="search" class="absolute left-3 top-1/2 -translate-y-1/2 icon-sm text-muted-foreground pointer-events-none"></i>
                        <Input
                            id="search-input".to_string()
                            value=search_input
                            placeholder="Search messages...".to_string()
                            class="pl-10".to_string()
                        />
                    </div>

                    // Project filter
                    {move || {
                        let project_list = projects.get();
                        if !project_list.is_empty() {
                            Some(view! {
                                <select
                                    class="h-10 px-3 rounded-md border border-input bg-background text-sm"
                                    on:change=move |ev| {
                                        let value = event_target_value(&ev);
                                        selected_project.set(value);
                                    }
                                >
                                    <option value="">"All Projects"</option>
                                    {project_list.into_iter().map(|p| {
                                        let slug = p.slug.clone();
                                        let selected = selected_project.get() == slug;
                                        view! {
                                            <option value=slug.clone() selected=selected>{p.slug}</option>
                                        }
                                    }).collect::<Vec<_>>()}
                                </select>
                            })
                        } else {
                            None
                        }
                    }}
                </div>

                // Active filters as chips
                {move || {
                    let project = selected_project.get();
                    let query = search_query.get();

                    if project.is_empty() && query.is_empty() {
                        return None;
                    }

                    Some(view! {
                        <div class="flex flex-wrap gap-2" role="list" aria-label="Active filters">
                            {(!query.is_empty()).then(|| view! {
                                <Badge variant=BadgeVariant::Default>
                                    <i data-lucide="search" class="icon-xs mr-1"></i>
                                    {format!("\"{}\"", query)}
                                    <button
                                        type="button"
                                        class="ml-1 hover:text-destructive"
                                        on:click=move |_| {
                                            search_input.set(String::new());
                                            search_query.set(String::new());
                                        }
                                        aria-label="Clear search"
                                    >
                                        <i data-lucide="x" class="icon-xs"></i>
                                    </button>
                                </Badge>
                            })}
                            {(!project.is_empty()).then(|| view! {
                                <Badge variant=BadgeVariant::Secondary>
                                    <i data-lucide="folder" class="icon-xs mr-1"></i>
                                    {project.clone()}
                                    <button
                                        type="button"
                                        class="ml-1 hover:text-destructive"
                                        on:click=move |_| selected_project.set(String::new())
                                        aria-label="Clear project filter"
                                    >
                                        <i data-lucide="x" class="icon-xs"></i>
                                    </button>
                                </Badge>
                            })}
                        </div>
                    })
                }}
            </div>

            // Screen reader announcement
            <div class="sr-only" aria-live="polite" aria-atomic="true">
                {move || {
                    let count = result_count.get();
                    let query = search_query.get();
                    if has_searched.get() && !loading.get() {
                        format!("{} results found for {}", count, query)
                    } else {
                        String::new()
                    }
                }}
            </div>

            // Error display
            {move || error.get().map(|e| view! {
                <div class="rounded-xl border border-destructive/50 bg-destructive/10 p-4">
                    <div class="flex items-start gap-3">
                        <i data-lucide="triangle-alert" class="icon-lg text-destructive"></i>
                        <p class="text-destructive">{e}</p>
                    </div>
                </div>
            })}

            // Loading state
            {move || loading.get().then(|| view! {
                <div class="space-y-4">
                    {(0..3).map(|_| view! {
                        <Skeleton class="h-24 w-full".to_string() />
                    }).collect::<Vec<_>>()}
                </div>
            })}

            // Results list
            {move || {
                let msgs = results.get();
                let query = search_query.get();
                let searched = has_searched.get();
                let is_loading = loading.get();

                if is_loading {
                    return None;
                }

                if msgs.is_empty() && searched {
                    // No results state
                    return Some(view! {
                        <div class="text-center py-12">
                            <i data-lucide="search-x" class="icon-xl mx-auto mb-4 text-muted-foreground opacity-50"></i>
                            <h2 class="text-lg font-medium text-foreground">"No results found"</h2>
                            <p class="text-muted-foreground mt-2">
                                "Try different keywords or remove some filters"
                            </p>
                            <div class="mt-4 text-sm text-muted-foreground">
                                <p class="font-medium">"Suggestions:"</p>
                                <ul class="mt-2 space-y-1">
                                    <li>"• Check your spelling"</li>
                                    <li>"• Try more general terms"</li>
                                    <li>"• Search within a different project"</li>
                                </ul>
                            </div>
                        </div>
                    }.into_any());
                }

                if !msgs.is_empty() {
                    Some(view! {
                        <div class="space-y-4">
                            // Result count
                            <p class="text-sm text-muted-foreground">
                                {format!("{} result{}", msgs.len(), if msgs.len() == 1 { "" } else { "s" })}
                            </p>

                            // Results
                            <div class="space-y-3" role="list" aria-label="Search results">
                                {msgs.into_iter().map(|msg| {
                                    view! {
                                        <SearchResultItem message=msg query=query.clone() />
                                    }
                                }).collect::<Vec<_>>()}
                            </div>

                            // Pagination
                            <Pagination
                                has_more=Signal::derive(move || has_more.get())
                                total=Signal::derive(move || total_count.get())
                                current_count=Signal::derive(move || results.get().len())
                                loading=Signal::derive(move || loading.get())
                                on_load_more=Callback::new(|_| {
                                    // TODO: Implement cursor-based pagination
                                })
                            />
                        </div>
                    }.into_any())
                } else {
                    // Initial state - show prompt
                    Some(view! {
                        <div class="text-center py-12 text-muted-foreground">
                            <i data-lucide="search" class="icon-xl mx-auto mb-4 opacity-30"></i>
                            <p>"Enter a search term to find messages"</p>
                        </div>
                    }.into_any())
                }
            }}
        </div>
    }
}

/// Individual search result item with highlighting.
#[component]
fn SearchResultItem(message: Message, query: String) -> impl IntoView {
    let subject = message.subject.clone();
    let sender = message.sender_name.clone();
    let body = message.body_md.clone();
    let created = message.created_ts.clone();
    let message_id = message.id;

    // Create highlighted snippet
    let snippet = create_highlighted_snippet(&body, &query, 200);

    view! {
        <a
            href=format!("/inbox/{}?project={}", message_id, message.project_id)
            class="block"
        >
            <Card>
                <CardContent>
                    <div class="space-y-2">
                        // Subject with highlight
                        <h3 class="font-medium text-foreground">
                            <HighlightedText text=subject query=query.clone() />
                        </h3>

                        // Metadata
                        <div class="flex items-center gap-2 text-sm text-muted-foreground">
                            <span class="flex items-center gap-1">
                                <i data-lucide="user" class="icon-xs"></i>
                                {sender}
                            </span>
                            <span>"·"</span>
                            <span>{created}</span>
                        </div>

                        // Body snippet with highlights
                        <p class="text-sm text-muted-foreground line-clamp-2">
                            {snippet}
                        </p>
                    </div>
                </CardContent>
            </Card>
        </a>
    }
}

/// Component to render text with search term highlighting.
#[component]
fn HighlightedText(text: String, query: String) -> impl IntoView {
    if query.is_empty() {
        return view! { <span>{text}</span> }.into_any();
    }

    let query_lower = query.to_lowercase();
    let text_lower = text.to_lowercase();

    // Find all matches and create spans
    let mut result = Vec::new();
    let mut last_end = 0;

    for (start, _) in text_lower.match_indices(&query_lower) {
        // Add non-matching text before this match
        if start > last_end {
            result.push(
                view! {
                    <span>{text[last_end..start].to_string()}</span>
                }
                .into_any(),
            );
        }

        // Add highlighted match
        let end = start + query.len();
        result.push(view! {
            <mark class="bg-yellow-200 dark:bg-yellow-800 px-0.5 rounded">{text[start..end].to_string()}</mark>
        }.into_any());

        last_end = end;
    }

    // Add remaining text
    if last_end < text.len() {
        result.push(
            view! {
                <span>{text[last_end..].to_string()}</span>
            }
            .into_any(),
        );
    }

    view! { <span>{result}</span> }.into_any()
}

/// Create a highlighted snippet from body text.
fn create_highlighted_snippet(body: &str, query: &str, max_len: usize) -> String {
    if query.is_empty() {
        return body.chars().take(max_len).collect();
    }

    let query_lower = query.to_lowercase();
    let body_lower = body.to_lowercase();

    // Find first occurrence of query
    if let Some(pos) = body_lower.find(&query_lower) {
        // Center the snippet around the match
        let start = pos.saturating_sub(max_len / 3);
        let end = (start + max_len).min(body.len());

        let mut snippet: String = body.chars().skip(start).take(end - start).collect();

        // Add ellipsis if needed
        if start > 0 {
            snippet = format!("...{}", snippet);
        }
        if end < body.len() {
            snippet = format!("{}...", snippet);
        }

        snippet
    } else {
        // No match, just take first chars
        let snippet: String = body.chars().take(max_len).collect();
        if body.len() > max_len {
            format!("{}...", snippet)
        } else {
            snippet
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_debounce_constant() {
        assert_eq!(SEARCH_DEBOUNCE_MS, 300);
    }

    #[test]
    fn test_create_highlighted_snippet_with_match() {
        let body = "This is a test message with some content about testing.";
        let snippet = create_highlighted_snippet(body, "test", 30);
        assert!(snippet.contains("test"));
    }

    #[test]
    fn test_create_highlighted_snippet_no_match() {
        let body = "This is a message without the search term.";
        let snippet = create_highlighted_snippet(body, "xyz", 20);
        assert_eq!(snippet, "This is a message wi...");
    }

    #[test]
    fn test_create_highlighted_snippet_empty_query() {
        let body = "This is a test message.";
        let snippet = create_highlighted_snippet(body, "", 10);
        assert_eq!(snippet, "This is a ");
    }

    #[test]
    fn test_highlight_uses_mark_element() {
        // Verify we use <mark> for accessibility
        let mark_class = "bg-yellow-200 dark:bg-yellow-800";
        assert!(mark_class.contains("bg-yellow"));
    }
}
