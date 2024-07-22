![image](https://github.com/RustLangES/RustLangES.github.io/assets/56278796/cc7009a2-a11b-4847-a561-fcc6807e1d98)


<p align="center">
<img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/RustLangES/RustLangES.github.io/clippy.yml?label=ci" />
<img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/RustLangES/RustLangES.github.io/gh-pages.yml?label=deploy" />
</p>

## Requisitos Generales
antes de empezar tienes que instalar estos programas
- [Rust](https://rust-lang.org/tools/install)
- [NodeJs](https://nodejs.org)

## Requisitos Windows >= 10
- [BusyBox](https://busybox.net/)
  - try: `scoop install busybox`
  - try: `choco install busybox`

## Desarrollo
> [!NOTE]
> necesitas fetch git submodules para clonar los assets externos para el desarrollo 

```sh
git submodule update --init --recursive
```

Now you can run:

```bash
rustup toolchain install nightly
```
```bash
rustup default nightly
```
```bash
rustup target add wasm32-unknown-unknown
```
```bash
npm install
```
```bash
cargo install cargo-make
cargo make serve
```

For do a commit:
```
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



# Summary

This project uses a custom branch of Leptos to be able to directly serve the output directory as a static website.

`cargo make serve` serves the div directory with watch mode and hot-reload enabled.
`cargo make build` builds the project in release. The output will be in the `dist` directory and the command will not serve it, but quit instead.
`cargo make fmt` formats with `rustfmt` and `leptosfmt`.
