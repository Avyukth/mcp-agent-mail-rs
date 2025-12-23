<script lang="ts">
	import '../app.css';
	import { ModeWatcher } from 'mode-watcher';
	import { Toaster } from '$lib/components/ui/sonner/index.js';
	import { AppSidebar } from '$lib/components/layout/index.js';
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();

	// TODO: Fetch actual unread count from API
	let unreadCount = $state(3);
</script>

<ModeWatcher />
<Toaster />

<div class="min-h-screen flex">
	<!-- Sidebar (handles both mobile sheet trigger and desktop sidebar) -->
	<AppSidebar {unreadCount} />

	<!-- Main content -->
	<div class="flex-1 flex flex-col min-w-0">
		<main class="flex-1 p-4 md:p-6 bg-background overflow-auto">
			{@render children()}
		</main>
	</div>
</div>
