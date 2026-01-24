# easydocker

A continuous integration tool designed for easy integration with docker build systems

![Go](https://img.shields.io/badge/go-%2300ADD8.svg?style=for-the-badge&logo=go&logoColor=white)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)
![Docker](https://img.shields.io/badge/license-MIT-%230db7ed.svg?style=for-the-badge&logo=mit&logoColor=white)
![Linux](https://img.shields.io/badge/linux-Ubuntu.svg?style=for-the-badge&logo=linux&logoColor=white)

## How it works?
Will scanning all directory in workspace, if found docker-compose.yml will be listed, then after listed up will load informations about containers, images, and analytics.

## Features

- 📦 **Container Management** - View and manage Docker Compose projects
- 🐳 **Image Management** - List and delete Docker images
- 📊 **Real-time Analytics** - Monitor CPU, memory, and network usage of running containers

## Installation

```bash
# Building
make all

# install, path /usr/local/bin
make install # need root
```

## Usage

```bash
# without installing with command $(make install)
make run
# or
easydocker
```

## Independent build
```bash
# build easydocker runner
make build-go

# build easydocker binary
make build-rust
```

## Keybindings

| Key | Action |
|-----|--------|
| `q` | Quit |
| `r` | Refresh current tab |
| `Tab` / `Shift+Tab` | Switch between tabs |
| `↑` / `↓` | Navigate items |
| `Enter` | Open menu / Execute action |
| `Esc` | Close menu |
| `←` / `→` | Scroll logs |

## Tabs

1. **Containers** - Shows docker-compose.yml projects found in workspace
2. **Images** - Lists all Docker images
3. **Analytics** - Real-time container resource monitoring

## License

MIT License - Fitrian Musya 2026
