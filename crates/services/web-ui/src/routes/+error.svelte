<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import Home from 'lucide-svelte/icons/home';
	import ArrowLeft from 'lucide-svelte/icons/arrow-left';
	import FolderKanban from 'lucide-svelte/icons/folder-kanban';
	import Mail from 'lucide-svelte/icons/mail';
	import Inbox from 'lucide-svelte/icons/inbox';
	import AlertCircle from 'lucide-svelte/icons/alert-circle';
	import { Button } from '$lib/components/ui/button';
	import { BlurFade } from '$lib/components/magic';

	let status = $derived($page.status);
	let message = $derived($page.error?.message ?? 'Page not found');

	function goBack() {
		if (browser) {
			history.back();
		}
	}

	const suggestions = [
		{ href: '/', label: 'Dashboard', icon: Home },
		{ href: '/projects', label: 'Projects', icon: FolderKanban },
		{ href: '/mail', label: 'Mail', icon: Mail },
		{ href: '/inbox', label: 'Inbox', icon: Inbox }
	];
</script>

<div
	class="min-h-[80vh] flex items-center justify-center px-4"
	data-testid="error-page"
>
	<BlurFade delay={0}>
		<div class="text-center space-y-6 max-w-md">
			<!-- Error Icon & Code -->
			<div class="flex flex-col items-center gap-4">
				<div class="w-20 h-20 rounded-full bg-red-100 dark:bg-red-900/20 flex items-center justify-center">
					<AlertCircle class="h-10 w-10 text-red-600 dark:text-red-400" />
				</div>
				<div class="text-7xl font-bold text-muted-foreground/30">
					{status}
				</div>
			</div>

			<!-- Error Message -->
			<div class="space-y-2">
				<h1 class="text-2xl font-semibold text-foreground">
					{#if status === 404}
						Page Not Found
					{:else if status === 500}
						Server Error
					{:else}
						Something Went Wrong
					{/if}
				</h1>
				<p class="text-muted-foreground">
					{#if status === 404}
						The page you're looking for doesn't exist or has been moved.
					{:else}
						{message}
					{/if}
				</p>
			</div>

			<!-- Actions -->
			<div class="flex flex-col sm:flex-row gap-3 justify-center">
				<Button variant="outline" onclick={goBack} class="min-h-[44px]">
					<ArrowLeft class="h-4 w-4 mr-2" />
					Go Back
				</Button>
				<Button onclick={() => goto('/')} class="min-h-[44px]">
					<Home class="h-4 w-4 mr-2" />
					Dashboard
				</Button>
			</div>

			<!-- Suggestions -->
			{#if status === 404}
				<div class="pt-6 border-t border-border">
					<p class="text-sm text-muted-foreground mb-4">
						Looking for one of these?
					</p>
					<div class="flex flex-wrap gap-2 justify-center">
						{#each suggestions as { href, label, icon: Icon }}
							<Button variant="ghost" size="sm" href={href} class="min-h-[44px]">
								<Icon class="h-4 w-4 mr-2" />
								{label}
							</Button>
						{/each}
					</div>
				</div>
			{/if}
		</div>
	</BlurFade>
</div>
