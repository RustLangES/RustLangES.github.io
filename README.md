# ![image](https://github.com/RustLangES/RustLangES.github.io/assets/56278796/cc7009a2-a11b-4847-a561-fcc6807e1d98)

<!-- markdownlint-disable-next-line MD033 -->
<p align="center">
    <!-- markdownlint-disable-next-line MD033 -->
<img alt="GitHub Workflow Status (with event)"
    src="https://img.shields.io/github/actions/workflow/status/RustLangES/
    RustLangES.github.io/clippy.yml?label=ci" />
<!-- markdownlint-disable-next-line MD033 -->
<img alt="GitHub Workflow Status (with event)"
    src="https://img.shields.io/github/actions/workflow/status/RustLangES/
    RustLangES.github.io/gh-pages.yml?label=deploy" />
</p>

> [!CAUTION]
> Esta pagina no se publicará hasta que el nuevo diseño se encuentre listo
> <https://www.figma.com/design/S9yCZSaZ9q54XSojWNhJft/Rust-Lang-ES?node-id=0-1&p=f&t=Cn6Q0QzGflLYCOgo-0>

## Como ejecutar

### Requisitos Generales

Antes de empezar es necesario tener estos programas

- [Rust](https://rust-lang.org/tools/install) (el `rust-toolchain.toml` fija `nightly-2026-08-05`)
- [NodeJs](https://nodejs.org) + [pnpm](https://pnpm.io/installation) (`npm i -g pnpm`)
- `wasm-bindgen-cli` **0.2.126** — debe coincidir con `Cargo.lock` / `flake.nix`; si no, `cargo leptos` falla con `schema version mismatch`:
  ```bash
  cargo install -f wasm-bindgen-cli --version 0.2.126
  ```

> [!CAUTION]
> **No uses `cargo run` a secas.** El binario del servidor requiere la feature `ssr` (`actix-web`/`leptos_actix` son opcionales). `cargo run` sin features no resuelve esos crates y verás 8 errores `E0432/E0433`. Usa siempre `cargo make serve` o `cargo leptos`. Si lo intentas, ahora verás un mensaje que te dice qué hacer.

### Clonar correctamente (submodules)

Este repo tiene dos submodules: `extras` y `design-system-components`. Si clonas sin ellos `build.rs` falla.

```bash
git clone --recursive https://github.com/RustLangES/RustLangES.github.io.git
cd RustLangES.github.io
# si ya clonaste sin --recursive:
git submodule update --init --recursive
```

Verifica con `make doctor` o `cargo make doctor`.

### Requisitos Windows >= 10

[BusyBox](https://busybox.net/)

- Usando scoop `scoop install busybox`
- Usando choco  `choco install busybox`

### Desarrollo — setup en un comando (recomendado)

Este proyecto usa `Makefile` / `cargo make` para no tener que escribir comandos manualmente.

```bash
rustup toolchain install nightly-2026-08-05
rustup target add wasm32-unknown-unknown

cargo install cargo-make cargo-leptos --version 0.3.2
cargo install leptosfmt --version 0.1.33    # opcional, para fmt
# rusty-hook se instala solo si lo necesitas: cargo install rusty-hook

cargo make setup      # = submodules + pnpm install en design-system + build CSS + postinstall
cargo make doctor     # verifica toolchain, wasm-bindgen, submodules y bundled.css
cargo make serve      # dev server con hot-reload (http://127.0.0.1:4561)
```

Equivalente con `make`:

```bash
make setup && make doctor && make serve
```

<details>
<summary>Setup manual (si prefieres paso a paso)</summary>

```bash
git submodule update --init --recursive
pnpm install --ignore-scripts
cd design-system-components && pnpm install --ignore-scripts
cd styles && pnpm run build   # genera node_modules/@rustlanges/styles/dist/bundled.css
cd ../..
pnpm run postinstall           # copia bundled.css a la raíz
cargo leptos watch --features development --hot-reload
```

</details>

### Cómo ejecutar (según herramienta)

| Comando | Qué hace |
|---|---|
| `cargo make serve` / `make serve` | **Recomendado.** Compila CSS, luego `cargo leptos watch --features development --hot-reload` |
| `cargo make build` / `make build` | Build release en `dist/` (`cargo leptos serve -r --split`) |
| `cargo leptos watch --features development --hot-reload` | Sin `cargo-make`, directo |
| `cargo leptos serve --hot-reload --features development` | Variante `serve` |
| `cargo run --features ssr,development` | Solo si necesitas `cargo run` explícito |

### Nix (alternativa)

> [!NOTE]
> Asegúrate de tener los flakes activados.

```bash
direnv allow   # o nix develop — te da rust nightly, wasm-bindgen 0.2.126, cargo-leptos, pnpm, tailwindcss
cargo make setup
cargo make serve
```

### Troubleshooting

**`cargo run` → `unresolved import leptos_actix / actix_web` (E0432/E0433)**
Ya está arreglado: `Cargo.toml` marca el bin con `required-features = ["ssr"]` y `src/main.rs` muestra un mensaje con el comando correcto. Usa `cargo run --features ssr` o mejor `cargo make serve`.

**`build.rs` → `extras/ not found` / `extras/ is empty` / `unwrap()` panic**
No inicializaste submodules: `git submodule update --init --recursive` o `cargo make setup`.

**`node_modules/@rustlanges/styles/dist/bundled.css not found` / `bundled.css missing`**
No se generó el CSS: `cargo make setup` (o manual `cd design-system-components/styles && pnpm run build`).

**`wasm-bindgen schema version mismatch`**
Tu `wasm-bindgen-cli` no coincide con `Cargo.lock` (0.2.126): `cargo install -f wasm-bindgen-cli --version 0.2.126` — fijado también en `flake.nix` y `rust-toolchain.toml`.

**Dudas del entorno:** `cargo make doctor` / `make doctor` / `pnpm run doctor`

## Configura tu VSCode

Agrega esto en tu `settings.json`

```json
{
  "emmet.includeLanguages": {
    "rust": "html",
    "*.rs": "html"
  },
  "tailwindCSS.includeLanguages": {
      "rust": "html",
      "*.rs": "html"
  },
  "files.associations": {
      "*.rs": "rust"
  },
  "editor.quickSuggestions": {
    "other": "on",
    "comments": "on",
    "strings": true
  },
  "css.validate": false
}
```

## Resumen

Este proyecto utiliza una rama personalizada de Leptos para poder servir
directamente el directorio de salida como un sitio web estático.

`cargo make serve` sirve el directorio div con watch mode y hot-reload.
`cargo make build` compila el proyecto en release. La salida estará en el
directorio `dist` y el comando no lo servirá, sino que se cerrará.
`cargo make fmt` formatea con `rustfmt` y `leptosfmt`.
