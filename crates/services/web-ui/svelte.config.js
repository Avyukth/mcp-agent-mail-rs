import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
	kit: {
		adapter: adapter({
			pages: 'build',
			assets: 'build',
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
