# Tealinux Installer

Tauri & SvelteKit

## Prequisites
- [`rust`](https://www.rust-lang.org/)
- [`bun`](https://bun.sh)
- [`tauri`](https://v2.tauri.app/start/prerequisites/)

## Linux Dependency
- lshw
- dmidecode
- rsync
- fuse2
- gcc-libs
- webkit2gtk-4.1
- libappindicator-gtk3
- appmenu-gtk-module

To install the required dependencies on a Linux system using `pacman`, run the following command:
```bash
sudo pacman -Sy \
  webkit2gtk-4.1 \
  base-devel \
  curl \
  wget \
  file \
  openssl \
  appmenu-gtk-module \
  libappindicator-gtk3 \
  librsvg \ 
  lshw \ 
  dmidecode \ 
  rsync \ 
  fuse2 \ 
  gcc-libs \ 
```

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

## Contributing
To contribute, here's list of [`TODO`](https://github.com/tealinuxos/tealinux-installer/blob/master/TODO.md). Thank you for considering it!
