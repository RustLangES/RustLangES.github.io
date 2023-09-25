![image](https://github.com/RustLangES/RustLangES.github.io/assets/56278796/cc7009a2-a11b-4847-a561-fcc6807e1d98)


<p align="center">
<img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/RustLangES/RustLangES.github.io/clippy.yml?label=ci" />
<img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/RustLangES/RustLangES.github.io/gh-pages.yml?label=deploy" />
</p>

## Desarrollo
> [!NOTE]
> You need run `git submodule update --init --recursive` to get external assets for development generation
Now you can run:
```sh
trunk serve
```

## Requisitos
- [Rust](https://rust-lang.org/tools/install)
- [NodeJs](https://nodejs.org)
    ```bash
    npm install -D tailwindcss
    ```
- [Trunk](https://trunk.dev)

## Configura tu VSCode
Agrega esto en tu `settings.json`

```json
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
  "css.validate": false,
```
