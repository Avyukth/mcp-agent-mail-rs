<script lang="ts">
	import { onMount } from 'svelte';
	import { isStaticMode, dataProvider } from '$lib/data';
	import { AlertTriangle, Archive, X } from 'lucide-svelte';

	// ============================================================================
	// State
	// ============================================================================

	let visible = $state(true);
	let exportedAt = $state<string | null>(null);

	// ============================================================================
	// Mode Detection
	// ============================================================================

	const staticMode = isStaticMode();

	// ============================================================================
	// Lifecycle
	// ============================================================================

	onMount(async () => {
		if (staticMode) {
			try {
				const meta = await dataProvider.getMeta();
				exportedAt = meta.exportedAt;
			} catch {
				exportedAt = null;
			}
		}
	});

	// ============================================================================
	// Helpers
	// ============================================================================

	function dismiss() {
		visible = false;
	}

	function formatDate(dateStr: string): string {
		try {
			const date = new Date(dateStr);
			return date.toLocaleDateString('en-US', {
				year: 'numeric',
				month: 'short',
				day: 'numeric',
				hour: '2-digit',
				minute: '2-digit'
			});
		} catch {
			return dateStr;
		}
	}
</script>

{#if staticMode && visible}
	<div
		class="bg-amber-50 dark:bg-amber-950/50 border-b border-amber-200 dark:border-amber-800 px-4 py-2"
		role="banner"
		aria-label="Demo mode notice"
	>
		<div class="max-w-7xl mx-auto flex items-center justify-between gap-4">
			<div class="flex items-center gap-3 text-sm text-amber-800 dark:text-amber-200">
				<div class="flex items-center gap-2">
					<Archive class="w-4 h-4 shrink-0" />
					<span class="font-medium">Static Archive Mode</span>
				</div>
				<span class="hidden sm:inline text-amber-600 dark:text-amber-400">|</span>
				<span class="hidden sm:inline text-amber-600 dark:text-amber-400">
					This is a read-only snapshot of the mail archive.
				</span>
				{#if exportedAt}
					<span class="hidden md:inline text-amber-600 dark:text-amber-400">|</span>
					<span class="hidden md:inline text-amber-600 dark:text-amber-400">
						Exported: {formatDate(exportedAt)}
					</span>
				{/if}
			</div>
			<div class="flex items-center gap-2">
				<a
					href="/data/archive.zip"
					download
					class="hidden sm:inline-flex items-center gap-1 text-xs px-2 py-1 bg-amber-100 dark:bg-amber-900/50 text-amber-700 dark:text-amber-300 rounded hover:bg-amber-200 dark:hover:bg-amber-900 transition-colors"
				>
					Download Archive
				</a>
				<button
					type="button"
					onclick={dismiss}
					class="p-1 text-amber-600 dark:text-amber-400 hover:text-amber-800 dark:hover:text-amber-200 transition-colors"
					aria-label="Dismiss banner"
				>
					<X class="w-4 h-4" />
				</button>
			</div>
		</div>
	</div>
{/if}

{#if staticMode}
	<!-- Hidden features notice in static mode -->
	<style>
		/* Hide compose/send buttons in static mode */
		:global([data-action='compose']),
		:global([data-action='send']),
		:global([data-action='reply']),
		:global([data-action='delete']) {
			display: none !important;
		}
	</style>
{/if}
