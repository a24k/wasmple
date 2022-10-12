import { defineConfig } from 'vite';

export default defineConfig({
    plugins: [],
    server: {
        port: 3000,
    },
    build: {
        target: 'esnext',
    },
});
