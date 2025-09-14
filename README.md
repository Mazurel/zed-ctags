# Ctags LSP extension for Zed

Add ctags LSP server to your Zed editor for enhanced code navigation and symbol lookup.
Ctags generates an index of source code symbols (functions, classes, variables) to enable "go to definition" and symbol search features.
You can enable this server by referring to it as `ctags-lsp` in your Zed configuration.

## Features

- **Go to definition**: Jump directly to symbol definitions
- **Symbol search**: Find functions, classes, and variables across your workspace
- **Code navigation**: Navigate through your codebase efficiently
- **Workspace symbol lookup**: Search for symbols across all files in your project
- **Multi-language support**: Works with any language supported by ctags

## Installation

### 1. Install the Extension

> (TODO: This is not ready yet!) Install this extension from the Zed extensions marketplace by searching for "ctags-lsp".

To install this extension locally, you can first clone this repository, and then load it using `zed: install dev extension` action:

```shell
git clone https://github.com/mazurel/zed-ctags.git
```

TIP: You can see installation status by running `zed: open log`.

### 2. Install Ctags

This extension requires `ctags` to be installed on your system.

To install ctags, please refer to the official documentation: https://github.com/universal-ctags/ctags

## Zed Configuration

To enable `ctags-lsp` LSP for `C` language by default, add the following to your `settings.json`:

```json
{
  "languages": {
    "C": {
      "language_servers": ["ctags-lsp"]
    }
  }
}
```

You can also enable `ctags-lsp` for any other ctags-supported language by adding it to the `languages` configuration in your `settings.json`:

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

### Supported Languages

Ctags supports many programming languages including but not limited to:
- C/C++
- Java
- Python
- JavaScript
- TypeScript
- Go
- Rust
- PHP
- Ruby
- And many more...

### Project-specific Configuration

If you want to change these settings only for the current project, you can modify the `languages` configuration in your project's `.zed/settings.json` file.

For further reference, please see: https://zed.dev/docs/configuring-languages#choosing-language-servers

## Special thanks to

- [Universal Ctags](https://github.com/universal-ctags/ctags) for providing the ctags tool.
- [Ctags LSP](https://github.com/netmute/ctags-lsp) for providing the ctags LSP.
- [Zed](https://zed.dev) for providing the editor.
