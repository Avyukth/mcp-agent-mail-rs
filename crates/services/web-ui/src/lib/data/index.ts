/**
 * Data Provider Index
 *
 * Build-time conditional export that enables Vite tree-shaking.
 *
 * Usage:
 *   import { dataProvider } from '$lib/data';
 *   const projects = await dataProvider.getProjects();
 *
 * Build modes:
 *   - VITE_DATA_MODE=api    → Uses apiProvider (fetch from /api/*)
 *   - VITE_DATA_MODE=static → Uses staticProvider (bundled JSON)
 *
 * CRITICAL: The conditional MUST use import.meta.env directly for tree-shaking.
 * DO NOT extract to a variable first.
 */

import type { DataProvider, DashboardStats, StaticDataMeta } from './provider';

// Re-export types for convenience
export type { DataProvider, DashboardStats, StaticDataMeta };
export { isStaticMode } from './provider';

// ============================================================================
// Build-time Provider Selection
// ============================================================================

/**
 * CRITICAL TREE-SHAKING PATTERN:
 *
 * This pattern ensures Vite eliminates unused code at build time:
 * - When VITE_DATA_MODE='api': static-provider.ts is NOT included
 * - When VITE_DATA_MODE='static': api-provider.ts is NOT included
 *
 * The `import.meta.env.VITE_DATA_MODE` check MUST be inline.
 * DO NOT refactor to: const mode = import.meta.env.VITE_DATA_MODE
 */

let _provider: DataProvider | null = null;

async function getProvider(): Promise<DataProvider> {
	if (_provider) return _provider;

	// CRITICAL: Direct env check for tree-shaking
	if (import.meta.env.VITE_DATA_MODE === 'static') {
		const { staticProvider } = await import('./static-provider');
		_provider = staticProvider;
	} else {
		const { apiProvider } = await import('./api-provider');
		_provider = apiProvider;
	}

	return _provider;
}

// ============================================================================
// Synchronous Provider Access
// ============================================================================

/**
 * Get the data provider synchronously.
 * Must call initDataProvider() first in +layout.ts
 */
export function getDataProvider(): DataProvider {
	if (!_provider) {
		throw new Error(
			'DataProvider not initialized. Call initDataProvider() in +layout.ts or use getProviderAsync().'
		);
	}
	return _provider;
}

/**
 * Initialize the data provider (call once in +layout.ts)
 */
export async function initDataProvider(): Promise<DataProvider> {
	return getProvider();
}

/**
 * Get data provider asynchronously (safe alternative)
 */
export async function getProviderAsync(): Promise<DataProvider> {
	return getProvider();
}

// ============================================================================
// Convenience Exports
// ============================================================================

/**
 * Proxy object that lazily initializes the provider.
 * Use this for simple cases where async init is acceptable.
 */
export const dataProvider: DataProvider = new Proxy({} as DataProvider, {
	get(_target, prop: keyof DataProvider) {
		return async (...args: unknown[]) => {
			const provider = await getProvider();
			const method = provider[prop];
			if (typeof method === 'function') {
				return (method as (...args: unknown[]) => unknown).apply(provider, args);
			}
			return method;
		};
	}
});

// ============================================================================
// Re-export Types from api/types for convenience
// ============================================================================

export type {
	Project,
	Agent,
	Message,
	Thread,
	ThreadSummary,
	UnifiedInboxResponse,
	UnifiedInboxMessage,
	ActivityItem,
	ArchiveCommit,
	ArchiveFile,
	ToolMetric,
	ToolStats,
	FileReservation,
	Attachment
} from '$lib/api/types';
