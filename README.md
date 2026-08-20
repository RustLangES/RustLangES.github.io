# ![image](https://github.com/RustLangES/RustLangES.github.io/assets/56278796/cc7009a2-a11b-4847-a561-fcc6807e1d98)

<div align="center">
    
[![ci](https://github.com/RustLangES/RustLangES.github.io/actions/workflows/clippy.yml/badge.svg)](https://github.com/RustLangES/RustLangES.github.io/actions/workflows/clippy.yml) [![deploy](https://github.com/RustLangES/RustLangES.github.io/actions/workflows/pages.yml/badge.svg)](https://github.com/RustLangES/RustLangES.github.io/actions/workflows/pages.yml)
    
</div>

> [!CAUTION]
> Esta pagina no se publicará hasta que el nuevo diseño se encuentre listo
> <https://www.figma.com/design/S9yCZSaZ9q54XSojWNhJft/Rust-Lang-ES?node-id=0-1&p=f&t=Cn6Q0QzGflLYCOgo-0>

## Como ejecutar

### Requisitos Generales

Antes de empezar es necesario tener estos programas

- **[Rust](https://rust-lang.org/tools/install)**
- **Compilador de C/C++**
- **[NodeJs](https://nodejs.org/en/download) en la version 24**

**Rust:**

```bash
rustup toolchain install nightly
rustup default nightly
rustup target add wasm32-unknown-unknown

cargo install cargo-make
cargo install rusty-hook
cargo install leptosfmt --version 0.1.33
```

> [!IMPORTANT]
> Necesitas `wasm-bindgen-cli` en la versión `0.2.126` (la misma que fija
> `flake.nix`). Si no coincide con el `wasm-bindgen` del `Cargo.lock`,
> `cargo make serve`/`build` falla con un error de "schema version mismatch".
> ```bash
> cargo install -f wasm-bindgen-cli --version 0.2.126
> 

Si usas NixOS

> [!NOTE]
> Asegúrate de tener los flakes activados.

```bash
direnv allow
```

### Requisitos Windows >= 10

[BusyBox](https://busybox.net/)

- Usando scoop `scoop install busybox`
- Usando choco  `choco install busybox`

### Desarrollo

> [!NOTE]
> Necesitas fetch git submodules para clonar los assets externos para el desarrollo

Este proyecto usa `cargo make` para agilizar ciertos procesos y no tener que estar escribiendo comandos manualmente.

Con estos comandos podrá empezar a desarrollar

Para configurar el proyecto en general, se recomienda usar el siguiente comando, ya que este comando engloba todos los camandos listados más abajo:

```bash
cargo make setup
```

O si prefieres ejecutar cada uno de los comandos manualmente:

```bash

git submodule update --init --recursive
pnpm install --ignore-scripts
cd design-system-components && pnpm install --ignore-scripts
cd styles && pnpm run build
cd ../.. && pnpm run postinstall
```

Para ejecutar el proyecto:

Ahora podemos iniciar el servidor con:

```bash
cargo make serve
```

Sin cargo make:
```bash
cargo leptos serve --hot-reload --features development
```

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

## Configura tu Zed

Agrega esto en tu `settings.json`

```json
{
  "lsp": {
    "rust-analyzer": {
      "binary": {
        "path": "rust-analyzer"
      },
      "initialization_options": {
        "cargo": {
          "features": [
            "ssr"
          ]
        },
        "check": {
          "command": "check",
          "extraArgs": [
            "--features",
            "ssr"
          ]
        }
      }
    }
  }
}
```

## Resumen

Este proyecto utiliza una rama personalizada de Leptos para poder servir
directamente el directorio de salida como un sitio web estático.

`cargo make serve` sirve el directorio div con watch mode y hot-reload.
`cargo make build` compila el proyecto en release. La salida estará en el
directorio `dist` y el comando no lo servirá, sino que se cerrará.
`cargo make fmt` formatea con `rustfmt` y `leptosfmt`.
