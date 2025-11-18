# Cacao Monorepo

Cacao is a family allowance management platform. The current MVP combines a Go REST API with an Expo (React Native) mobile app so that a "giver" can configure wallets/allowances and a "baby" can request reimbursements.

## Project Status
- [x] Go API (Gin) lives under `server/` and exposes `/health` plus `/api/v1/auth/login` with the seeded `amanda / 1234` credentials.
- [x] Expo mobile app performs a real login call before rendering the dashboard tabs; Settings now includes a basic sign-out.
- [x] Monorepo scaffolding (`server/*`, `apps/*`, `shared/*`, `infra/*`, `docs/*`) matches the system design spec.
- [ ] Data persistence, domain APIs, and automated CI/CD are still under active development.

See `docs/sys-design.md` for the architecture blueprint and decision history.

## Repository Structure
```
server/               # Go module (cmd/, internal/, configs/, migrations/, tools/)
apps/
  mobile/             # Expo Router app (React Native)
  web-admin/          # Placeholder for the future React admin console
shared/               # OpenAPI schema, generated SDKs, design tokens
infra/                # docker-compose, GitHub Actions, infra as code
docs/                 # product guide, system design, 30-day plan
```

## Prerequisites
- Go 1.21+
- Node.js 20+ (ships with npm)
- Expo CLI (`npx expo` is enough)
- SQLite (for local dev) or MySQL 8 if you want to wire the backend to a DB

## Quick Start
```bash
# clone the repo
 git clone <repo-url>
 cd Cacao

# Go dependencies
 (cd server && go mod tidy)

# Mobile dependencies
 npm --prefix apps/mobile install
```

## Running the Stack
### API (Go)
```bash
npm run dev:api
```
This runs `go run ./cmd/api` from the `server/` module. The server listens on `:8080` by default (configure via `CACAO_API_PORT`). Visit `http://localhost:8080/health` or exercise the login endpoint below to verify end-to-end auth.

### Authentication Smoke Test
```bash
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"amanda","password":"1234"}'
```
Successful responses include a short-lived token and the seeded admin profile. Update `CACAO_ADMIN_USERNAME`, `CACAO_ADMIN_PASSWORD`, and `CACAO_ADMIN_DISPLAY_NAME` to change the bootstrap account.

### Mobile App (Expo)
```bash
npm run dev:mobile
```
This proxies to `npm --prefix apps/mobile run start`, launching Expo Dev Tools. From there you can:
- press `a` to open Android emulator
- press `w` to launch the web preview
- use the Expo Go app on a device to scan the QR code

## Environment Variables
```
CACAO_ENV=development
CACAO_API_PORT=8080
CACAO_ALLOW_CORS=true
CACAO_ADMIN_USERNAME=amanda
CACAO_ADMIN_PASSWORD=1234
CACAO_SESSION_SECRET=dev-secret
CACAO_ADMIN_DISPLAY_NAME=Amanda
EXPO_PUBLIC_API_URL=http://localhost:8080
```
The Expo app reads the `EXPO_PUBLIC_*` values while the Go server ingests the `CACAO_*` settings via `server/internal/platform/config`.

## Testing & Linting
```bash
# Go (from repo root)
 cd server && go test ./...

# Mobile
 npm --prefix apps/mobile run lint
 npm --prefix apps/mobile run typecheck
```

## Troubleshooting
- **Expo lint reports BOM issues**: ensure files are saved without BOM (UTF-8) and note any fixes inside `docs/sys-design.md`.
- **`expo-router` dependency conflicts**: always run `npx expo install <package>` so that versions match the current SDK (54.x).
- **Windows path woes**: use the `\\?\` long-path prefix when deleting `apps/mobile` and document the workaround in `docs/plan-30d.md`.

## Contributing
1. Read `docs/product-guide.md` (MVP scope) and `docs/sys-design.md` (architecture) before coding.
2. Document new setup/infra commands inline within the relevant `docs/*.md` files so future engineers can reproduce your steps.
3. Submit PRs targeting the `main` branch; ensure Go builds, Expo lint/typecheck, and any relevant tests pass.

## Roadmap Highlights
- Wire Go API to MySQL/SQLite using the schema defined in `infra/db/CacaoInit.sql`.
- Publish OpenAPI specs in `shared/api-schema` and generate client SDKs.
- Add GitHub Actions workflows in `infra/github` (lint/test/build/EAS submission).
- Implement real wallet/allowance/request flows across backend and mobile.
