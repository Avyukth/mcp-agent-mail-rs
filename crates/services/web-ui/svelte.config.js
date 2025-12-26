import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

// Build mode determines output directory:
// - 'embedded' (default): outputs to build/ for rust-embed
// - 'static': outputs to build-static/ for GitHub Pages
const buildMode = process.env.VITE_BUILD_MODE || 'embedded';
const outputDir = buildMode === 'static' ? 'build-static' : 'build';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
	kit: {
		adapter: adapter({
			pages: outputDir,
			assets: outputDir,
			fallback: 'index.html',
			precompress: true,
			strict: true
		}),
		paths: {
			base: ''
		},
		alias: {
			$components: 'src/lib/components',
			$api: 'src/lib/api',
			$stores: 'src/lib/stores'
		}
	}
};

export default config;
