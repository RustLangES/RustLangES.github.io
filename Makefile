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
# Build & Serve
# ----------------------

prebuild:
	cp node_modules/@rustlanges/styles/dist/bundled.css bundled.css

build: prebuild
	$(CARGO) leptos serve -r --split

serve: prebuild
	$(CARGO) leptos watch --features development --hot-reload
