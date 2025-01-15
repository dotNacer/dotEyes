# dotEyes 👓

Screen Recording App with a editing interface, using Tauri 2, Svelte 5 and shadcn-svelte.

Forked from [alysonhower/tauri2-svelte5-shadcn](https://github.com/alysonhower/tauri2-svelte5-shadcn) (thanks to him for the boilerplate)

## Requirements

In order to run this boilerplate, you need to install Bun and Rust. If you are on Windows I also recommend installing MSVC before the other dependencies (make sure to check the "Desktop development with C++" workload).

Some useful links:

-   https://bun.sh/docs/installation
-   https://www.rust-lang.org/tools/install
-   https://visualstudio.microsoft.com/vs/community/

## Setup

```
git clone https://github.com/dotNacer/dotEyes.git
cd dotEyes
bun i
```

## Useful commands

### Start dev server

```
bun run tauri dev
```

### Build executable

```
bun run tauri build
```

## Other links

### Svelte 5

https://svelte.dev/docs

### Tauri 2

https://tauri.app/start/

### shadcn-svelte

https://next.shadcn-svelte.com/

## Planned stack

| Composant        | Technologie/Librairie         |
| ---------------- | ----------------------------- |
| Base Desktop App | Tauri + Rust                  |
| Frontend         | Svelte + TailwindCSS + ShadCN |
| Backend Vidéo    | ffmpeg, xlib                  |
| Effets Vidéo     | ffmpeg, p5.js (optionnel)     |
| MnK              | robotjs, xlib                 |
| Realtime         | WebSocket / IPC (Tauri)       |

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
