# Tealinux Installer

Tauri & SvelteKit

## Prequisites
- [`rust`](https://www.rust-lang.org/)
- [`bun`](https://bun.sh)
- [`tauri`](https://wwwrust-lang.org/)

## Running

Install all dependencies:
```bash
bun install
```
Run the project with:

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
