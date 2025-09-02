# Helm Cleaner

`helm-cleaner` is a Rust-based CLI tool to **uninstall Helm releases** in Kubernetes namespaces, with optional namespace deletion and interactive selection. It also supports **bash completions** and configurable Helm binaries.  

---

## Features

- Interactive selection of Helm releases in a namespace  
- `<ALL RELEASES>` option to uninstall all releases at once  
- Optional namespace deletion after uninstall  
- Skip confirmation prompts with `--force`  
- Configurable Helm binary via `--helm-bin` or `HELM_BIN` environment variable  
- Supports bash completions (`helm-cleaner completions`)  

---

## Installation

$ cargo install --git https://github.com/krille0x7c2/helm-cleaner.git

---

## Optional: Generate shell completions

`helm-cleaner` can generate shell completions for Bash, Zsh, and Fish. This allows tab completion for commands and flags.

---

### Bash

```bash
helm-cleaner completions > helm-cleaner.bash
source helm-cleaner.bash
sudo cp helm-cleaner.bash /etc/bash_completion.d/helm-cleaner

---

#### Zsh

```zsh
helm-cleaner completions > _helm-cleaner
mkdir -p ~/.zsh/completions
mv _helm-cleaner ~/.zsh/completions/

---

#### Fish

Generate completions for Fish:

```fish
helm-cleaner completions > helm-cleaner.fish
mkdir -p ~/.config/fish/completions
mv helm-cleaner.fish ~/.config/fish/completions/

---

## Usage

Run `helm-cleaner` to uninstall a Helm release in a specific namespace:

```bash
helm-cleaner uninstall --namespace my-namespace

### Uninstall all releases and delete namespace without prompt

```bash
helm-cleaner uninstall --namespace my-namespace --delete-namespace --force

## Flags / Options

| Flag | Description |
|------|-------------|
| `--namespace, -n` | Kubernetes namespace to target |
| `--release, -r` | Helm release to uninstall (optional) |
| `--delete-namespace` | Delete the namespace after uninstalling releases |
| `--force` | Skip confirmation prompts |
| `--helm-bin` | Specify a custom Helm binary (default: `helm`) |

MIT License © Christian Bodelsson
