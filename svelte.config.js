// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import adapter from '@sveltejs/adapter-static'
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte'

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
	kit: {
		adapter: adapter({
			fallback: '200.html',
		}),
		prerender: {
			entries: ['*'],
		},
		alias: {
			$models: './src/models',
			$invoke: './src/invoke',
			$routes: './src/routes/routes.ts',
			$theme: './src/theme',
			$components: './src/components',
			$queries: './src/queries',
			$context: './src/routes/state.svelte.ts',
			$pages: './src/routes',
			$utils: './src/utils.ts',
		},
	},
}

export default config
