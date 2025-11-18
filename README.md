# Cacao Monorepo

Cacao is a family allowance management platform. The current MVP combines a Go REST API with an Expo (React Native) mobile app so that a "giver" can configure wallets/allowances and a "baby" can request reimbursements.

## Project Status
- ✅ Go API skeleton (Gin router + `/health`) builds successfully.
- ✅ Expo mobile app boots via Expo Router and includes placeholder Dashboard/Requests/Settings tabs.
- ✅ Shared monorepo scaffolding (`cmd/`, `internal/`, `apps/mobile`, `shared/*`, `infra/*`, etc.) matches the system design spec.
- ⌛ Data persistence, real API endpoints, and automated CI/CD are still under active development.

See `doc/setup-log.md` for the chronological list of commands executed when this repo was initialized.

## Repository Structure
```
cmd/api               # Go entrypoint (wire up router/server)
internal/platform     # config, router, server scaffolding
internal/<domain>     # placeholders for auth, families, wallets, ...
apps/mobile           # Expo Router app (TypeScript)
shared/               # reserved for OpenAPI schema, UI kit, generated clients
infra/                # db bootstrap, GitHub Actions, EAS configs
migrations/           # database schema migrations
configs/              # env templates and app configs
doc/                  # product guide, system design, setup log
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
 go mod tidy

# Mobile dependencies
 npm --prefix apps/mobile install
```

## Running the Stack
### API (Go)
```bash
npm run dev:api
```
This is a thin wrapper around `go run ./cmd/api`. The server listens on `:8080` by default (configure via `CACAO_API_PORT`). Visit `http://localhost:8080/health` to verify.

### Mobile App (Expo)
```bash
npm run dev:mobile
```
This proxies to `npm --prefix apps/mobile run start`, launching Expo Dev Tools. From there you can:
- press `a` to open Android emulator
- press `w` to launch the web preview
- use the Expo Go app on a device to scan the QR code

### Environment Variables
Create a `.env` (or platform-specific `.env.local`) with values such as:
```
CACAO_ENV=development
CACAO_API_PORT=8080
CACAO_ALLOW_CORS=true
EXPO_PUBLIC_API_URL=http://localhost:8080
```
The Expo app reads `app.json -> extra.apiUrl` while the Go server uses the `CACAO_*` variables via `internal/platform/config`.

## Testing & Linting
```bash
# Go
 go test ./...

# Mobile
 npm --prefix apps/mobile run lint
 npm --prefix apps/mobile run typecheck
```

## Troubleshooting
- **Expo lint reports BOM issues**: ensure files are saved without BOM (UTF-8). `node -e` scripts in `doc/setup-log` show how we stripped BOMs.
- **`expo-router` dependency conflicts**: always run `npx expo install <package>` so that versions match the current SDK (54.x).
- **Permission errors removing `apps/mobile`**: on Windows, use the `\\?\` long-path prefix (see `doc/setup-log.md`).

## Contributing
1. Read `doc/product-guide.md` (MVP scope) and `doc/sys-design.md` (architecture).
2. Log new setup/infra commands in `doc/setup-log.md` so future engineers can reproduce your steps.
3. Submit PRs targeting the `main` branch; ensure Go builds, Expo lint/typecheck, and any relevant tests pass.

## Roadmap Highlights
- Wire Go API to MySQL/SQLite using the schema defined in `infra/db/CacaoInit.sql`.
- Publish OpenAPI specs in `shared/api-schema` and generate client SDKs.
- Add GitHub Actions workflows in `infra/github` (lint/test/build/EAS submission).
- Implement real wallet/allowance/request flows on both backend and mobile.
