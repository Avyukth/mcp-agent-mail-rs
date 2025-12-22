<script lang="ts">
	import { Skeleton } from '$lib/components/ui/skeleton/index.js';

	let { rows = 3 }: { rows?: number } = $props();
</script>

<div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden">
	<table class="w-full">
		<thead class="bg-gray-50 dark:bg-gray-700">
			<tr>
				<th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
					Slug
				</th>
				<th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
					Path
				</th>
				<th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
					Created
				</th>
				<th class="px-6 py-3 text-right text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
					Actions
				</th>
			</tr>
		</thead>
		<tbody class="divide-y divide-gray-200 dark:divide-gray-700">
			{#each Array(rows) as _, i}
				<tr>
					<td class="px-6 py-4 whitespace-nowrap">
						<Skeleton class="h-5 w-24 skeleton-shimmer" />
					</td>
					<td class="px-6 py-4">
						<Skeleton class="h-4 w-48 skeleton-shimmer" />
					</td>
					<td class="px-6 py-4 whitespace-nowrap">
						<Skeleton class="h-4 w-24 skeleton-shimmer" />
					</td>
					<td class="px-6 py-4 whitespace-nowrap text-right">
						<Skeleton class="h-4 w-20 ml-auto skeleton-shimmer" />
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
