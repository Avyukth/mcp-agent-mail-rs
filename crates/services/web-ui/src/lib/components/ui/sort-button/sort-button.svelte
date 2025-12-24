<script lang="ts">
	import ArrowUp from 'lucide-svelte/icons/arrow-up';
	import ArrowDown from 'lucide-svelte/icons/arrow-down';
	import ArrowUpDown from 'lucide-svelte/icons/arrow-up-down';
	import { cn } from '$lib/utils';

	type SortDirection = 'asc' | 'desc' | null;

	interface Props {
		field: string;
		label: string;
		currentField?: string;
		currentDirection?: SortDirection;
		onSort?: (field: string, direction: SortDirection) => void;
		class?: string;
	}

	let {
		field,
		label,
		currentField,
		currentDirection = null,
		onSort,
		class: className = ''
	}: Props = $props();

	let isActive = $derived(currentField === field);
	let direction = $derived(isActive ? currentDirection : null);

	// Build accessible label for screen readers
	let ariaLabel = $derived(() => {
		if (!isActive || !direction) {
			return `Sort by ${label}`;
		}
		const dirLabel = direction === 'asc' ? 'ascending' : 'descending';
		return `Sort by ${label}, currently sorted ${dirLabel}. Click to change.`;
	});

	function handleClick() {
		let newDirection: SortDirection;
		if (!isActive) {
			// First click on this field: ascending
			newDirection = 'asc';
		} else if (direction === 'asc') {
			// Second click: descending
			newDirection = 'desc';
		} else {
			// Third click: ascending again
			newDirection = 'asc';
		}
		onSort?.(field, newDirection);
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault();
			handleClick();
		}
	}
</script>

<button
	type="button"
	onclick={handleClick}
	onkeydown={handleKeydown}
	aria-label={ariaLabel()}
	aria-pressed={isActive}
	data-sort-field={field}
	data-sort-direction={direction}
	class={cn(
		'flex items-center gap-1.5 px-2 py-1.5 text-sm font-medium rounded-md transition-colors',
		'text-muted-foreground hover:text-foreground hover:bg-muted/50',
		isActive && 'text-foreground bg-muted/50',
		className
	)}
>
	<span>{label}</span>
	{#if isActive && direction}
		{#if direction === 'asc'}
			<ArrowUp class="h-3.5 w-3.5" />
		{:else}
			<ArrowDown class="h-3.5 w-3.5" />
		{/if}
	{:else}
		<ArrowUpDown class="h-3.5 w-3.5 opacity-50" />
	{/if}
</button>
