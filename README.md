# Ctags LSP extensions for Zed

Add ctags LSP server to your Zed editor.
You can enable this server by refering to it as `ctags-lsp` in your Zed configuration.

## Installation

This extensions requires `ctags` to be installed on your system.

To install ctags, please refer to the official documentation: https://github.com/universal-ctags/ctags

## Zed Configuration

To enable `ctags-lsp` LSP for `C` language by default:

```json
  "languages": {
    "C": {
      "language_servers": ["ctags-lsp"]
    }
  },
```

You can also enable `ctags-lsp` for any other ctags-supported language by adding it to the `languages` configuration:

```json
  "languages": {
    "C": {
      "language_servers": ["ctags-lsp"]
    },
    "C++": {
      "language_servers": ["ctags-lsp"]
    },
    "Java": {
      "language_servers": ["ctags-lsp"]
    },
    "Python": {
      "language_servers": ["ctags-lsp"]
    }
  },
```

If you want to change this settings only for current project, you can modify the `languages` configuration in your project's `.zed/settings.json` file.

For further reference, please see: https://zed.dev/docs/configuring-languages#choosing-language-servers

## Special thanks to

- [Universal Ctags](https://github.com/universal-ctags/ctags) for providing the ctags tool.
- [Ctags LSP](https://github.com/netmute/ctags-lsp) for providing the ctags LSP.
- [Zed](https://zed.dev) for providing the editor.
