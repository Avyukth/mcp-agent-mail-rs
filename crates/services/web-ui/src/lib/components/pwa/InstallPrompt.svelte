<script lang="ts">
	import { browser } from '$app/environment';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import { Checkbox } from '$lib/components/ui/checkbox/index.js';
	import Download from 'lucide-svelte/icons/download';
	import X from 'lucide-svelte/icons/x';
	import Share from 'lucide-svelte/icons/share';

	interface BeforeInstallPromptEvent extends Event {
		prompt(): Promise<void>;
		userChoice: Promise<{ outcome: 'accepted' | 'dismissed' }>;
	}

	const STORAGE_KEY = 'pwa-install-dismissed-permanently';
	const AUTO_DISMISS_MS = 10000;

	let deferredPrompt = $state<BeforeInstallPromptEvent | null>(null);
	let showPrompt = $state(false);
	let isIOS = $state(false);
	let isStandalone = $state(false);
	let dontShowAgain = $state(false);
	let autoDismissTimer: ReturnType<typeof setTimeout> | null = null;

	$effect(() => {
		if (!browser) return;

		const permanentlyDismissed = localStorage.getItem(STORAGE_KEY) === 'true';
		if (permanentlyDismissed) return;

		isIOS = /iPad|iPhone|iPod/.test(navigator.userAgent);
		isStandalone = window.matchMedia('(display-mode: standalone)').matches;

		if (isStandalone) return;

		const handleBeforeInstall = (e: Event) => {
			e.preventDefault();
			deferredPrompt = e as BeforeInstallPromptEvent;
			showPrompt = true;
			startAutoDismissTimer();
		};

		window.addEventListener('beforeinstallprompt', handleBeforeInstall);

		if (isIOS && !isStandalone) {
			const timer = setTimeout(() => {
				showPrompt = true;
				startAutoDismissTimer();
			}, 3000);
			return () => {
				clearTimeout(timer);
				clearAutoDismissTimer();
			};
		}

		return () => {
			window.removeEventListener('beforeinstallprompt', handleBeforeInstall);
			clearAutoDismissTimer();
		};
	});

	function startAutoDismissTimer() {
		clearAutoDismissTimer();
		autoDismissTimer = setTimeout(() => {
			if (showPrompt) dismiss();
		}, AUTO_DISMISS_MS);
	}

	function clearAutoDismissTimer() {
		if (autoDismissTimer) {
			clearTimeout(autoDismissTimer);
			autoDismissTimer = null;
		}
	}

	async function handleInstall() {
		if (!deferredPrompt) return;

		await deferredPrompt.prompt();
		const { outcome } = await deferredPrompt.userChoice;

		if (outcome === 'accepted') {
			localStorage.setItem(STORAGE_KEY, 'true');
			showPrompt = false;
		}
		deferredPrompt = null;
		clearAutoDismissTimer();
	}

	function dismiss() {
		showPrompt = false;
		clearAutoDismissTimer();
		if (browser && dontShowAgain) {
			localStorage.setItem(STORAGE_KEY, 'true');
		}
	}
</script>

{#if showPrompt && !isStandalone}
	<div
		class="fixed bottom-4 left-4 right-4 z-40 md:left-auto md:right-4 md:w-96"
		role="dialog"
		aria-labelledby="install-title"
		data-testid="pwa-install-modal"
	>
		<Card.Root class="shadow-lg border-primary/20">
			<Card.Header class="pb-2">
				<div class="flex items-start justify-between">
					<Card.Title id="install-title" class="text-base">Install Agent Mail</Card.Title>
					<Button
						variant="ghost"
						size="icon"
						class="h-8 w-8 -mr-2 -mt-2"
						onclick={dismiss}
						aria-label="Close"
						data-testid="pwa-close-button"
					>
						<X class="h-4 w-4" />
					</Button>
				</div>
			</Card.Header>
			<Card.Content class="space-y-3">
				{#if isIOS}
					<p class="text-sm text-muted-foreground">
						Install this app on your device for quick access:
					</p>
					<ol class="text-sm text-muted-foreground space-y-2 list-decimal list-inside">
						<li class="flex items-center gap-2">
							Tap the Share button <Share class="h-4 w-4 inline" />
						</li>
						<li>Scroll down and tap "Add to Home Screen"</li>
						<li>Tap "Add" to confirm</li>
					</ol>
				{:else}
					<p class="text-sm text-muted-foreground">
						Install Agent Mail for quick access and offline support.
					</p>
					<Button onclick={handleInstall} class="w-full gap-2">
						<Download class="h-4 w-4" />
						Install App
					</Button>
				{/if}

				<div class="flex items-center gap-2 pt-2 border-t">
					<Checkbox
						id="dont-show-again"
						bind:checked={dontShowAgain}
						data-testid="dont-show-again-checkbox"
					/>
					<label for="dont-show-again" class="text-sm text-muted-foreground cursor-pointer">
						Don't show again
					</label>
				</div>

				<Button
					variant="outline"
					size="sm"
					onclick={dismiss}
					class="w-full"
					data-testid="pwa-dismiss-button"
				>
					Later
				</Button>
			</Card.Content>
		</Card.Root>
	</div>
{/if}
