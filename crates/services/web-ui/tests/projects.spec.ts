import { test, expect } from '@playwright/test';

const API_BASE = 'http://localhost:8000';

// Configure to run tests serially to avoid race conditions
test.describe.configure({ mode: 'serial' });

test.describe('Projects Page', () => {
	test.beforeAll(async ({ request }) => {
		// Create dummy test data via API
		console.log('Creating test project...');
		const projectRes = await request.post(`${API_BASE}/api/project/ensure`, {
			data: { human_key: '/test/playwright-e2e-test' }
		});
		expect(projectRes.ok()).toBeTruthy();
		const project = await projectRes.json();
		console.log('Created project:', project);

		// Create dummy agent (ignore if already exists)
		console.log('Creating test agent...');
		const agentRes = await request.post(`${API_BASE}/api/agent/register`, {
			data: {
				project_slug: project.slug,
				name: 'PlaywrightTestAgent',
				program: 'playwright',
				model: 'e2e-test',
				task_description: 'E2E testing with Playwright'
			}
		});
		// Agent may already exist from previous run - that's OK
		if (agentRes.ok()) {
			const agent = await agentRes.json();
			console.log('Created agent:', agent);
		} else {
			console.log('Agent already exists or error:', await agentRes.text());
		}
	});

	test('should display projects list', async ({ page }) => {
		// Capture browser console logs
		const consoleLogs: string[] = [];
		page.on('console', msg => {
			consoleLogs.push(`[${msg.type()}] ${msg.text()}`);
		});

		// Capture JavaScript errors
		const pageErrors: string[] = [];
		page.on('pageerror', error => {
			pageErrors.push(`[PAGE ERROR] ${error.message}`);
			console.log('[PAGE ERROR]', error.message, error.stack);
		});

		// Capture network requests
		const networkRequests: string[] = [];
		page.on('request', request => {
			networkRequests.push(`>> ${request.method()} ${request.url()}`);
		});
		page.on('response', response => {
			networkRequests.push(`<< ${response.status()} ${response.url()}`);
		});

		// Navigate to projects page
		await page.goto('/projects');

		// Wait for the page to be fully loaded
		await page.waitForLoadState('networkidle');

		// Check page title is visible
		await expect(page.getByRole('heading', { name: 'Projects' })).toBeVisible();

		// Take screenshot showing initial state
		await page.screenshot({ path: 'test-results/projects-initial.png' });

		// Check API is working directly
		const response = await page.request.get(`${API_BASE}/api/projects`);
		const projects = await response.json();
		console.log('API returned projects:', JSON.stringify(projects, null, 2));
		expect(projects.length).toBeGreaterThan(0);

		// Wait for either the table OR the loading spinner to disappear
		// The loading spinner has class "animate-spin"
		const loadingSpinner = page.locator('.animate-spin');

		// Wait for loading to finish (spinner should disappear)
		try {
			await expect(loadingSpinner).not.toBeVisible({ timeout: 10000 });
		} catch (e) {
			// Take screenshot if still loading
			await page.screenshot({ path: 'test-results/projects-still-loading.png' });
			console.log('Page is still loading after 10 seconds');
		}

		// Take screenshot after waiting
		await page.screenshot({ path: 'test-results/projects-after-wait.png' });

		// Check current page state
		const pageContent = await page.content();
		console.log('Page contains table:', pageContent.includes('<table'));
		console.log('Page contains "No projects yet":', pageContent.includes('No projects yet'));
		console.log('Page contains loading spinner:', pageContent.includes('animate-spin'));

		// Log console output
		console.log('\n=== BROWSER CONSOLE ===');
		consoleLogs.forEach(log => console.log(log));

		// Log network requests
		console.log('\n=== NETWORK REQUESTS ===');
		networkRequests.forEach(req => console.log(req));

		// Log page errors
		console.log('\n=== PAGE ERRORS ===');
		if (pageErrors.length === 0) {
			console.log('No page errors captured');
		} else {
			pageErrors.forEach(err => console.log(err));
		}

		// Wait for projects to load and display
		// Either we see the table or the empty state
		const table = page.locator('table');
		const emptyState = page.locator('text=No projects yet');

		// Wait for one of these to appear
		await expect(table.or(emptyState)).toBeVisible({ timeout: 15000 });

		// Check if table exists (projects loaded)
		const isTableVisible = await table.isVisible();
		console.log('Table visible:', isTableVisible);

		if (isTableVisible) {
			// Verify at least one project row exists
			const rows = page.locator('tbody tr');
			const rowCount = await rows.count();
			console.log(`Found ${rowCount} project rows`);
			expect(rowCount).toBeGreaterThan(0);

			// Verify project slug is visible
			const firstRow = rows.first();
			await expect(firstRow).toBeVisible();

			// Take final screenshot showing projects
			await page.screenshot({ path: 'test-results/projects-table.png' });
		} else {
			// Log what we see instead
			const bodyText = await page.locator('body').textContent();
			console.log('Page content:', bodyText?.substring(0, 1000));
			await page.screenshot({ path: 'test-results/projects-empty.png' });
			throw new Error('Projects table not visible - showing empty state despite API having projects');
		}
	});

	test('should navigate to project agents page', async ({ page }) => {
		await page.goto('/projects');
		await page.waitForLoadState('networkidle');

		// Wait for table to load
		const table = page.locator('table');
		await expect(table).toBeVisible({ timeout: 10000 });

		// Click on first project link
		const projectLink = page.locator('table tbody tr a').first();

		if (await projectLink.isVisible()) {
			// Get the project slug from the href
			const href = await projectLink.getAttribute('href');
			console.log('Clicking project link:', href);
			const projectSlug = href?.replace('/projects/', '') || '';
			await projectLink.click();

			// Should navigate to project page
			await page.waitForURL(/\/projects\/.+/);
			await page.screenshot({ path: 'test-results/project-detail.png' });

			// Should show project slug in the page heading
			const heading = page.getByRole('heading', { level: 1 }).filter({ hasText: projectSlug });
			await expect(heading).toBeVisible({ timeout: 10000 });
		}
	});

	test('API health check', async ({ request }) => {
		const response = await request.get(`${API_BASE}/health`);
		expect(response.ok()).toBeTruthy();
		const data = await response.json();
		expect(data.status).toBe('healthy');
		console.log('Health check:', data);
	});

	test('API projects endpoint returns data', async ({ request }) => {
		const response = await request.get(`${API_BASE}/api/projects`);
		expect(response.ok()).toBeTruthy();
		const projects = await response.json();
		console.log('Projects from API:', projects);
		expect(Array.isArray(projects)).toBeTruthy();
		expect(projects.length).toBeGreaterThan(0);
	});
});
