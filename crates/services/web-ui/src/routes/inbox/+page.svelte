<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import { getProjects, getAgents, getInbox, type Project, type Agent, type Message } from '$lib/api/client';
	import { toast } from 'svelte-sonner';
	import ComposeMessage from '$lib/components/ComposeMessage.svelte';
	import Inbox from 'lucide-svelte/icons/inbox';
	import PenSquare from 'lucide-svelte/icons/pen-square';
	import RefreshCw from 'lucide-svelte/icons/refresh-cw';
	import X from 'lucide-svelte/icons/x';
	import { MessageListSkeleton } from '$lib/components/skeletons';
	import { Badge } from '$lib/components/ui/badge';
	import { ShimmerButton, BlurFade } from '$lib/components/magic';
	import * as Sheet from '$lib/components/ui/sheet';

	let projects = $state<Project[]>([]);
	let agents = $state<Agent[]>([]);
	let messages = $state<Message[]>([]);
	let loading = $state(true);
	let loadingMessages = $state(false);
	let error = $state<string | null>(null);

	// Selected filters from URL or user selection
	let selectedProject = $state<string>('');
	let selectedAgent = $state<string>('');

	// Compose modal - use Sheet on mobile, Dialog on desktop
	let showCompose = $state(false);

	// Detect mobile viewport
	let isMobile = $state(false);
	$effect(() => {
		if (browser) {
			const checkMobile = () => {
				isMobile = window.innerWidth < 768;
			};
			checkMobile();
			window.addEventListener('resize', checkMobile);
			return () => window.removeEventListener('resize', checkMobile);
		}
	});

	// Use $effect for client-side data loading in Svelte 5
	$effect(() => {
		if (browser) {
			initPage();
		}
	});

	async function initPage() {
		try {
			projects = await getProjects();

			// Check URL params for pre-selection
			const urlProject = $page.url.searchParams.get('project');
			const urlAgent = $page.url.searchParams.get('agent');

			if (urlProject) {
				selectedProject = urlProject;
				await loadAgentsForProject(urlProject);
				if (urlAgent) {
					selectedAgent = urlAgent;
					await loadMessages();
				}
			}
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load data';
		} finally {
			loading = false;
		}
	}

	async function loadAgentsForProject(projectSlug: string) {
		try {
			agents = await getAgents(projectSlug);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load agents';
			agents = [];
		}
	}

	async function handleProjectChange() {
		selectedAgent = '';
		messages = [];
		if (selectedProject) {
			await loadAgentsForProject(selectedProject);
			updateUrl();
		} else {
			agents = [];
		}
	}

	async function handleAgentChange() {
		updateUrl();
		if (selectedProject && selectedAgent) {
			await loadMessages();
		} else {
			messages = [];
		}
	}

	async function loadMessages() {
		if (!selectedProject || !selectedAgent) return;
		if (loadingMessages) return; // Prevent duplicate loads

		loadingMessages = true;
		error = null;
		try {
			messages = await getInbox(selectedProject, selectedAgent);
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load messages';
			messages = [];
		} finally {
			loadingMessages = false;
		}
	}

	function updateUrl() {
		const params = new URLSearchParams();
		if (selectedProject) params.set('project', selectedProject);
		if (selectedAgent) params.set('agent', selectedAgent);
		const newUrl = params.toString() ? `?${params.toString()}` : '/inbox';
		goto(newUrl, { replaceState: true, keepFocus: true });
	}

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		const now = new Date();
		const isToday = date.toDateString() === now.toDateString();

		if (isToday) {
			return date.toLocaleTimeString('en-US', {
				hour: '2-digit',
				minute: '2-digit'
			});
		}

		return date.toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric'
		});
	}

	function getImportanceVariant(importance: string): "default" | "secondary" | "destructive" | "outline" {
		switch (importance) {
			case 'high':
				return 'destructive';
			case 'low':
				return 'secondary';
			default:
				return 'default';
		}
	}

	function truncateBody(body: string | undefined, maxLength: number = 100): string {
		if (!body) return '';
		if (body.length <= maxLength) return body;
		return body.substring(0, maxLength) + '...';
	}

	function handleMessageSent() {
		showCompose = false;
		loadMessages();
		toast.success('Message sent successfully');
	}
</script>

<div class="space-y-4 md:space-y-6">
	<!-- Header -->
	<BlurFade delay={0}>
		<div class="flex items-center justify-between">
			<div>
				<h1 class="text-xl md:text-2xl font-bold text-gray-900 dark:text-white">Inbox</h1>
				<p class="text-sm md:text-base text-gray-600 dark:text-gray-400">View messages for your agents</p>
			</div>
			{#if selectedProject && selectedAgent}
				<ShimmerButton
					size={isMobile ? 'sm' : 'md'}
					on:click={() => showCompose = true}
				>
					<PenSquare class="h-4 w-4 mr-2" />
					<span class="hidden sm:inline">Compose</span>
					<span class="sm:hidden">New</span>
				</ShimmerButton>
			{/if}
		</div>
	</BlurFade>

	<!-- Filters -->
	<BlurFade delay={100}>
		<div class="bg-white dark:bg-gray-800 rounded-xl p-3 md:p-4 shadow-sm border border-gray-200 dark:border-gray-700">
			<div class="flex flex-col sm:flex-row gap-3 md:gap-4">
				<!-- Project Selector -->
				<div class="flex-1">
					<label for="projectSelect" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
						Project
					</label>
					<select
						id="projectSelect"
						bind:value={selectedProject}
						onchange={handleProjectChange}
						class="w-full min-h-[44px] px-3 md:px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary-500 focus:border-transparent"
					>
						<option value="">Select a project...</option>
						{#each projects as project}
							<option value={project.slug}>{project.human_key}</option>
						{/each}
					</select>
				</div>

				<!-- Agent Selector -->
				<div class="flex-1">
					<label for="agentSelect" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
						Agent
					</label>
					<select
						id="agentSelect"
						bind:value={selectedAgent}
						onchange={handleAgentChange}
						disabled={!selectedProject || agents.length === 0}
						class="w-full min-h-[44px] px-3 md:px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-primary-500 focus:border-transparent disabled:opacity-50 disabled:cursor-not-allowed"
					>
						<option value="">Select an agent...</option>
						{#each agents as agent}
							<option value={agent.name}>{agent.name}</option>
						{/each}
					</select>
				</div>

				<!-- Refresh Button -->
				{#if selectedProject && selectedAgent}
					<div class="flex items-end">
						<button
							onclick={loadMessages}
							disabled={loadingMessages}
							class="w-full sm:w-auto min-h-[44px] px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors disabled:opacity-50 flex items-center justify-center gap-2"
							aria-label="Refresh messages"
						>
							<RefreshCw class="h-4 w-4 {loadingMessages ? 'animate-spin' : ''}" />
							<span class="sm:inline">Refresh</span>
						</button>
					</div>
				{/if}
			</div>
		</div>
	</BlurFade>

	<!-- Error Message -->
	{#if error}
		<BlurFade delay={200}>
			<div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-xl p-4">
				<p class="text-red-700 dark:text-red-400">{error}</p>
			</div>
		</BlurFade>
	{/if}

	<!-- Loading State -->
	{#if loading}
		<div class="flex items-center justify-center py-12">
			<div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-600"></div>
		</div>
	{:else if !selectedProject || !selectedAgent}
		<!-- Selection Prompt -->
		<BlurFade delay={200}>
			<div class="bg-white dark:bg-gray-800 rounded-xl p-8 md:p-12 text-center shadow-sm border border-gray-200 dark:border-gray-700">
				<div class="mb-4 flex justify-center"><Inbox class="h-12 w-12 text-gray-400" /></div>
				<h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-2">Select an Agent</h3>
				<p class="text-gray-600 dark:text-gray-400">
					Choose a project and agent to view their inbox.
				</p>
			</div>
		</BlurFade>
	{:else if loadingMessages}
		<MessageListSkeleton rows={5} />
	{:else if messages.length === 0}
		<!-- Empty Inbox -->
		<BlurFade delay={200}>
			<div class="bg-white dark:bg-gray-800 rounded-xl p-8 md:p-12 text-center shadow-sm border border-gray-200 dark:border-gray-700">
				<div class="mb-4 flex justify-center"><Inbox class="h-12 w-12 text-gray-400" /></div>
				<h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-2">Inbox is empty</h3>
				<p class="text-gray-600 dark:text-gray-400 mb-4">
					No messages for {selectedAgent} yet.
				</p>
				<ShimmerButton on:click={() => showCompose = true}>
					Send a Message
				</ShimmerButton>
			</div>
		</BlurFade>
	{:else}
		<!-- Messages List -->
		<BlurFade delay={200}>
			<div class="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden">
				<div class="p-3 md:p-4 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between">
					<span class="text-sm text-gray-600 dark:text-gray-400">
						{messages.length} message{messages.length === 1 ? '' : 's'}
					</span>
				</div>
				<ul class="divide-y divide-gray-200 dark:divide-gray-700" role="list">
					{#each messages as message, index}
						<li
							class="animate-in fade-in slide-in-from-bottom-2"
							style="animation-delay: {index * 50}ms; animation-fill-mode: both;"
						>
							<a
								href="/inbox/{message.id}?project={selectedProject}&agent={selectedAgent}"
								class="block min-h-[72px] p-3 md:p-4 hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors active:bg-gray-100 dark:active:bg-gray-700"
							>
								<div class="flex items-start justify-between gap-3 md:gap-4">
									<div class="flex-1 min-w-0">
										<div class="flex flex-wrap items-center gap-1.5 md:gap-2 mb-1">
											<h4 class="font-medium text-gray-900 dark:text-white truncate text-sm md:text-base">
												{message.subject || '(No subject)'}
											</h4>
											{#if message.importance !== 'normal'}
												<Badge variant={getImportanceVariant(message.importance)}>
													{message.importance}
												</Badge>
											{/if}
											{#if message.ack_required}
												<Badge variant="outline" class="border-amber-500 text-amber-600 dark:text-amber-400">
													ACK
												</Badge>
											{/if}
											{#if message.thread_id}
												<Badge variant="secondary">
													Thread
												</Badge>
											{/if}
										</div>
										<p class="text-xs md:text-sm text-gray-600 dark:text-gray-400 truncate">
											{truncateBody(message.body_md)}
										</p>
									</div>
									<div class="text-xs md:text-sm text-gray-500 dark:text-gray-400 whitespace-nowrap shrink-0">
										{formatDate(message.created_ts)}
									</div>
								</div>
							</a>
						</li>
					{/each}
				</ul>
			</div>
		</BlurFade>
	{/if}
</div>

<!-- Compose - Sheet on mobile, Dialog on desktop -->
{#if isMobile}
	<Sheet.Root bind:open={showCompose}>
		<Sheet.Content side="bottom" class="h-[90vh] rounded-t-xl">
			<Sheet.Header class="pb-4">
				<Sheet.Title>New Message</Sheet.Title>
				<Sheet.Description>
					Send a message from {selectedAgent}
				</Sheet.Description>
			</Sheet.Header>
			<div class="flex-1 overflow-y-auto">
				<ComposeMessage
					projectSlug={selectedProject}
					senderName={selectedAgent}
					{agents}
					onClose={() => showCompose = false}
					onSent={handleMessageSent}
				/>
			</div>
		</Sheet.Content>
	</Sheet.Root>
{:else if showCompose}
	<div
		class="fixed inset-0 bg-black/50 flex items-center justify-center p-4 z-50"
		onclick={(e) => { if (e.target === e.currentTarget) showCompose = false; }}
		onkeydown={(e) => { if (e.key === 'Escape') showCompose = false; }}
		role="dialog"
		aria-modal="true"
		aria-labelledby="compose-title"
		tabindex="-1"
	>
		<div class="bg-white dark:bg-gray-800 rounded-xl shadow-xl max-w-2xl w-full max-h-[90vh] overflow-hidden">
			<div class="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700">
				<h2 id="compose-title" class="text-lg font-semibold text-gray-900 dark:text-white">New Message</h2>
				<button
					onclick={() => showCompose = false}
					class="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
					aria-label="Close"
				>
					<X class="h-5 w-5" />
				</button>
			</div>
			<ComposeMessage
				projectSlug={selectedProject}
				senderName={selectedAgent}
				{agents}
				onClose={() => showCompose = false}
				onSent={handleMessageSent}
			/>
		</div>
	</div>
{/if}

<style>
	/* Staggered animation keyframes */
	@keyframes fade-in {
		from { opacity: 0; }
		to { opacity: 1; }
	}

	@keyframes slide-in-from-bottom-2 {
		from { transform: translateY(8px); }
		to { transform: translateY(0); }
	}

	.animate-in {
		animation: fade-in 300ms ease-out, slide-in-from-bottom-2 300ms ease-out;
	}

	/* Respect reduced motion */
	@media (prefers-reduced-motion: reduce) {
		.animate-in {
			animation: none;
		}
	}
</style>
