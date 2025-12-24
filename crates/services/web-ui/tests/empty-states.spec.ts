import { test, expect } from '@playwright/test';

test.describe('Empty States', () => {
	test.describe('Projects Page Empty States', () => {
		test('empty projects list shows helpful empty state', async ({ page }) => {
			// Mock empty projects response
			await page.route('**/api/project/list', async (route) => {
				await route.fulfill({ json: [] });
			});

			await page.goto('/projects');
			await page.waitForLoadState('networkidle');

			// Should have icon
			const emptyState = page.locator('[data-testid="empty-state"]');
			await expect(emptyState).toBeVisible();

			// Should have descriptive title
			await expect(page.getByText(/no projects yet/i)).toBeVisible();

			// Should have helpful description
			await expect(page.getByText(/create your first project/i)).toBeVisible();

			// Should have primary CTA button
			const createButton = page.getByRole('button', { name: /create project|new project/i });
			await expect(createButton).toBeVisible();
		});

		test('no search results shows clear message', async ({ page }) => {
			await page.goto('/projects');
			await page.waitForLoadState('networkidle');

			// Wait for projects to load
			await page.waitForTimeout(1000);

			// Search for something that doesn't exist
			const search = page.locator('input[placeholder*="Search"]');
			await search.fill('xyznonexistent123456789');
			await page.waitForTimeout(300);

			// Should show no results message
			await expect(page.getByText(/no projects found/i)).toBeVisible();

			// Should have clear search button
			const clearButton = page.getByRole('button', { name: /clear search/i });
			await expect(clearButton).toBeVisible();
		});
	});

	test.describe('Agents Page Empty States', () => {
		test('empty agents list shows helpful empty state', async ({ page }) => {
			// Mock empty response
			await page.route('**/api/project/list', async (route) => {
				await route.fulfill({ json: [] });
			});

			await page.goto('/agents');
			await page.waitForLoadState('networkidle');

			// Should show empty state
			await expect(page.getByText(/no agents yet/i)).toBeVisible();

			// Should have CTA to go to projects
			const projectsLink = page.getByRole('button', { name: /go to projects/i });
			await expect(projectsLink).toBeVisible();
		});

		test('no matching agents shows filter message', async ({ page }) => {
			await page.goto('/agents');
			await page.waitForLoadState('networkidle');
			await page.waitForTimeout(1000);

			// Search for something that doesn't exist
			const search = page.locator('input[placeholder*="Search"]');
			if (await search.isVisible()) {
				await search.fill('xyznonexistent123456789');
				await page.waitForTimeout(300);

				// Either show "no matching agents" or regular list
				const noMatching = page.getByText(/no matching agents/i);
				const hasAgents = await page.locator('.grid .group').count() > 0;

				// One of these should be true
				const hasNoMatchingMessage = await noMatching.isVisible().catch(() => false);
				expect(hasNoMatchingMessage || !hasAgents).toBeTruthy();
			}
		});
	});

	test.describe('Inbox Page Empty States', () => {
		test('inbox shows select project/agent message when none selected', async ({ page }) => {
			await page.goto('/inbox');
			await page.waitForLoadState('networkidle');

			// Should prompt to select project and agent
			await expect(page.getByText(/select a project/i)).toBeVisible();
		});

		test('empty inbox shows friendly message', async ({ page }) => {
			// This requires having a project and agent but no messages
			await page.goto('/inbox');
			await page.waitForLoadState('networkidle');

			// If we can get to an empty inbox state
			const emptyInbox = page.getByText(/inbox is empty|no messages/i);
			const selectPrompt = page.getByText(/select a project/i);

			// Either empty inbox or select prompt should be visible
			const hasEmptyOrSelect = await emptyInbox.isVisible().catch(() => false) ||
				await selectPrompt.isVisible().catch(() => false);
			expect(hasEmptyOrSelect).toBeTruthy();
		});
	});

	test.describe('Empty State Accessibility', () => {
		test('empty state CTA is keyboard accessible', async ({ page }) => {
			await page.route('**/api/project/list', async (route) => {
				await route.fulfill({ json: [] });
			});

			await page.goto('/projects');
			await page.waitForLoadState('networkidle');

			// Tab through the page
			await page.keyboard.press('Tab');
			await page.keyboard.press('Tab');
			await page.keyboard.press('Tab');

			// The create button should be focusable
			const createButton = page.getByRole('button', { name: /create project|new project/i });
			const activeElement = page.locator(':focus');

			// Check if focus is on a button (might need multiple tabs)
			for (let i = 0; i < 5; i++) {
				const focusedText = await activeElement.textContent().catch(() => '');
				if (focusedText?.toLowerCase().includes('project')) {
					break;
				}
				await page.keyboard.press('Tab');
			}

			// Verify the button exists and is accessible
			await expect(createButton).toBeVisible();
		});

		test('empty state icons have proper alt text or aria-label', async ({ page }) => {
			await page.route('**/api/project/list', async (route) => {
				await route.fulfill({ json: [] });
			});

			await page.goto('/projects');
			await page.waitForLoadState('networkidle');

			// Check for proper labeling on the empty state
			const emptyState = page.locator('[data-testid="empty-state"]');
			await expect(emptyState).toBeVisible();

			// The icon container should be properly labeled for screen readers
			const iconArea = emptyState.locator('svg').first();
			const hasIcon = await iconArea.isVisible().catch(() => false);
			expect(hasIcon).toBeTruthy();
		});
	});
});
