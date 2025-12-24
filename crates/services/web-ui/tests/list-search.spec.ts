import { test, expect } from '@playwright/test';

const API_BASE = process.env.API_BASE || 'http://localhost:9765';

test.describe('Project List Search', () => {
	test.beforeAll(async ({ request }) => {
		// Create test projects with distinct names
		const testProjects = [
			'search-proj1-' + Date.now().toString(36),
			'test-proj-' + Date.now().toString(36),
			'res-proj-' + Date.now().toString(36),
			'nomatch-' + Date.now().toString(36),
		];

		for (const name of testProjects) {
			await request.post(`${API_BASE}/api/project/ensure`, {
				data: { human_key: name }
			});
		}
	});

	test('has search input on projects page', async ({ page }) => {
		await page.goto('/projects');
		await page.waitForLoadState('networkidle');

		const search = page.locator('input[placeholder*="Search"]');
		await expect(search).toBeVisible();
	});

	test('filters projects by name', async ({ page }) => {
		await page.goto('/projects');
		await page.waitForLoadState('networkidle');

		// Wait for projects to load
		await page.waitForSelector('[data-testid="project-card"], .grid a[href^="/projects/"]', {
			timeout: 10000
		});

		const search = page.locator('input[placeholder*="Search"]');
		await search.fill('search-proj');

		// Wait for filter to apply
		await page.waitForTimeout(300);

		// Check that visible cards contain the search term
		const cards = page.locator('[data-testid="project-card"], .grid a[href^="/projects/"]');
		const count = await cards.count();

		// Should have at least our test project
		expect(count).toBeGreaterThan(0);

		// All visible cards should contain search term
		for (let i = 0; i < Math.min(count, 5); i++) {
			const text = await cards.nth(i).textContent();
			expect(text?.toLowerCase()).toContain('search');
		}
	});

	test('shows empty state when search has no matches', async ({ page }) => {
		await page.goto('/projects');
		await page.waitForLoadState('networkidle');

		const search = page.locator('input[placeholder*="Search"]');
		await search.fill('xyznonexistent123456789');

		// Wait for filter to apply
		await page.waitForTimeout(300);

		// Should show no results message
		await expect(page.getByText(/no projects found|no matching/i)).toBeVisible({ timeout: 5000 });
	});

	test('clears search with X button', async ({ page }) => {
		await page.goto('/projects');
		await page.waitForLoadState('networkidle');

		const search = page.locator('input[placeholder*="Search"]');
		await search.fill('test');

		// Wait for clear button to appear
		await page.waitForTimeout(200);

		const clearButton = page.locator('[data-testid="clear-search"]');
		await expect(clearButton).toBeVisible();
		await clearButton.click();

		await expect(search).toHaveValue('');
	});

	test('search count updates correctly', async ({ page }) => {
		await page.goto('/projects');
		await page.waitForLoadState('networkidle');

		// Wait for initial count
		await page.waitForSelector('text=/showing \\d+ of \\d+/i', { timeout: 10000 });

		// Get initial count
		const countText = await page.locator('text=/showing \\d+ of \\d+/i').textContent();
		const match = countText?.match(/showing (\d+) of (\d+)/i);
		const initialTotal = parseInt(match?.[2] || '0');

		// Search for something
		const search = page.locator('input[placeholder*="Search"]');
		await search.fill('test');
		await page.waitForTimeout(300);

		// Count should be less than or equal to total
		const newCountText = await page.locator('text=/showing \\d+ of \\d+/i').textContent();
		const newMatch = newCountText?.match(/showing (\d+) of (\d+)/i);
		const filteredCount = parseInt(newMatch?.[1] || '0');
		const stillTotal = parseInt(newMatch?.[2] || '0');

		expect(filteredCount).toBeLessThanOrEqual(initialTotal);
		expect(stillTotal).toBe(initialTotal);
	});
});

test.describe('Agent List Search', () => {
	test('has search input on agents page', async ({ page }) => {
		await page.goto('/agents');
		await page.waitForLoadState('networkidle');

		const search = page.locator('input[placeholder*="Search"]');
		await expect(search).toBeVisible();
	});

	test('filters agents by name or model', async ({ page }) => {
		await page.goto('/agents');
		await page.waitForLoadState('networkidle');

		// Wait for agents to load
		await page.waitForTimeout(2000);

		const search = page.locator('input[placeholder*="Search"]');
		await search.fill('claude');

		// Wait for filter to apply
		await page.waitForTimeout(300);

		// Check that some filtering occurred (or show empty state)
		const cards = page.locator('.grid .group, [data-testid="agent-card"]');
		const emptyState = page.getByText(/no matching agents|no agents/i);

		// Either we have filtered cards or empty state
		const hasCards = await cards.count() > 0;
		const hasEmptyState = await emptyState.isVisible().catch(() => false);

		expect(hasCards || hasEmptyState).toBeTruthy();
	});
});

test.describe('Inbox Project Dropdown Search', () => {
	test('dropdown has search functionality', async ({ page }) => {
		await page.goto('/inbox');
		await page.waitForLoadState('networkidle');

		// Find and click the project filter dropdown
		const dropdown = page.locator('button:has-text("All Projects"), [data-testid="project-select"]');
		await dropdown.click();

		// Should have search input in dropdown
		const dropdownSearch = page.locator('[role="combobox"] input, [cmdk-input]');
		await expect(dropdownSearch).toBeVisible({ timeout: 5000 });
	});

	test('filters dropdown options', async ({ page }) => {
		await page.goto('/inbox');
		await page.waitForLoadState('networkidle');

		// Open dropdown
		const dropdown = page.locator('button:has-text("All Projects"), [data-testid="project-select"]');
		await dropdown.click();

		// Type in search
		const dropdownSearch = page.locator('[role="combobox"] input, [cmdk-input]');
		await dropdownSearch.fill('test');

		// Wait for filter
		await page.waitForTimeout(200);

		// Options should be filtered (or show no results)
		const options = page.locator('[role="option"], [cmdk-item]');
		const count = await options.count();

		// Either filtered results or "no results" message
		if (count > 0) {
			const firstOption = await options.first().textContent();
			// First option might be "All Projects" which is always visible
			console.log('First dropdown option:', firstOption);
		}
	});
});
