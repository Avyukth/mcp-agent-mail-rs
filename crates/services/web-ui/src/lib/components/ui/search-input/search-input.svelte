<script lang="ts">
	import Search from 'lucide-svelte/icons/search';
	import X from 'lucide-svelte/icons/x';
	import { cn } from '$lib/utils';

	interface Props {
		value?: string;
		placeholder?: string;
		class?: string;
		onchange?: (value: string) => void;
	}

	let {
		value = $bindable(''),
		placeholder = 'Search...',
		class: className = '',
		onchange
	}: Props = $props();

	function clear() {
		value = '';
		onchange?.('');
	}

	function handleInput(e: Event) {
		const target = e.target as HTMLInputElement;
		value = target.value;
		onchange?.(target.value);
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			clear();
		}
	}
</script>

<div class={cn('relative', className)}>
	<Search class="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground pointer-events-none" />
	<input
		type="text"
		{value}
		{placeholder}
		oninput={handleInput}
		onkeydown={handleKeydown}
		class="w-full pl-10 pr-10 h-10 rounded-md border border-input bg-background text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
	/>
	{#if value}
		<button
			type="button"
			class="absolute right-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground hover:text-foreground transition-colors"
			onclick={clear}
			data-testid="clear-search"
			aria-label="Clear search"
		>
			<X class="h-4 w-4" />
		</button>
	{/if}
</div>
