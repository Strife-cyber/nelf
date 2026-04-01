# Nelf Backend

Rust backend API for Nelf, designed to run behind Traefik and serve the frontend at `https://nelf.strife-cyber.org`.

## Stack

- Rust
- Axum
- SeaORM + PostgreSQL
- S3-compatible object storage
- Docker + Docker Compose
- Traefik with Let's Encrypt

## Required environment variables

The app reads configuration from environment variables and will fail fast if critical values are missing.

| Variable | Purpose |
| :-- | :-- |
| `APP_HOST` | Host interface to bind the API server to. Use `0.0.0.0` in containers. |
| `APP_PORT` | Port the API listens on. Default deployment value is `3000`. |
| `CORS_ALLOWED_ORIGINS` | Comma-separated browser origins allowed to call the API. Production value: `https://nelf.strife-cyber.org`. |
| `ENABLE_SWAGGER` | Enables `/swagger-ui`. Keep this `false` in production. |
| `DATABASE_URL` | PostgreSQL connection string. |
| `JWT_SECRET` | Secret used to sign and validate JWTs. Must be set to a strong random value. |
| `AWS_ACCESS_KEY_ID` | S3 access key. |
| `AWS_SECRET_ACCESS_KEY` | S3 secret key. |
| `AWS_DEFAULT_REGION` | S3 region. |
| `AWS_BUCKET` | Bucket name for uploaded assets. |
| `AWS_URL` | Base URL for your S3-compatible storage endpoint. |
| `AWS_USE_PATH_STYLE_ENDPOINT` | Path-style S3 access toggle. Keep `true` for many S3-compatible providers. |

Use [.env.example](C:\Users\dunam\RustroverProjects\nelf\.env.example) as the template. The tracked [.env](C:\Users\dunam\RustroverProjects\nelf\.env) now contains safe placeholders only.

## Local development

1. Fill in the values in `.env`.
2. Start PostgreSQL and your S3-compatible storage.
3. Run the API:

```bash
cargo run
```

The API will start on `APP_HOST:APP_PORT`.

For local frontend testing, you can temporarily set:

```env
CORS_ALLOWED_ORIGINS=http://localhost:4321
ENABLE_SWAGGER=true
```

Swagger will then be available at `http://127.0.0.1:3000/swagger-ui/` if you keep the default local port.

## Docker

Build the image:

```bash
docker build -t nelf-api .
```

Run the container directly:

```bash
docker run --env-file .env --name nelf-api --restart unless-stopped nelf-api
```

The container listens on port `3000` internally and is meant to be reached through Traefik, not by publishing a host port.

## Docker Compose deployment with external Traefik

This repository includes [docker-compose.yml](C:\Users\dunam\RustroverProjects\nelf\docker-compose.yml) for an API-only deployment.

Assumptions baked into the compose file:

- Traefik runs in a separate container.
- Traefik and this API share the external Docker network `traefik-net`.
- The public API domain is `api.nelf.strife-cyber.org`.
- The HTTPS entrypoint name is `https`.
- The Let's Encrypt resolver name is `letsencrypt`.

Deploy steps on the VPS:

1. Make sure the external network exists:

```bash
docker network create traefik-net
```

2. Copy the repo to the VPS.
3. Fill in production values in `.env`.
4. Start or update the service:

```bash
docker compose up -d --build
```

Traefik will route `https://api.nelf.strife-cyber.org` to the backend container on port `3000` over `traefik-net`.

## Production notes

- Leave `ENABLE_SWAGGER=false` in production unless you intentionally want public API docs.
- Do not commit real secrets to `.env`.
- PostgreSQL and object storage are external dependencies and are not bundled into the compose stack.
- `JWT_SECRET` should be a long, random secret generated specifically for production.
