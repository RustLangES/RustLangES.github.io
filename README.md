![image](https://github.com/RustLangES/RustLangES.github.io/assets/56278796/cc7009a2-a11b-4847-a561-fcc6807e1d98)


<p align="center">
<img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/RustLangES/RustLangES.github.io/clippy.yml?label=ci" />
<img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/RustLangES/RustLangES.github.io/gh-pages.yml?label=deploy" />
</p>

> [!CAUTION]
> Esta pagina no se publicará hasta que el nuevo diseño se encuentre listo
> https://www.figma.com/design/S9yCZSaZ9q54XSojWNhJft/Rust-Lang-ES?node-id=0-1&p=f&t=Cn6Q0QzGflLYCOgo-0

## Como ejecutar


### Requisitos Generales 
Antes de empezar es necesario tener estos programas
- [Rust](https://rust-lang.org/tools/install)
- [NodeJs](https://nodejs.org)

### Requisitos Windows >= 10 
-[BusyBox](https://busybox.net/)
	- Usando scoop `scoop install busybox`
	- Usando choco  `choco install busybox`

### Desarrollo
> [!NOTE]
> necesitas fetch git submodules para clonar los assets externos para el desarrollo 


Con estos comandos podrá empezar a desarrollar

```bash
git submodule update --init --recursive
rustup toolchain install nightly
rustup default nightly
rustup target add wasm32-unknown-unknown


cd .. && git clone https://github.com/RustLangES/design-system-components
cd design-system-components && pnpm install
cd styles && pnpm run build

cd ../../RustLangES.github.io && npm install

# Sin cargo make:
cargo leptos serve --hot-reload --features development

# Con cargo make:
cargo install cargo-make
cargo make serve
```


---


### Si usas nix

> [!NOTE]
> Asegúrate de tener los flakes activados.

Si usas nix es bastante sencillo, solo necesitas este comando:
```bash
nix develop
```

Ahora podemos iniciar el servidor con:
```bash
cargo make serve
```

Para hacer un commit:
```bash
cargo install rusty-hook
cargo install leptosfmt --version 0.1.13
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



# Resumen
Este proyecto utiliza una rama personalizada de Leptos para poder servir directamente el directorio de salida como un sitio web estático.

`cargo make serve` sirve el directorio div con watch mode y hot-reload. 
`cargo make build` compila el proyecto en release. La salida estará en el directorio `dist` y el comando no lo servirá, sino que se cerrará. 
`cargo make fmt` formatea con `rustfmt` y `leptosfmt`.
