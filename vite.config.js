import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
    build: {
        rollupOptions: {
            external: [
                '@tauri-apps/plugin-process'
            ]
        }
    },
	plugins: [sveltekit(), tailwindcss()],
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	}
});
