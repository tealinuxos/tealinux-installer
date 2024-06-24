# Tealinux Installer

Tauri & SvelteKit

## Prequisites
- [`rust`](https://www.rust-lang.org/)
- [`bun`](https://bun.sh)
- [`tauri`](https://tauri.app/v1/guides/getting-started/prerequisites/)

## Linux Dependency
- lshw
- dmidecode
- rsync
- fuse2
- gcc-libs

## Running

Install all dependencies:
```bash
bun install
```
Running the project:

```bash
bunx tauri dev

# with sudo privileges
sudo -E bunx tauri dev
```

## Building

To build and bundle into an executable:

```bash
bunx tauri build

# if encountering error running image.sh
NO_STRIP=true bunx tauri build
```
