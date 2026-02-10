.PHONY: dev build build-rust build-web test test-rust test-web fmt fmt-rust fmt-web check check-rust check-web clean

# ── Dependency chain: fmt → check → test → build ────────────
# make build  will run: fmt → check → test → build
# make test   will run: fmt → check → test
# make check  will run: fmt → check

# ── Development ───────────────────────────────────────────
dev:
	pnpm tauri dev

dev-web:
	pnpm dev

# ── Format ───────────────────────────────────────────────
fmt: fmt-rust fmt-web

fmt-rust:
	cargo fmt --manifest-path src-tauri/Cargo.toml

fmt-web:
	pnpm format

# ── Lint / Check (depends on fmt) ────────────────────────
check: fmt check-rust check-web

check-rust:
	cargo clippy --all-targets --manifest-path src-tauri/Cargo.toml -- -D warnings
	cargo fmt --check --manifest-path src-tauri/Cargo.toml

check-web:
	pnpm format:check
	pnpm lint
	pnpm tsc --noEmit

# ── Test (depends on check) ──────────────────────────────
test: check test-rust test-web

test-rust:
	cargo test --manifest-path src-tauri/Cargo.toml

test-web:
	pnpm test

test-watch:
	pnpm test:watch

# ── Build (depends on test) ──────────────────────────────
build: test build-rust build-web

build-rust:
	cargo build --manifest-path src-tauri/Cargo.toml

build-web:
	pnpm build

build-release:
	cargo build --manifest-path src-tauri/Cargo.toml --release

build-android:
	cargo tauri android build

build-ios:
	cargo tauri ios build

# ── Clean ─────────────────────────────────────────────────
clean:
	cargo clean --manifest-path src-tauri/Cargo.toml
	rm -rf dist

# ── Setup ─────────────────────────────────────────────────
setup:
	pnpm install
	rustup target add aarch64-apple-ios aarch64-apple-ios-sim aarch64-linux-android armv7-linux-androideabi

# ── Misc ──────────────────────────────────────────────────
db-reset:
	rm -f src-tauri/target/debug/cacao.db*
