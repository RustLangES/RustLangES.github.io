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
