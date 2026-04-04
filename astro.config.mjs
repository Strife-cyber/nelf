// @ts-check
import { defineConfig } from 'astro/config';
import node from '@astrojs/node';
import { fileURLToPath } from 'node:url';


import react from '@astrojs/react';

import tailwindcss from "@tailwindcss/vite";

const astroPrerenderEntrypoint = fileURLToPath(new URL('./node_modules/astro/dist/entrypoints/prerender.js', import.meta.url));
const astroLegacyEntrypoint = fileURLToPath(new URL('./node_modules/astro/dist/entrypoints/legacy.js', import.meta.url));

// https://astro.build/config
export default defineConfig({
  site: 'https://nelf.strife-cyber.org',
  output: 'server',
  adapter: node({
    mode: 'standalone',
  }),

  vite: {
    plugins: [tailwindcss()],
    resolve: {
      alias: {
        'astro/entrypoints/prerender': astroPrerenderEntrypoint,
        'astro/entrypoints/legacy': astroLegacyEntrypoint,
      },
    },
  },

  integrations: [react()],

  i18n: {
    defaultLocale: "en",
    locales: ["en", "fr"]
  }
});
