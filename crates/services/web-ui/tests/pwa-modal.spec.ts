import { test, expect } from '@playwright/test';

const injectBeforeInstallPrompt = () => {
	window.addEventListener('load', () => {
		setTimeout(() => {
			const event = new Event('beforeinstallprompt', { cancelable: true });
			(event as any).prompt = () => Promise.resolve();
			(event as any).userChoice = Promise.resolve({ outcome: 'dismissed' });
			window.dispatchEvent(event);
		}, 100);
	});
};

test.describe('PWA Install Modal', () => {
	test.beforeEach(async ({ page }) => {
		await page.addInitScript(() => localStorage.clear());
	});

	test('shows modal on first visit when beforeinstallprompt fires', async ({ page }) => {
		await page.addInitScript(injectBeforeInstallPrompt);
		await page.goto('/');
		await page.waitForLoadState('networkidle');

		const modal = page.locator('[data-testid="pwa-install-modal"]');
		await expect(modal).toBeVisible({ timeout: 5000 });
	});

	test('has "Don\'t show again" checkbox', async ({ page }) => {
		await page.addInitScript(injectBeforeInstallPrompt);
		await page.goto('/');
		await page.waitForLoadState('networkidle');

		const modal = page.locator('[data-testid="pwa-install-modal"]');
		await expect(modal).toBeVisible({ timeout: 5000 });

		const checkbox = modal.locator('[data-testid="dont-show-again-checkbox"]');
		await expect(checkbox).toBeVisible();

		const label = modal.getByText(/don't show again/i);
		await expect(label).toBeVisible();
	});

	test('remembers "Don\'t show again" preference', async ({ page }) => {
		await page.addInitScript(injectBeforeInstallPrompt);
		await page.goto('/');
		await page.waitForLoadState('networkidle');

		const modal = page.locator('[data-testid="pwa-install-modal"]');
		await expect(modal).toBeVisible({ timeout: 5000 });

		await modal.locator('[data-testid="dont-show-again-checkbox"]').click();
		await modal.locator('[data-testid="pwa-dismiss-button"]').click();
		await expect(modal).not.toBeVisible();

		await page.reload();
		await page.waitForLoadState('networkidle');
		await page.waitForTimeout(1000);

		await expect(modal).not.toBeVisible();
	});

	test('can dismiss without "Don\'t show again"', async ({ page }) => {
		await page.addInitScript(injectBeforeInstallPrompt);
		await page.goto('/');
		await page.waitForLoadState('networkidle');

		const modal = page.locator('[data-testid="pwa-install-modal"]');
		await expect(modal).toBeVisible({ timeout: 5000 });

		await modal.locator('[data-testid="pwa-dismiss-button"]').click();
		await expect(modal).not.toBeVisible();

		await page.addInitScript(injectBeforeInstallPrompt);
		await page.reload();
		await page.waitForLoadState('networkidle');

		await expect(modal).toBeVisible({ timeout: 5000 });
	});

	test('auto-dismisses after timeout', async ({ page }) => {
		await page.addInitScript(injectBeforeInstallPrompt);
		await page.goto('/');
		await page.waitForLoadState('networkidle');

		const modal = page.locator('[data-testid="pwa-install-modal"]');
		await expect(modal).toBeVisible({ timeout: 5000 });

		await expect(modal).not.toBeVisible({ timeout: 15000 });
	});

	test('does not block main content interaction', async ({ page }) => {
		await page.addInitScript(injectBeforeInstallPrompt);
		await page.goto('/');
		await page.waitForLoadState('networkidle');

		const modal = page.locator('[data-testid="pwa-install-modal"]');
		await expect(modal).toBeVisible({ timeout: 5000 });

		const projectsLink = page.getByRole('link', { name: /projects/i });
		await projectsLink.click();

		await expect(page).toHaveURL(/\/projects/);
	});

	test('close button dismisses modal', async ({ page }) => {
		await page.addInitScript(injectBeforeInstallPrompt);
		await page.goto('/');
		await page.waitForLoadState('networkidle');

		const modal = page.locator('[data-testid="pwa-install-modal"]');
		await expect(modal).toBeVisible({ timeout: 5000 });

		await modal.locator('[data-testid="pwa-close-button"]').click();
		await expect(modal).not.toBeVisible();
	});

	test('does not show modal if already installed (standalone mode)', async ({ page }) => {
		await page.addInitScript(() => {
			Object.defineProperty(window, 'matchMedia', {
				writable: true,
				value: (query: string) => ({
					matches: query === '(display-mode: standalone)',
					media: query,
					onchange: null,
					addListener: () => {},
					removeListener: () => {},
					addEventListener: () => {},
					removeEventListener: () => {},
					dispatchEvent: () => true
				})
			});
		});

		await page.goto('/');
		await page.waitForLoadState('networkidle');
		await page.waitForTimeout(1000);

		const modal = page.locator('[data-testid="pwa-install-modal"]');
		await expect(modal).not.toBeVisible();
	});
});
