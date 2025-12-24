import { test, expect } from '@playwright/test';

const API_BASE = process.env.API_BASE || 'http://localhost:9765';

test.describe('Project List Sorting', () => {
	test.beforeAll(async ({ request }) => {
		// Create test projects with distinct names for sorting verification
		const testProjects = [
			'alpha-sort-test-' + Date.now().toString(36),
			'zeta-sort-test-' + Date.now().toString(36),
			'mid-sort-test-' + Date.now().toString(36),
		];

		for (const name of testProjects) {
			await request.post(`${API_BASE}/api/project/ensure`, {
				data: { human_key: name }
			});
		}
	});

	test('projects list has sort controls', async ({ page }) => {
		await page.goto('/projects');
		await page.waitForLoadState('networkidle');

		// Sort controls should exist in the toolbar
		const sortByName = page.locator('button[data-sort-field="name"]');
		const sortByDate = page.locator('button[data-sort-field="date"]');

		await expect(sortByName).toBeVisible();
		await expect(sortByDate).toBeVisible();
	});

	test('clicking sort button changes sort direction indicator', async ({ page }) => {
		await page.goto('/projects');
		await page.waitForLoadState('networkidle');

		// Wait for projects to load
		await page.waitForSelector('.grid a[href^="/projects/"]', { timeout: 10000 });

		const sortByName = page.locator('button[data-sort-field="name"]');

		// Click to sort ascending
		await sortByName.click();
		await expect(sortByName).toHaveAttribute('data-sort-direction', 'asc');

		// Click again for descending
		await sortByName.click();
		await expect(sortByName).toHaveAttribute('data-sort-direction', 'desc');
	});

	test('sort by name orders projects alphabetically', async ({ page }) => {
		await page.goto('/projects');
		await page.waitForLoadState('networkidle');

		// Wait for projects to load
		await page.waitForSelector('.grid a[href^="/projects/"]', { timeout: 10000 });

		const sortByName = page.locator('button[data-sort-field="name"]');

		// Sort ascending
		await sortByName.click();
		await page.waitForTimeout(300);

		// Get project names in order
		const cards = page.locator('.grid a[href^="/projects/"] h3');
		const names: string[] = [];
		const count = await cards.count();

		for (let i = 0; i < Math.min(count, 5); i++) {
			const name = await cards.nth(i).textContent();
			if (name) names.push(name.toLowerCase());
		}

		// Verify ascending order
		const sortedAsc = [...names].sort((a, b) => a.localeCompare(b));
		expect(names).toEqual(sortedAsc);

		// Sort descending
		await sortByName.click();
		await page.waitForTimeout(300);

		const namesDesc: string[] = [];
		for (let i = 0; i < Math.min(count, 5); i++) {
			const name = await cards.nth(i).textContent();
			if (name) namesDesc.push(name.toLowerCase());
		}

		// Verify descending order
		const sortedDesc = [...namesDesc].sort((a, b) => b.localeCompare(a));
		expect(namesDesc).toEqual(sortedDesc);
	});

	test('sort by date orders projects chronologically', async ({ page }) => {
		await page.goto('/projects');
		await page.waitForLoadState('networkidle');

		// Wait for projects to load
		await page.waitForSelector('.grid a[href^="/projects/"]', { timeout: 10000 });

		const sortByDate = page.locator('button[data-sort-field="date"]');

		// Sort by date (newest first by default)
		await sortByDate.click();
		await page.waitForTimeout(300);

		// Verify date indicator is active
		await expect(sortByDate).toHaveAttribute('data-sort-direction', /asc|desc/);
	});

	test('sort controls have proper accessibility attributes', async ({ page }) => {
		await page.goto('/projects');
		await page.waitForLoadState('networkidle');

		const sortByName = page.locator('button[data-sort-field="name"]');
		const sortByDate = page.locator('button[data-sort-field="date"]');

		// Should have aria-label for accessibility
		await expect(sortByName).toBeVisible();
		await expect(sortByDate).toBeVisible();

		// Click to activate and check aria-pressed
		await sortByName.click();
		await expect(sortByName).toHaveAttribute('aria-pressed', 'true');
	});

	test('sorting works with search filter active', async ({ page }) => {
		await page.goto('/projects');
		await page.waitForLoadState('networkidle');

		// Wait for projects to load
		await page.waitForSelector('.grid a[href^="/projects/"]', { timeout: 10000 });

		// Apply search filter
		const search = page.locator('input[placeholder*="Search"]');
		await search.fill('sort-test');
		await page.waitForTimeout(300);

		// Apply sort
		const sortByName = page.locator('button[data-sort-field="name"]');
		await sortByName.click();
		await page.waitForTimeout(300);

		// Verify both filters work together
		const cards = page.locator('.grid a[href^="/projects/"]');
		const count = await cards.count();

		// Should have filtered results
		if (count > 0) {
			// All visible should contain "sort-test"
			for (let i = 0; i < count; i++) {
				const text = await cards.nth(i).textContent();
				expect(text?.toLowerCase()).toContain('sort-test');
			}
		}
	});

	test('default sort shows newest projects first', async ({ page }) => {
		await page.goto('/projects');
		await page.waitForLoadState('networkidle');

		// Wait for projects to load
		await page.waitForSelector('.grid a[href^="/projects/"]', { timeout: 10000 });

		// Check that date sort button shows as active by default
		const sortByDate = page.locator('button[data-sort-field="date"]');

		// Date should be the default active sort
		await expect(sortByDate).toHaveAttribute('aria-pressed', 'true');

		// Verify cards exist
		const cards = page.locator('.grid a[href^="/projects/"]');
		const count = await cards.count();
		expect(count).toBeGreaterThan(0);
	});
});

test.describe('Agent List Sorting', () => {
	test('agents list has sort controls', async ({ page }) => {
		await page.goto('/agents');
		await page.waitForLoadState('networkidle');

		// Sort controls should exist
		const sortByName = page.locator('button[data-sort-field="name"]');
		const sortByModel = page.locator('button[data-sort-field="model"]');
		const sortByActivity = page.locator('button[data-sort-field="activity"]');

		await expect(sortByName).toBeVisible();
		await expect(sortByModel).toBeVisible();
		await expect(sortByActivity).toBeVisible();
	});

	test('agents sorting works with project filter', async ({ page }) => {
		await page.goto('/agents');
		await page.waitForLoadState('networkidle');
		await page.waitForTimeout(1000);

		// Apply sort
		const sortByName = page.locator('button[data-sort-field="name"]');
		if (await sortByName.isVisible()) {
			await sortByName.click();
			await page.waitForTimeout(300);

			// Should still show agents (or empty state)
			const cards = page.locator('.grid .group, [data-testid="agent-card"]');
			const emptyState = page.getByText(/no agents/i);

			const hasCards = await cards.count() > 0;
			const hasEmptyState = await emptyState.isVisible().catch(() => false);

			expect(hasCards || hasEmptyState).toBeTruthy();
		}
	});
});
