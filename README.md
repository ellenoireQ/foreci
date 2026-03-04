# easydocker

A continuous integration tool designed for easy integration with docker build systems

![Go](https://img.shields.io/badge/go-%2300ADD8.svg?style=for-the-badge&logo=go&logoColor=white)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)
![Docker](https://img.shields.io/badge/license-MIT-%230db7ed.svg?style=for-the-badge&logo=mit&logoColor=white)
![Linux](https://img.shields.io/badge/linux-Ubuntu.svg?style=for-the-badge&logo=linux&logoColor=white)
![macOS](https://img.shields.io/badge/macOS-10.15%2B-black.svg?style=for-the-badge&logo=apple&logoColor=white)
![Windows](https://img.shields.io/badge/windows-10%2B-%230078D6.svg?style=for-the-badge&logo=windows&logoColor=white)

## How it works?
Will scanning all directory in workspace, if found docker-compose.yml will be listed, then after listed up will load informations about containers, images, and analytics.

## Features

- 📦 **Container Management** - View and manage Docker Compose projects
- 🐳 **Image Management** - List and delete Docker images
- 📊 **Real-time Analytics** - Monitor CPU, memory, and network usage of running containers

## Preview
![Preview](./docs/gif/IMG_8807.gif)

## Installation

### Linux / macOS

```bash
# Build
make all

# Install to /usr/local/bin (requires root)
make install
```

### Windows

**Prerequisites:** [Rust](https://rustup.rs), [Go](https://go.dev/dl/), [Docker Desktop](https://www.docker.com/products/docker-desktop/)

```powershell
# Build + install to %USERPROFILE%\.local\bin
.\install.ps1

# Or install to a custom directory
.\install.ps1 -InstallPath "C:\tools"

# Build only
.\install.ps1 -BuildOnly
```

Restart your terminal after install so the updated `PATH` takes effect.

Alternatively, if you have [make](https://gnuwin32.sourceforge.net/packages/make.htm) installed:

```powershell
make all     # build both binaries
make install # copy to %USERPROFILE%\.local\bin
```

## Usage

```bash
# Linux/macOS – run without installing
make run

# Any platform – run directly after install
easydocker
```

## Independent build
```bash
# Build easydocker runner
make build-go

# Build easydocker binary
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
