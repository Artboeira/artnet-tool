---
stepsCompleted: [1, 2, 3, 4]
inputDocuments: []
session_topic: 'New software product — concept definition from scratch'
session_goals: 'Define core functionality; generate unique/differentiating feature ideas'
selected_approach: 'ai-recommended'
techniques_used: ['First Principles Thinking', 'What If Scenarios']
ideas_generated: 7
session_active: false
workflow_completed: true
context_file: ''
---

# Brainstorming Session Results

**Facilitator:** NODATA
**Date:** 2026-03-06

---

## Session Overview

**Topic:** New software product — concept definition from scratch
**Goals:** Define what the software should do + generate ideas for unique features

**Session Setup:** Starting from zero. No existing concept — full open exploration.
Primary focus: (1) core functionality definition, (2) unique feature ideation.

---

## Technique Selection

**Approach:** AI-Recommended Techniques
**Analysis Context:** New software product from scratch, with focus on defining core functionality and unique features

**Techniques Used:**
- **First Principles Thinking** — Strip assumptions, find what problem truly needs solving
- **What If Scenarios** — Explode possibilities around real problems; where unique features are born

---

## All Ideas Generated

### Core Engine — Record & Play

**[Core #1]: The DMX Liberator**
_Concept:_ Standalone ArtNet recorder + player. Captures time-based DMX animations AND static snapshots from any source software (Resolume, TouchDesigner, MadMapper, etc.), then plays them back independently — no source software needed. Passive network listener, non-intrusive.
_Novelty:_ Source-agnostic. Doesn't interrupt existing workflow. Just captures the signal.

**[Core #2]: Dual Playback Engine**
_Concept:_ Two coexisting playback modes — a Cue List (trigger-based, console-style) and a Timeline (time-based, DAW-style). Scenes live in both worlds simultaneously.
_Novelty:_ Most DMX players force one paradigm. This gives both.

**[UI #2]: The Playback Lab**
_Concept:_ Dedicated tab for recording management and playback editing. Each recording is manipulable: speed (0.1x–10x), direction (forward/reverse), playback mode (loop, ping-pong, random frame-jump). Non-destructive — original stays intact.
_Novelty:_ Treats DMX recordings like audio clips in a DAW — with creative playback manipulation, not just play/stop.

---

### Monitoring & Editing

**[UI #1]: The Differential Monitor**
_Concept:_ Dedicated tab showing all ArtNet universes and DMX channels in real-time, highlighting only what's changing. Active channels glow, static channels fade, zeroed channels are invisible.
_Novelty:_ Shows the life of the signal — not a wall of 512 numbers, but a living differential view of movement and change.

**[UI #3]: Channel Sculptor** *(Post-v1)*
_Concept:_ Visual channel editor — click into any recorded scene and edit individual channel curves over time. Draw, smooth, offset, scale, or mute specific channels without re-recording.
_Novelty:_ Post-capture editing of DMX data like editing MIDI notes in a DAW.

---

### Triggering & Control

**[Core #3]: Multi-Trigger System**
_Concept:_ Every scene/cue can be fired via keyboard, MIDI, web remote (smartphone on local network), or sequencer — all configurable per scene.
_Novelty:_ No dongle, no extra hardware — your phone becomes a professional trigger pad.

**[UI #5]: Smart Web Remote**
_Concept:_ Local-network smartphone controller showing: live intensity feedback, master intensity fader per universe, scene trigger pads, mini cue list with current position, and a blackout panic button. Works in any browser, no app install.
_Novelty:_ A LD's monitor + controller in their pocket during a show, without touching the main machine.

**[Feature #6]: Show Mode**
_Concept:_ Locked fullscreen performance state — only essential controls visible, accidental clicks blocked, high-contrast UI optimized for dark stage environments.
_Novelty:_ Built for the real context of live performance, not just the studio.

---

### Creative Power

**[UI #4]: Scene Layering Engine** *(Post-v1)*
_Concept:_ Stack multiple recorded scenes as layers, each with opacity/intensity, blend mode (HTP, LTP, additive, subtract), and per-universe or per-channel masking. Layers can run at different speeds or directions simultaneously.
_Novelty:_ Brings the Photoshop/Resolume layer paradigm into a pure DMX player — no other standalone DMX tool does this.

---

### Hardware & Portability

**[Feature #7]: Portable DMX Node** *(Post-v1)*
_Concept:_ The software runs headless on a Raspberry Pi or mini PC (NUC), turning it into a standalone plug-and-play ArtNet playback device. Controlled entirely via web remote. Powers on and auto-loads last show state. No monitor, no laptop.
_Novelty:_ Transforms software into a hardware product — a dedicated, affordable DMX scene player installable in a venue, rigged to a truss, or built into a touring case.

---

## Idea Organization and Prioritization

### Thematic Clusters

| Theme | Ideas |
|---|---|
| Core Engine | DMX Liberator, Dual Playback Engine, Playback Lab |
| Monitoring & Editing | Differential Monitor, Channel Sculptor |
| Triggering & Control | Multi-Trigger System, Smart Web Remote, Show Mode |
| Creative Power | Scene Layering Engine |
| Hardware & Portability | Portable DMX Node |

### V1 Build Priority

| Priority | Feature | Reason |
|---|---|---|
| 🔴 Must | ArtNet recorder (passive sniffer) | Core value |
| 🔴 Must | Static snapshot capture | Simplest scene type |
| 🔴 Must | Cue List + keyboard/MIDI trigger | Makes it usable live |
| 🔴 Must | Differential Monitor tab | See what you're capturing |
| 🟡 Should | Timeline view | Sequencing capability |
| 🟡 Should | Web Remote (basic) | Trigger from phone |
| 🟡 Should | Playback controls (speed, reverse, loop modes) | Creative playback |
| 🟢 Post-v1 | Scene Layering Engine | Powerful but complex |
| 🟢 Post-v1 | Show Mode | Polish |
| 🟢 Post-v1 | Channel Sculptor | Deep editing |
| 🟢 Post-v1 | Portable DMX Node (RPi/NUC) | Hardware product |

### Architecture Notes (Inform from Day 1)
- Cross-platform build target: Windows + Linux ARM (Raspberry Pi)
- Headless mode: full functionality without display
- Auto-start on boot: loads last project automatically
- Low resource footprint: must stay lean for embedded hardware

---

## Session Summary

**Product Name (working):** ArtNet Tool / DMX Scene Player

**The One-Line Pitch:**
> A source-agnostic ArtNet recorder and playback engine — capture DMX animations from any software, play them back independently, trigger them live from keyboard, MIDI, or your phone.

**Key Creative Breakthroughs:**
- The "passive sniffer" model removes all friction — no proxy, no interruption, just capture
- Dual Cue List + Timeline gives both console and DAW paradigms in one tool
- The Portable DMX Node concept turns software into a hardware+software product
- Differential Monitor reframes how a LD reads DMX — by change, not raw values

**What Makes This Different:**
No existing standalone tool combines source-agnostic ArtNet recording, creative playback manipulation (speed/reverse/ping-pong), smartphone web remote, and portable hardware deployment in a single focused application.

**Immediate Next Steps:**
1. Define tech stack (cross-platform: Electron/Tauri + Node/Rust? Python?)
2. Prototype the ArtNet passive sniffer and basic scene capture
3. Build the Differential Monitor as first UI milestone
4. Design the scene data format (file structure for saved recordings)
