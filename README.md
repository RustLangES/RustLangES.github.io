![image](https://github.com/RustLangES/RustLangES.github.io/assets/56278796/cc7009a2-a11b-4847-a561-fcc6807e1d98)


<p align="center">
<img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/RustLangES/RustLangES.github.io/clippy.yml?label=ci" />
<img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/RustLangES/RustLangES.github.io/gh-pages.yml?label=deploy" />
</p>

## Requisitos Generales
antes de empezar tienes que instalar estos programas
- [Rust](https://rust-lang.org/tools/install)
- [NodeJs](https://nodejs.org)
- [Trunk](https://trunk.dev)

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
```sh
trunk serve
```


```bash 
npm install 
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

