import { test, expect } from '@playwright/test';

const API_BASE = 'http://localhost:8000';

// UUID pattern: 8-4-4-4-12 hex chars (case insensitive)
const UUID_PATTERN = /[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}/i;

// Configure tests to run serially
test.describe.configure({ mode: 'serial' });

test.describe('UUID Hiding - P0 Security', () => {
	test.beforeAll(async ({ request }) => {
		// Create test project with UUID-like slug
		const projectRes = await request.post(`${API_BASE}/api/project/ensure`, {
			data: { human_key: '/test/uuid-hiding-test' }
		});
		expect(projectRes.ok()).toBeTruthy();
		const project = await projectRes.json();
		console.log('Created project with slug:', project.slug);
	});

	test('project cards do NOT show UUIDs', async ({ page }) => {
		await page.goto('/projects');
		await page.waitForLoadState('networkidle');

		// Wait for loading to complete
		await expect(page.locator('.animate-pulse').first()).not.toBeVisible({ timeout: 10000 }).catch(() => {});

		// Get all project card content
		const cards = page.locator('[data-testid="project-card"]');
		const cardCount = await cards.count();

		if (cardCount === 0) {
			// Try alternative selector for cards (the actual card container)
			const altCards = page.locator('.grid a[href^="/projects/"]');
			const altCount = await altCards.count();
			expect(altCount).toBeGreaterThan(0);

			// Check each card text for UUID patterns
			for (let i = 0; i < altCount; i++) {
				const cardText = await altCards.nth(i).textContent();
				expect(cardText).not.toMatch(UUID_PATTERN);
			}
		} else {
			// Check each card text for UUID patterns
			for (let i = 0; i < cardCount; i++) {
				const cardText = await cards.nth(i).textContent();
				expect(cardText).not.toMatch(UUID_PATTERN);
			}
		}
	});

	test('project detail breadcrumb shows human-readable name not UUID', async ({ page }) => {
		// First get a project slug from the API
		const response = await page.request.get(`${API_BASE}/api/projects`);
		const projects = await response.json();
		expect(projects.length).toBeGreaterThan(0);

		const project = projects[0];
		await page.goto(`/projects/${project.slug}`);
		await page.waitForLoadState('networkidle');

		// Check breadcrumb does not contain UUID
		const breadcrumb = page.locator('nav');
		const breadcrumbText = await breadcrumb.first().textContent();

		// Breadcrumb should NOT contain the UUID pattern
		expect(breadcrumbText).not.toMatch(UUID_PATTERN);
	});

	test('project detail header shows human_key not slug', async ({ page }) => {
		const response = await page.request.get(`${API_BASE}/api/projects`);
		const projects = await response.json();
		expect(projects.length).toBeGreaterThan(0);

		const project = projects[0];
		await page.goto(`/projects/${project.slug}`);
		await page.waitForLoadState('networkidle');

		// Wait for loading to complete
		await expect(page.locator('.animate-pulse').first()).not.toBeVisible({ timeout: 10000 }).catch(() => {});

		// Get page heading
		const heading = page.getByRole('heading', { level: 1 });
		const headingText = await heading.textContent();

		// Heading should NOT contain UUID pattern
		expect(headingText).not.toMatch(UUID_PATTERN);

		// Heading should contain human_key
		expect(headingText).toContain(project.human_key);
	});

	test('page title does NOT contain UUID', async ({ page }) => {
		const response = await page.request.get(`${API_BASE}/api/projects`);
		const projects = await response.json();
		expect(projects.length).toBeGreaterThan(0);

		const project = projects[0];
		await page.goto(`/projects/${project.slug}`);
		await page.waitForLoadState('networkidle');

		const title = await page.title();
		expect(title).not.toMatch(UUID_PATTERN);
	});

	test('agent cards do NOT show UUIDs', async ({ page }) => {
		const response = await page.request.get(`${API_BASE}/api/projects`);
		const projects = await response.json();
		expect(projects.length).toBeGreaterThan(0);

		const project = projects[0];
		await page.goto(`/projects/${project.slug}`);
		await page.waitForLoadState('networkidle');

		// Wait for loading to complete
		await expect(page.locator('.animate-pulse').first()).not.toBeVisible({ timeout: 10000 }).catch(() => {});

		// Get all agent card content
		const agentCards = page.locator('.grid > div');
		const cardCount = await agentCards.count();

		if (cardCount > 0) {
			for (let i = 0; i < cardCount; i++) {
				const cardText = await agentCards.nth(i).textContent();
				// UUID patterns should not appear in agent cards
				expect(cardText).not.toMatch(UUID_PATTERN);
			}
		}
	});

	test('code elements do not display full UUIDs', async ({ page }) => {
		await page.goto('/projects');
		await page.waitForLoadState('networkidle');

		// Wait for loading to complete
		await expect(page.locator('.animate-pulse').first()).not.toBeVisible({ timeout: 10000 }).catch(() => {});

		// Find all code elements on the page
		const codeElements = page.locator('code');
		const codeCount = await codeElements.count();

		for (let i = 0; i < codeCount; i++) {
			const codeText = await codeElements.nth(i).textContent();
			// Code elements should NOT contain full UUID patterns
			expect(codeText).not.toMatch(UUID_PATTERN);
		}
	});
});
