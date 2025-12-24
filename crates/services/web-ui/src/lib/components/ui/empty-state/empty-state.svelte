<script lang="ts">
	import type { Snippet } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';

	interface Props {
		title: string;
		description?: string;
		actionLabel?: string;
		onAction?: () => void;
		icon?: Snippet;
		class?: string;
	}

	let {
		title,
		description,
		actionLabel,
		onAction,
		icon,
		class: className = ''
	}: Props = $props();
</script>

<div
	data-testid="empty-state"
	class={cn(
		'flex flex-col items-center justify-center py-12 px-4 text-center',
		className
	)}
>
	{#if icon}
		<div class="mb-4 text-muted-foreground" aria-hidden="true">
			{@render icon()}
		</div>
	{/if}

	<h3 class="text-lg font-semibold text-foreground mb-2">
		{title}
	</h3>

	{#if description}
		<p class="text-sm text-muted-foreground max-w-md mb-6">
			{description}
		</p>
	{/if}

	{#if actionLabel && onAction}
		<Button onclick={onAction}>
			{actionLabel}
		</Button>
	{/if}
</div>
