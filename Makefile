CARGO ?= cargo
.DEFAULT_GOAL := serve

# ----------------------
# Formatting tasks
# ----------------------

cargo-format:
	$(CARGO) fmt --all

leptos-format:
	leptosfmt .

format: cargo-format leptos-format

fmt: format

fmt-all:
	leptosfmt .
	$(CARGO) fmt --all
	$(CARGO) clippy --fix -- -D warnings

check:
	leptosfmt --check .
	$(CARGO) fmt --all --check
	$(CARGO) clippy -- -D warnings

# ----------------------
# Utilidades
# ----------------------

get-sitemap:
	rm -f assets/sitemap.xml
	wget -S -P assets https://github.com/Phosphorus-M/sitemap-rustico/releases/download/latest/sitemap.xml

# ----------------------
# Diagnóstico
# ----------------------

doctor:
	@echo "== RustLangES doctor =="
	@command -v rustup >/dev/null 2>&1 || { echo "  ✗ rustup not found — https://rust-lang.org/tools/install"; exit 1; }; echo "  ✓ rustup $$(rustup --version 2>&1 | head -n1)"
	@rustup toolchain list 2>/dev/null | grep -q "nightly" && echo "  ✓ nightly toolchain" || echo "  ✗ nightly toolchain missing — rustup toolchain install nightly-2026-08-05"
	@rustup target list --installed 2>/dev/null | grep -q "wasm32-unknown-unknown" && echo "  ✓ wasm32-unknown-unknown" || echo "  ✗ wasm target missing — rustup target add wasm32-unknown-unknown"
	@command -v pnpm >/dev/null 2>&1 || { echo "  ✗ pnpm not found — npm i -g pnpm or https://pnpm.io/installation"; exit 1; }; echo "  ✓ pnpm $$(pnpm --version 2>&1 | head -n1)"
	@if command -v cargo-leptos >/dev/null 2>&1; then echo "  ✓ cargo-leptos $$(cargo leptos --version 2>&1 | head -n1)"; else echo "  ✗ cargo-leptos missing — cargo install cargo-leptos --version 0.3.2"; fi
	@if command -v wasm-bindgen >/dev/null 2>&1; then echo "  ✓ wasm-bindgen $$(wasm-bindgen --version 2>&1 | head -n1)"; else echo "  ! wasm-bindgen-cli missing — cargo install wasm-bindgen-cli --version 0.2.126"; fi
	@grep -A1 'name = "wasm-bindgen"$$' Cargo.lock 2>/dev/null | grep -q 'version = "0.2.126"' && echo "  ✓ Cargo.lock wasm-bindgen 0.2.126" || echo "  ! Cargo.lock wasm-bindgen version check skipped"
	@test -d extras && test -n "$$(ls -A extras 2>/dev/null)" && echo "  ✓ extras/" || echo "  ✗ extras/ empty — git submodule update --init --recursive"
	@test -f design-system-components/Cargo.toml && echo "  ✓ design-system-components/" || echo "  ✗ design-system-components/ missing — git submodule update --init --recursive"
	@test -f node_modules/@rustlanges/styles/dist/bundled.css && echo "  ✓ node_modules/@rustlanges/styles/dist/bundled.css" || echo "  ✗ bundled.css not built — cargo make setup"
	@test -f bundled.css && echo "  ✓ bundled.css" || echo "  ✗ bundled.css missing at repo root — pnpm run postinstall"
	@echo "done."

# ----------------------
# Configuración
# ----------------------

setup:
	git submodule update --init --recursive
	rm -rf node_modules package-lock.json
	pnpm install --ignore-scripts
	cd design-system-components && pnpm install --ignore-scripts
	cd design-system-components/styles && pnpm run build
	pnpm run postinstall
	@echo ""
	@echo "Setup complete. Run 'make doctor' to verify, then 'make serve'."

# ----------------------
# Build & Serve
# ----------------------

prebuild:
	@test -f node_modules/@rustlanges/styles/dist/bundled.css || (echo ""; echo "error: node_modules/@rustlanges/styles/dist/bundled.css not found."; echo "  Did you run 'cargo make setup' (or 'make setup') on first clone?"; echo "  Quick fix: cargo make setup"; echo ""; exit 1)
	cp node_modules/@rustlanges/styles/dist/bundled.css bundled.css

build: prebuild
	$(CARGO) leptos serve -r --split

serve: prebuild
	$(CARGO) leptos watch --features development --hot-reload
