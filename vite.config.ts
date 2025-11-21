import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';

const host = process.env.TAURI_DEV_HOST;

export default defineConfig(async () => ({
	plugins: [sveltekit(), tailwindcss()],
	build: {
		rollupOptions: {
			external: ['@tauri-apps/plugin-process']
		}
	}
}));
