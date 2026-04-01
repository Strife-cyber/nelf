# Nelf

Nelf is an Astro + React site with a public marketing/portfolio homepage and an admin dashboard for managing content (flyers, videos, users, website previews).

## Tech stack

- Astro 6
- React (islands) via `@astrojs/react`
- Tailwind CSS (via `@tailwindcss/vite`)

## Requirements

- Node.js `>= 22.12.0` (see `package.json`)
- npm

## Getting started

```bash
npm install
cp .env.example .env
npm run dev
```

Dev server runs at `http://localhost:4321`.

## Environment variables

This project expects a backend API. Configure the base URL using:

- `PUBLIC_API_BASE_URL` (must include `/api`)
  - Example: `http://localhost:3000/api`
  - If not set, the app falls back to `http://localhost:3000/api`

Because this value is used in browser code, it must use the `PUBLIC_` prefix (Astro only exposes `PUBLIC_*` env vars to the client).

## Routes

- `/` - public homepage (fetches flyers/videos/website previews from the API)
- `/privacy` - privacy policy
- `/terms` - terms of service
- `/admin/login` - admin sign-in (uses API credentials)
- `/admin` - redirects to `/admin/flyers`
- `/admin/:resource` - admin CRUD pages (`flyers`, `videos`, `users`, `website-previews`)
- `/admin/thumbnail-generator` - helper tool for generating video thumbnails
- `/admin/logout` - clears the stored token and returns to login

## Backend API expectations (high level)

The frontend uses `src/lib/api` and expects endpoints under `PUBLIC_API_BASE_URL`, including:

- `POST /auth/login` (returns `{ access_token, token_type }`)
- CRUD endpoints: `/flyers`, `/videos`, `/users`, `/website-previews`

Some UI flows also call helper endpoints like:

- `PATCH /{resource}/{id}/toggle-active`
- `GET /{resource}/active`
- `POST /website-previews/{id}/refresh`

## Commands

| Command           | Action |
| :---------------- | :----- |
| `npm run dev`     | Start dev server |
| `npm run build`   | Production build |
| `npm run preview` | Preview build |

## Project structure (quick map)

```text
src/
  components/         UI components (Astro + React)
  layouts/            Base layout
  lib/api/            API client + services (auth + CRUD)
  pages/              Site + admin routes
  styles/             Global styles
  types/              Shared TypeScript types
```

## Deployment note

Some pages use `export const prerender = false` (SSR at request time). That means `npm run build` will fail until you install and configure a server adapter.

Example (Node server):

```bash
npm i -D @astrojs/node
```

Then update `astro.config.mjs` to include a Node adapter and `output: 'server'` (see Astro docs for the latest recommended settings).
