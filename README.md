# CS2 Tapping

> [!WARNING]
> **Use at Your Own Risk**
>
> This tool is provided strictly for **demonstration purposes** — to showcase what correct counter-strafing looks like in CS2. It is not intended for use in competitive play.
>
> CS2 may detect this application as **input automation**. Using it on official servers could result in being **kicked from a match or permanently banned**.
>
> By using this application you confirm that you have read and understood this README and that you accept **full responsibility** for any consequences that may arise.

A lightweight counter-strafe helper for CS2. Automatically fires a brief opposite-direction key release to stop movement momentum the instant you let go of a strafe key — improving accuracy without replacing muscle memory.

## Features

- Global keyboard hook with configurable key bindings (A/D and optionally W/S)
- Randomised strafe delay (configurable min/max range) to avoid detectable patterns
- System tray icon (red = active, grey = inactive) with toggle and show/hide controls
- Single-instance enforcement
- Autostart with Windows
- Starts minimized to tray (optional)
- Settings persisted to `%LOCALAPPDATA%/cs2tapping/settings.json`

## Stack

- **Backend** — Rust + [Tauri 2](https://v2.tauri.app/)
- **Frontend** — [SvelteKit](https://kit.svelte.dev/) + TypeScript
- **Keyboard hook** — [rdev](https://crates.io/crates/rdev)

## Requirements

- Windows 10 / 11 (x64)

## Development

```bash
pnpm install
pnpm tauri:dev
```

## Build

```bash
pnpm tauri build
```

## License

MIT — see [LICENSE](LICENSE)
