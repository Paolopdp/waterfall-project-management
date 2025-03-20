### VSCODE configuration for Rust

```json
// .vscode/settings.json
{
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true
  },
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.procMacro.enable": true,
  "rust-analyzer.cargo.allFeatures": true,
  "rust-analyzer.completion.autoimport.enable": true
}
```
