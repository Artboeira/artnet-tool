# ARTNET TOOL

Cross-platform desktop application for passive ArtNet/sACN DMX recording and playback. Capture live lighting signals from any source on your network, save them as scenes, and replay them autonomously — no original hardware or software required.

## What it does

- **Passive capture** — Listens on your network interface without interrupting the source. Record ArtNet/sACN traffic from any lighting console, visualizer, or software.
- **Scene library & cue pad** — Save recordings as named scenes. Trigger them manually, via keyboard shortcut, MIDI note/CC, or on a time-based schedule.
- **Frame-accurate playback** — Replays scenes with ±1ms timing precision. Loop, one-shot, ping-pong, or reverse modes. Real-time speed control.
- **DMX monitor** — Live view of all active universes and 512 channels per universe, during both capture and playback.
- **Project files** — Save/load/export/import portable project files. Cross-platform compatible.
- **Auto-start & crash recovery** — Runs as an OS service on boot. Loads the last project and begins playback automatically. Recovers from crashes within 60 seconds.
- **Fully offline** — No accounts, no telemetry, no cloud. Works on air-gapped venue networks.

## Target hardware

Designed to run on Raspberry Pi 4 (ARM64) and equivalent mini PCs. Idle RAM < 50MB, CPU < 15% during active playback.

## Tech stack

| Layer | Technology |
|---|---|
| Desktop framework | Tauri 2 |
| Backend | Rust (std::thread + spin_sleep for playback, Tokio async for everything else) |
| Frontend | React 19 + TypeScript + Vite 6 |
| State management | Zustand |
| UI components | shadcn/ui + Tailwind CSS v3 (dark theme) |
| Packet capture | pcap / Npcap (Story 2.1+) |
| MIDI | midir (Story 4.2+) |

## Platform support

| Platform | Architecture | Status |
|---|---|---|
| Windows 10+ | x64 | Primary |
| macOS 12+ | Intel + Apple Silicon | Required |
| Linux (Debian/Ubuntu) | ARM64 (RPi 4+) | Required |

## Project structure

```
ARTNET-TOOL/
├── artnet-tool/          ← Tauri app source code
│   ├── src/              ← React + TypeScript frontend
│   ├── src-tauri/        ← Rust backend
│   └── package.json
├── _bmad-output/
│   ├── planning-artifacts/   ← PRD, Architecture, UX Design, Epics
│   └── implementation-artifacts/  ← Story files, sprint status
└── docs/
```

## Development setup

**Prerequisites:** [Rust](https://rustup.rs) · Node.js 18+ · npm

```bash
cd artnet-tool
npm install
npm run tauri dev
```

Windows users: Npcap is required for packet capture (installed automatically by the release installer). Not needed for frontend-only development.

## Implementation status

Progress tracked in [`_bmad-output/implementation-artifacts/sprint-status.yaml`](_bmad-output/implementation-artifacts/sprint-status.yaml).

| Epic | Description | Status |
|---|---|---|
| 1 | Foundation & Application Shell | in-progress |
| 2 | Signal Capture & Monitoring | backlog |
| 3 | Scene Library & Playback Engine | backlog |
| 4 | Triggering & Scheduling | backlog |
| 5 | Project Management | backlog |
| 6 | System Integration & Distribution | backlog |
| 7 | Observability & Reliability Polish | backlog |

## License

TBD
