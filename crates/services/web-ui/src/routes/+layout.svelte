<script lang="ts">
	import '../app.css';
	import { page } from '$app/stores';
	import { ModeWatcher, toggleMode } from 'mode-watcher';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Toaster } from '$lib/components/ui/sonner/index.js';
	import type { Snippet } from 'svelte';
	import Sun from 'lucide-svelte/icons/sun';
	import Moon from 'lucide-svelte/icons/moon';
	import Menu from 'lucide-svelte/icons/menu';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();

	interface NavItem {
		href: string;
		label: string;
		icon: string;
	}

	const navItems: NavItem[] = [
		{ href: '/', label: 'Dashboard', icon: '🏠' },
		{ href: '/projects', label: 'Projects', icon: '📁' },
		{ href: '/agents', label: 'Agents', icon: '🤖' },
		{ href: '/inbox', label: 'Inbox', icon: '📬' }
	];

	let sidebarOpen = $state(true);

	function isActive(href: string): boolean {
		if (href === '/') return $page.url.pathname === '/';
		return $page.url.pathname.startsWith(href);
	}
</script>

<ModeWatcher />
<Toaster />

<div class="min-h-screen flex">
	<aside
		class="w-64 bg-card border-r border-border flex-shrink-0 transition-all duration-300"
		class:hidden={!sidebarOpen}
	>
		<div class="p-4 border-b border-border">
			<h1 class="text-xl font-bold text-primary-600 dark:text-primary-400">📧 Agent Mail</h1>
			<p class="text-sm text-muted-foreground">MCP Communication Hub</p>
		</div>

		<nav class="p-4 space-y-1">
			{#each navItems as item}
				<a
					href={item.href}
					class="flex items-center gap-3 px-3 py-2 rounded-lg transition-colors"
					class:bg-primary-100={isActive(item.href)}
					class:dark:bg-primary-900={isActive(item.href)}
					class:text-primary-700={isActive(item.href)}
					class:dark:text-primary-300={isActive(item.href)}
					class:hover:bg-accent={!isActive(item.href)}
				>
					<span class="text-lg">{item.icon}</span>
					<span class="font-medium">{item.label}</span>
				</a>
			{/each}
		</nav>
	</aside>

	<div class="flex-1 flex flex-col">
		<header class="bg-card border-b border-border px-6 py-4">
			<div class="flex items-center justify-between">
				<Button variant="ghost" size="icon" onclick={() => (sidebarOpen = !sidebarOpen)}>
					<Menu class="h-5 w-5" />
					<span class="sr-only">Toggle sidebar</span>
				</Button>

				<div class="flex items-center gap-4">
					<span class="text-sm text-muted-foreground">MCP Agent Mail</span>
					<Button onclick={toggleMode} variant="outline" size="icon">
						<Sun
							class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 !transition-all dark:-rotate-90 dark:scale-0"
						/>
						<Moon
							class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 !transition-all dark:rotate-0 dark:scale-100"
						/>
						<span class="sr-only">Toggle theme</span>
					</Button>
				</div>
			</div>
		</header>

		<main class="flex-1 p-6 bg-background overflow-auto">
			{@render children()}
		</main>
	</div>
</div>
