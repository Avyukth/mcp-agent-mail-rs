<script lang="ts">
	import { Skeleton } from '$lib/components/ui/skeleton/index.js';
</script>

<!-- Status Cards -->
<div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
	{#each Array(3) as _, i}
		<div class="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
			<div class="flex items-center gap-3 mb-2">
				<Skeleton class="w-6 h-6 rounded skeleton-shimmer" />
				<Skeleton class="h-5 w-24 skeleton-shimmer" />
			</div>
			<Skeleton class="h-8 w-12 skeleton-shimmer" />
		</div>
	{/each}
</div>

<!-- Recent Projects -->
<div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700">
	<div class="p-4 border-b border-gray-200 dark:border-gray-700">
		<Skeleton class="h-6 w-32 skeleton-shimmer" />
	</div>
	<ul class="divide-y divide-gray-200 dark:divide-gray-700">
		{#each Array(5) as _, i}
			<li class="p-4">
				<div class="space-y-2">
					<Skeleton class="h-5 w-32 skeleton-shimmer" />
					<Skeleton class="h-4 w-48 skeleton-shimmer" />
				</div>
			</li>
		{/each}
	</ul>
</div>
