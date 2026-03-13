---
stepsCompleted: ['step-01-validate-prerequisites', 'step-02-design-epics', 'step-03-create-stories', 'step-04-final-validation']
inputDocuments:
  - '_bmad-output/planning-artifacts/prd.md'
  - '_bmad-output/planning-artifacts/architecture.md'
  - '_bmad-output/planning-artifacts/ux-design-specification.md'
---

# ARTNET-TOOL - Epic Breakdown

## Overview

This document provides the complete epic and story breakdown for ARTNET-TOOL, decomposing the requirements from the PRD, UX Design, and Architecture requirements into implementable stories.

## Requirements Inventory

### Functional Requirements

FR1: User can start and stop passive network recording that captures ArtNet/sACN traffic from any source on the local network
FR2: System captures ArtNet/sACN traffic without modifying, intercepting, or interrupting the source software or hardware
FR3: User can select which network interface the system listens on for capture
FR4: System captures time-based DMX animations with full temporal fidelity
FR5: System captures static DMX snapshots as single-frame scenes
FR6: System captures traffic from multiple ArtNet/sACN universes simultaneously in a single recording session
FR7: User can save a completed recording as a named scene in the scene library
FR8: User can rename, reorder, and delete scenes
FR9: User can configure playback mode per scene (loop, one-shot, ping-pong, reverse)
FR10: User can configure playback speed per scene as a multiplier
FR11: User can assign a keyboard shortcut trigger to a scene
FR12: User can assign a MIDI note or CC mapping as a trigger for a scene
FR13: System plays back a recorded scene as ArtNet/sACN output with frame-accurate timing
FR14: System plays back scenes completely independently of the original source software or hardware
FR15: User can select which network interface is used for ArtNet/sACN playback output
FR16: User can adjust a scene's playback speed in real-time during active playback
FR17: System plays scenes in the configured mode (loop, one-shot, ping-pong, reverse)
FR18: User can trigger a scene via its configured keyboard shortcut
FR19: User can trigger a scene via its configured MIDI input
FR20: User can trigger a scene manually from the scene library interface
FR21: System detects hot-plugged MIDI devices and makes them available without requiring a restart
FR22: User can configure a time-based schedule for a scene specifying active time windows and day recurrence
FR23: System automatically switches to the scheduled scene at the configured time without user interaction
FR24: User can view all active ArtNet/sACN universes and their DMX channel values in real-time
FR25: System visually differentiates channels that are actively changing, static, and zeroed
FR26: During recording, the monitor displays incoming captured signal from the network
FR27: During playback, the monitor displays the outgoing DMX signal being transmitted
FR28: User can create, save, and load projects that contain all scenes and configuration
FR29: User can export a project as a single portable file containing all scenes, schedules, trigger mappings, and settings
FR30: User can import a project file on a different machine and have it operate identically
FR31: User can configure which scene is active and which playback mode is used when a project loads
FR32: System validates project file integrity on load and reports specific errors if the file is corrupt or unreadable
FR33: Project files created on earlier versions of the application remain fully loadable on later versions
FR34: User can configure the application to launch automatically when the OS starts
FR35: When configured for auto-start, the system loads the last active project automatically on launch
FR36: When configured for auto-start, the system begins playback automatically on launch without user interaction
FR37: Installer automatically configures OS-level requirements (firewall rules, packet capture driver) needed for operation
FR38: System logs all significant application events, errors, and crashes to a persistent local log file
FR39: User can view the event and crash log from within the application
FR40: System automatically recovers from non-fatal crashes and resumes playback without requiring manual intervention
FR41: System displays a clear error with actionable guidance when the configured network interface is unavailable at startup
FR42: System notifies the user of available software updates when internet connectivity is present, without downloading or installing automatically

### NonFunctional Requirements

NFR1: ArtNet/sACN output during playback must be frame-accurate — DMX packet intervals must not deviate from the recorded timing by more than ±1ms under normal operating conditions
NFR2: Recorded scene data must preserve original packet timestamps with sufficient resolution to reproduce frame-accurate playback
NFR3: When auto-start is configured, the system must reach active playback state within 30 seconds of OS boot completing, without user interaction
NFR4: Total CPU usage must remain below 15% and RAM usage below 300MB on the minimum target hardware (Raspberry Pi 4, 4GB RAM) during active playback with no UI rendering
NFR5: Playback engine timing must not be affected by UI rendering, user interactions, or background tasks — the playback loop runs on a dedicated isolated thread/process
NFR6: Triggering a scene must result in DMX output beginning within 50ms of the trigger event
NFR7: The system must operate without degradation for a minimum of 30 consecutive days of unattended playback
NFR8: The system must automatically restart and resume playback within 60 seconds of a non-fatal crash, without requiring human intervention
NFR9: In the event of a non-fatal crash during playback, the DMX output gap must be less than 60 seconds before auto-recovery
NFR10: Following an unplanned machine shutdown, the system must automatically restart, reload the project, and resume playback on next boot without manual intervention
NFR11: The system must detect corrupt or incomplete project files at load time and report a specific, actionable error
NFR12: A venue operator with no prior training must be able to switch between scenes using configured keyboard shortcuts after a single explanation
NFR13: A venue tech unfamiliar with ARTNET-TOOL must be able to install the application and load a project file successfully by following the bundled setup guide
NFR14: All error messages must describe what went wrong and what the user should do next in plain language — no raw error codes or stack traces visible to end users
NFR15: Captured and replayed ArtNet packets must conform to the Art-Net 4 specification; Art-Net 3 sources must be captured and replayed correctly via backward compatibility
NFR16: Captured and replayed sACN packets must conform to the ANSI E1.31 specification for both unicast and multicast modes
NFR17: Core functionality must behave identically on Windows, macOS, and Linux ARM — no platform-exclusive feature degradation for MVP capabilities
NFR18: A project file created on Windows must load and operate correctly on macOS and Linux ARM without modification
NFR19: MIDI input handling must conform to the MIDI 1.0 specification for Note On/Off and Control Change messages across all supported platforms
NFR20: The application must not transmit any user data, project data, telemetry, or usage analytics to any remote server
NFR21: The application must not execute arbitrary code from project files — project file parsing must be sandboxed and validated against a known schema
NFR22: The application must not expose any open network ports or services beyond what is required for ArtNet/sACN operation on the configured interface

### Additional Requirements

**From Architecture:**
- STARTER TEMPLATE (Epic 1, Story 1): `npm create tauri-app@latest artnet-tool -- --template react-ts` — Tauri 2 + React 19 + TypeScript + Vite scaffold
- Install and configure `tauri-plugin-global-shortcut` for OS-level keyboard trigger capture (works when app is minimized/backgrounded)
- Configure Tokio async runtime for all subsystems except playback engine
- Playback engine runs on dedicated `std::thread` with `spin_sleep` for ±1ms timing (NFR1) — isolated from Tokio runtime
- Set up shadcn/ui + Tailwind CSS component library with dark theme
- Scaffold Zustand store slices: playback, capture, scenes, monitor, settings, errors
- All `invoke()` calls must be centralized in `src/lib/tauri.ts` — never in components
- All Tauri commands return `Result<T, String>` with plain-language `.to_user_message()` errors
- Tauri event names use `kebab-case` pattern: `{subsystem}-{event}`
- JSON project file fields use `snake_case` via `#[serde(rename_all = "snake_case")]`
- Feature-based frontend structure: `src/features/cue-pad/`, `src/features/monitor/`, `src/features/settings/`
- Rust backend: `commands/` (thin wrappers) + `subsystems/` (business logic) + `models/` (shared types)
- OS service layer for auto-start AND crash recovery: systemd (Linux), LaunchAgent (macOS), Windows Service via `windows-service` crate
- Windows installer: Npcap silent installer bundled + Windows Defender Firewall rules via NSIS post-install script (UAC required once)
- Daily log rotation: `tracing-appender` rolling file, max 30 files retained
- Project file: `schema_version: u32` field + migration chain for forward compatibility
- CI/CD: Tauri GitHub Action (`tauri-apps/tauri-action`) matrix build on git tag push — Win/macOS/Linux; macOS signing stubbed until Apple Developer account active
- `thiserror` for domain errors; `anyhow` for infrastructure errors; converted to `String` at IPC boundary

**From UX Design:**
- Three-tab main navigation: Cue Pad / Monitor / Settings — tab-based, no routing required
- Cue pad: grid layout of named scene cards with playback state indicators (active/inactive/loading)
- Capture bar: persistent top bar with record button, status indicator, network interface selector
- Differential monitor: universe panels with 512-channel grid; active channels prominent (color), static channels faded, zeroed channels invisible
- Speed slider: real-time speed control (FR16) visible on active scene card
- Error toast: non-blocking overlay for plain-language error messages (NFR14)
- Settings organized into sub-tabs: Network, MIDI, Schedule, Project, Boot, Log, Updates
- Dark professional theme throughout — optimized for use in dark venue environments

### FR Coverage Map

FR1: Epic 2 - Start/stop passive network recording
FR2: Epic 2 - Passive capture (non-intercepting)
FR3: Epic 2 - Network interface selector for capture
FR4: Epic 2 - Time-based DMX animation capture with temporal fidelity
FR5: Epic 2 - Static DMX snapshot capture
FR6: Epic 2 - Multi-universe simultaneous capture
FR7: Epic 3 - Save recording as named scene
FR8: Epic 3 - Rename, reorder, delete scenes
FR9: Epic 3 - Playback mode configuration per scene
FR10: Epic 3 - Playback speed multiplier per scene
FR11: Epic 4 - Assign keyboard shortcut trigger to scene
FR12: Epic 4 - Assign MIDI note/CC mapping to scene
FR13: Epic 3 - Frame-accurate ArtNet/sACN playback output
FR14: Epic 3 - Playback independent of original source
FR15: Epic 3 - Network interface selector for playback output
FR16: Epic 3 - Real-time speed adjustment during active playback
FR17: Epic 3 - Playback in configured mode (loop/one-shot/ping-pong/reverse)
FR18: Epic 4 - Trigger scene via keyboard shortcut
FR19: Epic 4 - Trigger scene via MIDI input
FR20: Epic 3 - Manual scene trigger from library interface
FR21: Epic 4 - Hot-plug MIDI device detection (no restart required)
FR22: Epic 4 - Time-based schedule configuration per scene
FR23: Epic 4 - Automatic scene switching at scheduled time
FR24: Epic 2 - Real-time DMX universe/channel monitor view
FR25: Epic 2 - Visual differentiation of active/static/zeroed channels
FR26: Epic 2 - Monitor displays incoming capture signal during recording
FR27: Epic 3 - Monitor displays outgoing DMX signal during playback
FR28: Epic 5 - Create, save, and load projects
FR29: Epic 5 - Export project as single portable file
FR30: Epic 5 - Import project file on different machine
FR31: Epic 5 - Configure default scene and playback mode on project load
FR32: Epic 5 - Validate project file integrity on load with specific errors
FR33: Epic 5 - Forward compatibility: older project files load on newer versions
FR34: Epic 6 - Configure application auto-start on OS boot
FR35: Epic 6 - Auto-load last active project on auto-start launch
FR36: Epic 6 - Auto-begin playback on auto-start launch
FR37: Epic 6 - Installer configures OS-level requirements (firewall, Npcap)
FR38: Epic 7 - Log significant events, errors, and crashes to persistent log file
FR39: Epic 7 - View event/crash log from within application
FR40: Epic 6 - Auto-recover from non-fatal crashes, resume playback
FR41: Epic 6 - Clear error with actionable guidance when network interface unavailable
FR42: Epic 7 - Notify user of available software updates (no auto-install)

## Epic List

### Epic 1: Foundation & Application Shell
Establish the complete project scaffold with all architectural patterns in place so all subsequent epics can be built on a consistent, tested foundation. No functional requirements are addressed here — this epic exists to create the greenfield starting point as specified in the architecture document.
**FRs covered:** None (greenfield scaffold — architectural foundation only)

### Epic 2: Signal Capture & Monitoring
Users can passively record ArtNet/sACN traffic from any source on their network, select which interface to listen on, and see all captured DMX universe data in real-time. After this epic, the application is a fully functional DMX signal recorder and live monitor.
**FRs covered:** FR1, FR2, FR3, FR4, FR5, FR6, FR24, FR25, FR26

### Epic 3: Scene Library & Playback Engine
Users can save recordings as named scenes, configure playback behavior per scene, and play back DMX output with frame-accurate timing — completely independent of the original source software. After this epic, the application is a fully functional DMX playback engine with a manageable scene library.
**FRs covered:** FR7, FR8, FR9, FR10, FR13, FR14, FR15, FR16, FR17, FR20, FR27

### Epic 4: Triggering & Scheduling
Users can assign keyboard shortcuts and MIDI mappings to trigger scenes, configure time-based schedules for automated scene switching, and have MIDI devices detected hot-plug without restart. After this epic, the application supports full unattended and remote-trigger operation.
**FRs covered:** FR11, FR12, FR18, FR19, FR21, FR22, FR23

### Epic 5: Project Management
Users can create, save, load, export, and import projects as portable files — including all scenes, schedules, trigger mappings, and settings — with integrity validation and forward-compatibility across application versions. After this epic, the application supports complete project portability across machines.
**FRs covered:** FR28, FR29, FR30, FR31, FR32, FR33

### Epic 6: System Integration & Distribution
Users can configure the application to auto-start and auto-play on OS boot, the system auto-recovers from crashes, the installer handles all OS-level prerequisites, and clear errors are shown when network interfaces are unavailable. After this epic, the application is deployable as a fully unattended venue automation system.
**FRs covered:** FR34, FR35, FR36, FR37, FR40, FR41

### Epic 7: Observability & Reliability Polish
Users can view a persistent event/crash log from within the application, and the system notifies users of available software updates without auto-installing. After this epic, the application has complete operational observability for venue technicians and self-managed update awareness.
**FRs covered:** FR38, FR39, FR42

---

## Epic 1: Foundation & Application Shell

Establish the complete project scaffold with all architectural patterns in place so all subsequent epics can be built on a consistent, tested foundation. No functional requirements are addressed here — this epic exists to create the greenfield starting point as specified in the architecture document.

### Story 1.1: Project Scaffold & Directory Structure

As a developer,
I want the project scaffolded from the Tauri 2 + React 19 + TypeScript starter template with the full directory structure defined in the architecture,
So that all subsequent development starts from a consistent, architecture-compliant foundation.

**Acceptance Criteria:**

**Given** the developer runs `npm create tauri-app@latest artnet-tool -- --template react-ts`
**When** the scaffold command completes
**Then** the project runs with `npm run tauri dev` without errors

**Given** the scaffolded project
**When** the directory structure is reviewed
**Then** `src/features/cue-pad/`, `src/features/monitor/`, `src/features/settings/` directories exist
**And** `src/stores/` and `src/lib/` directories exist
**And** `src-tauri/src/commands/`, `src-tauri/src/subsystems/`, `src-tauri/src/models/` directories exist with placeholder modules

**Given** the project structure
**When** `src/lib/tauri.ts` is reviewed
**Then** it exports typed wrapper functions for all Tauri `invoke()` calls
**And** no component file contains a direct `invoke()` call

### Story 1.2: UI Shell — Dark Theme, shadcn/ui & 3-Tab Navigation

As a developer,
I want shadcn/ui and Tailwind CSS installed with a dark theme and the 3-tab navigation shell (Cue Pad / Monitor / Settings),
So that all UI feature development can build on a consistent, themed component foundation.

**Acceptance Criteria:**

**Given** the application launches
**When** the main window renders
**Then** the UI displays in a dark professional theme with no light-mode flash
**And** the Tailwind CSS dark theme is applied globally

**Given** the main window is open
**When** the user views the navigation
**Then** three tabs are visible: "Cue Pad", "Monitor", and "Settings"
**And** clicking each tab switches the active view without page routing

**Given** the shadcn/ui library is installed
**When** any shadcn/ui component (e.g., Button, Tabs) is rendered
**Then** it displays correctly within the dark theme without style overrides needed

**Given** the application window
**When** it is resized to typical desktop dimensions
**Then** the tab layout remains usable and does not overflow or break

### Story 1.3: Zustand State Management Scaffold

As a developer,
I want all 6 Zustand store slices (playback, capture, scenes, monitor, settings, errors) defined with typed initial state and action stubs,
So that feature development can import and extend state without defining slice shapes from scratch.

**Acceptance Criteria:**

**Given** the store scaffold is implemented
**When** a developer imports any slice (e.g., `usePlaybackStore`, `useCaptureStore`)
**Then** the import resolves without TypeScript errors
**And** the initial state shape is defined according to the architecture slice structure

**Given** the `errors` slice
**When** an error action is dispatched
**Then** the error state is updated and the value is accessible for the error toast system to consume

**Given** all 6 store slice files
**When** TypeScript compilation runs
**Then** there are zero type errors related to store definitions

### Story 1.4: Rust Backend Structure & IPC Foundation

As a developer,
I want the Rust backend organized into `commands/`, `subsystems/`, and `models/` with the `thiserror`/`anyhow` error pattern and a `to_user_message()` trait converting all errors to plain strings at the IPC boundary,
So that all Rust features follow a consistent error handling and IPC contract.

**Acceptance Criteria:**

**Given** the Rust backend
**When** the directory layout is reviewed
**Then** `commands/`, `subsystems/`, and `models/` directories exist with placeholder modules registered in `lib.rs`

**Given** a Tauri command handler that returns an `Err`
**When** the frontend calls it via `invoke()` in `src/lib/tauri.ts`
**Then** the error arrives as a plain-language `String`
**And** no raw Rust error codes, type names, or stack trace fragments are visible

**Given** a domain error type using `thiserror`
**When** the `to_user_message()` trait is implemented on it
**Then** it returns a human-readable string describing the problem and next action (NFR14)

**Given** `src/lib/tauri.ts`
**When** it is reviewed
**Then** it is the only file in the frontend codebase that calls `invoke()`
**And** every exported function has explicit TypeScript parameter and return types matching the corresponding Rust command signature

### Story 1.5: Playback Engine Thread Isolation Scaffold

As a developer,
I want the playback engine scaffold running on a dedicated `std::thread` with `spin_sleep`, isolated from the Tokio async runtime,
So that the timing-critical playback loop is never affected by async scheduling or UI rendering (NFR1, NFR5).

**Acceptance Criteria:**

**Given** the application starts
**When** Tauri app state is initialized
**Then** a dedicated `std::thread` is spawned for the playback engine
**And** that thread is not managed by or blocked by the Tokio runtime

**Given** the playback thread scaffold
**When** the thread loop executes
**Then** it uses `spin_sleep` for sleep/wait operations
**And** a `std::sync::mpsc` channel is established to receive commands from Tokio subsystems

**Given** the Tokio runtime configuration
**When** the application is running
**Then** capture, MIDI, scheduler, monitor, and project subsystems all run within Tokio
**And** the playback thread runs independently on its own OS thread

**Given** the application receives a shutdown signal
**When** the shutdown sequence runs
**Then** the playback thread receives a stop command via the channel and exits cleanly without panicking

### Story 1.6: CI/CD Pipeline

As a developer,
I want a GitHub Actions workflow that builds platform-native installers for Windows, macOS, and Linux on every git tag push,
So that releases are produced automatically without manual per-platform build steps.

**Acceptance Criteria:**

**Given** a git tag is pushed to the repository
**When** the GitHub Actions workflow triggers
**Then** it builds installers for Windows (.msi/.exe), macOS (.dmg), and Linux (.AppImage/.deb)
**And** artifacts are uploaded as GitHub release assets

**Given** macOS signing credentials are not configured
**When** the macOS build runs
**Then** the build completes successfully and produces an unsigned .dmg artifact
**And** a clear comment in the workflow file notes that notarization is stubbed pending Apple Developer account

**Given** the workflow file
**When** reviewed
**Then** it uses `tauri-apps/tauri-action` with a matrix strategy across all three platforms
**And** the trigger is `on: push: tags: ['v*']`

---

## Epic 2: Signal Capture & Monitoring

Users can passively record ArtNet/sACN traffic from any source on their network, select which interface to listen on, and see all captured DMX universe data in real-time. After this epic, the application is a fully functional DMX signal recorder and live monitor.

### Story 2.1: Network Interface Discovery & Selection

As a lighting operator,
I want to select which network interface the system listens on for ArtNet/sACN capture,
So that I can target the correct network when the machine has multiple interfaces (e.g., wired + wireless).

**Acceptance Criteria:**

**Given** the application is running
**When** the user opens the capture bar's interface selector
**Then** a list of all available network interfaces on the machine is displayed with their names and IP addresses

**Given** the interface list is displayed
**When** the user selects an interface
**Then** it is saved as the active capture interface
**And** subsequent recording sessions use that interface

**Given** no interface has been configured
**When** the application starts
**Then** it prompts the user to select an interface before capture can begin

**Given** the previously selected interface is no longer available at startup
**When** the application loads
**Then** it displays a clear error with actionable guidance (NFR14, FR41)
**And** the interface selector is shown so the user can choose a different one

### Story 2.2: Passive ArtNet/sACN Capture Engine — Start, Stop & Record

As a lighting operator,
I want to start and stop passive network recording that captures ArtNet/sACN traffic from any source on my network, preserving timing and all active universes,
So that I can record DMX animations and snapshots without touching the source software or hardware.

**Acceptance Criteria:**

**Given** a network interface is selected
**When** the user clicks Record
**Then** the system begins passively capturing ArtNet and sACN packets on the configured interface
**And** the source software or hardware continues operating without interruption or modification (FR2)

**Given** recording is active
**When** the user clicks Stop
**Then** the capture ends and the recorded data (all packets with their original timestamps) is held in memory ready for saving

**Given** an active ArtNet/sACN source transmitting animations
**When** a recording session captures that traffic
**Then** each captured packet preserves its original timestamp with sufficient resolution for ±1ms frame-accurate playback (NFR2)

**Given** multiple ArtNet or sACN universes are active simultaneously on the network
**When** a recording session is running
**Then** all active universes are captured concurrently within the same session (FR6)

**Given** a source transmitting a static DMX state (one frame, no change)
**When** the recording is stopped and saved
**Then** the result is stored as a single-frame snapshot scene (FR5)

**Given** ArtNet traffic conforming to Art-Net 3 or Art-Net 4
**When** captured
**Then** packets are parsed and stored correctly per the Art-Net 4 specification with Art-Net 3 backward compatibility (NFR15)

**Given** sACN traffic (unicast or multicast)
**When** captured
**Then** packets are parsed and stored correctly per the ANSI E1.31 specification (NFR16)

**Given** recording is active on the minimum target hardware (Raspberry Pi 4)
**When** CPU and RAM usage are measured
**Then** total CPU remains below 15% and RAM below 300MB (NFR4)

### Story 2.3: Real-Time DMX Monitor — Universe Panels & Channel State Display

As a lighting operator,
I want to see all active ArtNet/sACN universes and their 512 DMX channel values in real-time, with visual differentiation between actively changing, static, and zeroed channels,
So that I can verify what signal is present on the network before and during recording.

**Acceptance Criteria:**

**Given** the Monitor tab is open and a recording is active
**When** ArtNet/sACN traffic is being captured
**Then** the monitor displays the incoming signal from the network in real-time (FR26)
**And** each active universe is shown as a separate panel

**Given** a universe panel is displayed
**When** channel values are rendered
**Then** channels actively changing value are shown prominently with a distinct color
**And** channels that are static (non-zero, not changing) are shown in a faded style
**And** channels that are zeroed (value = 0) are visually suppressed or hidden (FR25)

**Given** the monitor is displaying a universe
**When** a channel value changes
**Then** the display updates within one UI render cycle (no perceptible lag relative to network traffic)

**Given** the Monitor tab is open with no active capture or playback
**When** no ArtNet/sACN traffic is present
**Then** the monitor shows an empty or idle state without errors

**Given** a universe panel showing 512 channels
**When** the panel is rendered
**Then** all 512 channel slots are accessible (scrollable if needed) and their current values are readable

---

## Epic 3: Scene Library & Playback Engine

Users can save recordings as named scenes, configure playback behavior per scene, and play back DMX output with frame-accurate timing — completely independent of the original source software. After this epic, the application is a fully functional DMX playback engine with a manageable scene library.

### Story 3.1: Save Recording as Named Scene & Scene Library Management

As a lighting operator,
I want to save a completed recording as a named scene and manage my scene library (rename, reorder, delete),
So that I can build and maintain a collection of DMX cues ready for playback.

**Acceptance Criteria:**

**Given** a recording has been captured and stopped
**When** the user clicks "Save as Scene"
**Then** a name input appears and the user can provide a scene name
**And** the scene is added to the scene library with its recorded data

**Given** a scene exists in the library
**When** the user renames it
**Then** the new name is reflected immediately in the scene library and cue pad

**Given** multiple scenes in the library
**When** the user reorders them (drag or up/down controls)
**Then** the scenes display in the new order in the cue pad grid

**Given** a scene in the library
**When** the user deletes it
**Then** it is removed from the library
**And** if the scene was not active during playback, no DMX output is interrupted

**Given** the scene library
**When** the cue pad tab is viewed
**Then** each scene is shown as a named card in a grid layout with an inactive/idle state indicator

### Story 3.2: Frame-Accurate ArtNet/sACN Playback Engine

As a lighting operator,
I want a recorded scene to play back as ArtNet/sACN output with the same timing it was captured, completely independently of the original source software or hardware,
So that I can use the application as a standalone DMX playback device at the venue.

**Acceptance Criteria:**

**Given** a saved scene and a configured output network interface
**When** playback is started
**Then** the system transmits ArtNet/sACN packets on the selected output interface (FR15)
**And** the original source software or hardware is not required to be running (FR14)

**Given** a scene with recorded packet timestamps
**When** the playback engine processes it
**Then** DMX packet intervals do not deviate from the recorded timing by more than ±1ms (NFR1)

**Given** the playback engine is running
**When** measured on the minimum target hardware (Raspberry Pi 4)
**Then** total CPU usage remains below 15% and RAM below 300MB (NFR4)

**Given** the playback engine is transmitting
**When** the UI is interacted with (tabs switched, settings opened)
**Then** playback timing is not affected (NFR5)

**Given** a scene with multiple universes
**When** played back
**Then** all universes are transmitted concurrently with their original relative timing

**Given** output packets transmitted during playback
**When** inspected
**Then** ArtNet packets conform to the Art-Net 4 specification (NFR15)
**And** sACN packets conform to ANSI E1.31 (NFR16)

### Story 3.3: Playback Modes & Speed Configuration

As a lighting operator,
I want to configure each scene's playback mode (loop, one-shot, ping-pong, reverse) and a speed multiplier,
So that I can adapt recorded cues to different timing or looping needs without re-recording.

**Acceptance Criteria:**

**Given** a scene in the library
**When** the user opens its configuration
**Then** playback mode options are available: loop, one-shot, ping-pong, reverse

**Given** a scene configured as loop
**When** it reaches the end of the recording
**Then** it seamlessly restarts from the beginning without a timing gap

**Given** a scene configured as one-shot
**When** it reaches the end
**Then** playback stops and output goes dark (all channels zero)

**Given** a scene configured as ping-pong
**When** it reaches the end
**Then** it plays back in reverse until the beginning, then forward again, continuously

**Given** a scene configured as reverse
**When** played
**Then** it plays from the last frame to the first frame

**Given** a scene with a speed multiplier set (e.g., 0.5×, 2×)
**When** played back
**Then** the playback timing is scaled proportionally to the multiplier while maintaining frame accuracy

### Story 3.4: Cue Pad UI — Manual Scene Trigger & Real-Time Speed Control

As a lighting operator,
I want to trigger any scene manually from the cue pad and adjust its speed in real-time while it plays,
So that I can operate cues live during a show directly from the application interface.

**Acceptance Criteria:**

**Given** the Cue Pad tab is open
**When** the user clicks a scene card
**Then** playback of that scene begins within 50ms (NFR6)
**And** the scene card shows an active/playing state indicator

**Given** a scene is actively playing
**When** another scene card is clicked
**Then** the first scene stops and the new scene begins within 50ms

**Given** an actively playing scene card
**When** the speed slider is visible on the card
**Then** dragging the slider changes the playback speed in real-time without stopping or restarting playback (FR16)

**Given** playback is active
**When** the scene completes (one-shot mode)
**Then** the scene card returns to inactive state

### Story 3.5: Playback Monitor — Outgoing DMX Signal Display

As a lighting operator,
I want the monitor to display the DMX signal being transmitted during playback,
So that I can visually verify the correct output is being sent to the venue's lighting network.

**Acceptance Criteria:**

**Given** the Monitor tab is open and a scene is playing
**When** DMX packets are being transmitted
**Then** the monitor displays the outgoing values for each universe and channel in real-time (FR27)

**Given** the monitor is showing playback output
**When** channel values are rendered
**Then** the same active/static/zeroed visual differentiation from Epic 2 is applied to the playback output

**Given** playback is stopped
**When** the monitor is viewed
**Then** it returns to idle/empty state or shows the last known values faded

**Given** the monitor is rendering during playback
**When** measured
**Then** monitor rendering does not affect the playback engine's ±1ms timing (NFR5)

---

## Epic 4: Triggering & Scheduling

Users can assign keyboard shortcuts and MIDI mappings to trigger scenes, configure time-based schedules for automated scene switching, and have MIDI devices detected hot-plug without restart. After this epic, the application supports full unattended and remote-trigger operation.

### Story 4.1: Keyboard Shortcut Assignment & OS-Level Scene Trigger

As a lighting operator,
I want to assign a keyboard shortcut to each scene and trigger it from the keyboard — even when the app is minimized,
So that I can switch lighting cues hands-free during a live show without keeping the application in focus.

**Acceptance Criteria:**

**Given** a scene in the library
**When** the user opens its trigger settings
**Then** a keyboard shortcut can be recorded and assigned to that scene

**Given** a scene with an assigned keyboard shortcut
**When** the user presses that shortcut with the application in focus or minimized/backgrounded
**Then** the scene begins playing within 50ms of the keypress (NFR6)

**Given** two scenes assigned the same keyboard shortcut
**When** the assignment is attempted
**Then** the system warns the user of the conflict and does not allow duplicate assignments

**Given** a keyboard shortcut is assigned
**When** the application starts
**Then** the global shortcut is registered with the OS via `tauri-plugin-global-shortcut` and active immediately

**Given** a scene's shortcut is cleared by the user
**When** the application is running
**Then** the OS-level shortcut registration is removed and the key no longer triggers that scene

### Story 4.2: MIDI Device Management, Mapping & Hot-Plug Detection

As a lighting operator,
I want to assign a MIDI note or CC to each scene and trigger it from a MIDI controller — with new devices recognized automatically without restarting the app,
So that I can use physical MIDI controllers for hands-on show control.

**Acceptance Criteria:**

**Given** a scene in the library
**When** the user opens its MIDI trigger settings
**Then** they can assign a MIDI Note On (note + channel) or Control Change (CC number + channel) as the trigger

**Given** a MIDI controller connected and a scene with a MIDI mapping
**When** the mapped note or CC message is received
**Then** the scene begins playing within 50ms (NFR6)

**Given** MIDI messages received from any connected device
**When** processed
**Then** they conform to the MIDI 1.0 specification for Note On/Off and Control Change (NFR19)

**Given** a MIDI device that was not connected when the application started
**When** the device is plugged in
**Then** it appears in the MIDI device list without requiring an application restart (FR21)

**Given** a MIDI device that is unplugged while the application is running
**When** the device is removed
**Then** the application handles the disconnection gracefully without crashing
**And** the device's trigger mappings are retained for when it is reconnected

**Given** no MIDI devices are connected
**When** the MIDI settings are viewed
**Then** the UI shows an empty device list without errors

### Story 4.3: Time-Based Scene Scheduling & Auto-Switch

As a venue technician,
I want to configure a time-based schedule for each scene with active time windows and day recurrence, and have the system switch scenes automatically at the configured time,
So that lighting cues run on a timed program without requiring anyone to be present.

**Acceptance Criteria:**

**Given** a scene in the library
**When** the user opens its schedule settings
**Then** they can configure: start time, end time, and which days of the week the schedule is active

**Given** a scene with an active schedule
**When** the system clock reaches the configured start time on a scheduled day
**Then** the scene begins playing automatically without any user interaction (FR23)

**Given** a scene with a scheduled end time
**When** the system clock reaches the end time
**Then** playback stops automatically

**Given** multiple scenes with overlapping schedules
**When** a schedule conflict would occur
**Then** the system warns the user at configuration time

**Given** the application is running past midnight
**When** a scheduled scene's day recurrence includes the next day
**Then** the schedule correctly activates on the new day without requiring a restart

**Given** a scheduled auto-switch occurs
**When** measured
**Then** the scene begins playing within 50ms of the scheduled time (NFR6)

---

## Epic 5: Project Management

Users can create, save, load, export, and import projects as portable files — including all scenes, schedules, trigger mappings, and settings — with integrity validation and forward-compatibility across application versions. After this epic, the application supports complete project portability across machines.

### Story 5.1: Project Create, Save, Load & Startup Configuration

As a lighting operator,
I want to create, save, and load projects that contain all my scenes and configuration, and configure which scene and playback mode is active when the project opens,
So that I can organize my work into projects and have the correct cue ready when I open one.

**Acceptance Criteria:**

**Given** the application is running
**When** the user creates a new project
**Then** a fresh project with no scenes is created and becomes the active project

**Given** an active project with scenes and configuration
**When** the user saves the project
**Then** all scenes, trigger mappings, schedules, and settings are persisted to a JSON file on disk

**Given** a saved project file
**When** the user loads it
**Then** all scenes, trigger mappings, schedules, and settings are restored exactly as saved

**Given** a project's load configuration
**When** the user configures the startup scene and playback mode
**Then** those settings are saved with the project

**Given** a project with a configured startup scene and playback mode
**When** the project is loaded
**Then** the specified scene is made active and playback mode is set as configured (FR31)

**Given** the project file
**When** written to disk
**Then** it is JSON format with `snake_case` fields and a `schema_version` field
**And** it does not transmit any data to any remote server (NFR20)

### Story 5.2: Project Export & Cross-Platform Import

As a lighting operator,
I want to export my project as a single portable file and import it on a different machine — including a different operating system — and have it work identically,
So that I can deploy a project built on my workstation to a venue playback machine running a different OS.

**Acceptance Criteria:**

**Given** an active project
**When** the user exports it
**Then** a single self-contained file is produced containing all scenes, schedules, trigger mappings, and settings (FR29)

**Given** a project file exported on Windows
**When** imported on macOS or Linux ARM
**Then** all scenes, triggers, schedules, and settings are restored and the project operates identically (NFR18)

**Given** a project file exported on any supported platform
**When** imported on a different machine via the Import action
**Then** the import succeeds without requiring manual edits to the file (FR30)

**Given** the import process
**When** a project file is parsed
**Then** it is validated against the project schema before any data is loaded (NFR21)
**And** no code contained within the file is executed — only declared data is read

**Given** a successful import
**When** the project is active
**Then** it behaves identically to the exported original, including trigger mappings and schedule times

### Story 5.3: Project File Integrity Validation & Forward Compatibility

As a lighting operator,
I want the application to detect and clearly report corrupt project files at load time, and have older project files remain fully loadable on newer versions of the application,
So that I never lose a project silently and deployments survive software updates.

**Acceptance Criteria:**

**Given** a project file that is corrupt or structurally invalid
**When** the user attempts to load it
**Then** the load is rejected with a specific, plain-language error describing what is wrong and what the user should do (NFR11, NFR14, FR32)
**And** no partial state is applied

**Given** a project file with an incomplete or truncated JSON structure
**When** loaded
**Then** the integrity check catches it and reports the specific structural issue

**Given** a project file created with an older `schema_version`
**When** loaded on a newer version of the application
**Then** the migration chain upgrades the file's data model in memory (FR33)
**And** the project loads successfully without data loss

**Given** a project file with a `schema_version` higher than the running application supports
**When** loaded
**Then** the application reports a clear error indicating the file was created with a newer version and cannot be loaded

**Given** any project file being loaded
**When** parsed
**Then** `#[serde(deny_unknown_fields)]` ensures unexpected fields cause a validation error rather than silent data loss

---

## Epic 6: System Integration & Distribution

Users can configure the application to auto-start and auto-play on OS boot, the system auto-recovers from crashes, the installer handles all OS-level prerequisites, and clear errors are shown when network interfaces are unavailable. After this epic, the application is deployable as a fully unattended venue automation system.

### Story 6.1: OS Auto-Start, Auto-Load, Auto-Play & Startup Error Handling

As a venue technician,
I want to configure the application to launch automatically on OS boot, load the last active project, and begin playback — with clear errors if the network interface is unavailable,
So that the venue lighting runs automatically after power-on without anyone needing to be present.

**Acceptance Criteria:**

**Given** the user enables auto-start in Settings
**When** the OS boots
**Then** the application launches automatically without requiring user login interaction (FR34)
**And** the system reaches active playback state within 30 seconds of OS boot completing (NFR3)

**Given** auto-start is enabled and a project was last active
**When** the application launches on boot
**Then** it loads the last active project automatically (FR35)
**And** begins playback in the configured mode without any user interaction (FR36)

**Given** auto-start is enabled on Linux
**When** reviewed
**Then** the service is registered as a systemd unit

**Given** auto-start is enabled on macOS
**When** reviewed
**Then** the service is registered as a LaunchAgent plist

**Given** auto-start is enabled on Windows
**When** reviewed
**Then** the service is registered via the `windows-service` crate

**Given** the application launches (auto-start or manual) and the configured network interface is not available
**When** startup proceeds
**Then** a clear, plain-language error is displayed describing which interface is missing and what the user should do next (NFR14, FR41)
**And** the application does not crash — it waits for user action or interface availability

### Story 6.2: Crash Recovery — Auto-Restart & Playback Resumption

As a venue technician,
I want the system to automatically restart and resume playback after a non-fatal crash, and after an unplanned machine shutdown, without requiring anyone to intervene,
So that a crash or unexpected power loss doesn't leave the venue with no lighting for more than a minute.

**Acceptance Criteria:**

**Given** the application experiences a non-fatal crash during playback
**When** the crash occurs
**Then** the OS service layer detects the process exit and restarts the application (FR40)
**And** the application restarts and resumes playback within 60 seconds (NFR8)
**And** the DMX output gap is less than 60 seconds (NFR9)

**Given** the machine experiences an unplanned shutdown (power loss)
**When** the machine reboots
**Then** the auto-start service launches the application, loads the last project, and resumes playback automatically (NFR10)
**And** no manual intervention is required

**Given** the application has been running continuously
**When** 30 days have elapsed since last restart
**Then** playback continues without degradation and the crash recovery mechanism remains active (NFR7)

**Given** a crash occurs and the application restarts
**When** the restart completes
**Then** the crash event is logged to the persistent log file for later review

### Story 6.3: Platform Installer & OS Prerequisites

As a venue technician,
I want the installer to automatically configure all OS-level requirements — including the packet capture driver and firewall rules — so that the application works immediately after installation without manual system configuration,
So that I can deploy ARTNET-TOOL following the bundled setup guide without needing to be a system administrator.

**Acceptance Criteria:**

**Given** the Windows installer is run
**When** it executes (with UAC prompt once)
**Then** Npcap is installed silently in the background if not already present
**And** Windows Defender Firewall rules are added via NSIS post-install script to allow ArtNet/sACN traffic on the configured interface

**Given** the installer runs on macOS
**When** it completes
**Then** the application has the necessary network capture entitlements configured

**Given** the installer runs on Linux ARM
**When** it completes
**Then** the necessary permissions for raw packet capture are configured (e.g., `setcap` or group membership)

**Given** a venue tech with no prior ARTNET-TOOL experience
**When** they follow the bundled setup guide
**Then** they can install the application and load a project file successfully (NFR13)

**Given** the installer completes on any platform
**When** the application first launches
**Then** it can capture ArtNet/sACN traffic without requiring additional manual OS configuration by the user

---

## Epic 7: Observability & Reliability Polish

Users can view a persistent event/crash log from within the application, and the system notifies users of available software updates without auto-installing. After this epic, the application has complete operational observability for venue technicians and self-managed update awareness.

### Story 7.1: Persistent Event & Crash Logging

As a venue technician,
I want the application to log all significant events, errors, and crashes to a persistent local log file with automatic daily rotation,
So that I can diagnose problems after the fact even when nobody was present when they occurred.

**Acceptance Criteria:**

**Given** the application is running
**When** a significant event occurs (scene trigger, playback start/stop, capture start/stop, project load, error, crash)
**Then** it is written to the persistent local log file with a timestamp

**Given** the application experiences a crash
**When** it restarts
**Then** the crash event is present in the log file from the previous session

**Given** the log file grows over time
**When** a new day begins
**Then** `tracing-appender` rotates to a new log file
**And** a maximum of 30 log files are retained (older files are deleted)

**Given** the log file on disk
**When** reviewed
**Then** it contains no user project data, scene content, or DMX values — only operational events and errors (NFR20)

**Given** the application is running unattended for 30 days
**When** the log rotation mechanism is inspected
**Then** it has been running correctly and disk usage has not grown unboundedly (NFR7)

### Story 7.2: In-App Log Viewer

As a venue technician,
I want to view the event and crash log from within the application,
So that I can diagnose issues without needing to find the log file on disk manually.

**Acceptance Criteria:**

**Given** the Settings tab is open
**When** the user navigates to the Log sub-tab
**Then** the contents of the current log file are displayed in a scrollable view

**Given** the log viewer is open
**When** the user wants to see a previous session
**Then** they can access at minimum the current and previous session log files

**Given** a crash occurred in a previous session
**When** the log viewer is opened after restart
**Then** the crash entry is visible and readable in the log

**Given** the log viewer displays entries
**When** the user reads them
**Then** all entries are in plain language with no raw error codes or stack traces visible (NFR14)

### Story 7.3: Software Update Notifications

As a lighting operator,
I want to be notified when a new version of ARTNET-TOOL is available, without the application downloading or installing anything automatically,
So that I can choose when to update without risking an unplanned interruption to a live deployment.

**Acceptance Criteria:**

**Given** the application is running with internet connectivity
**When** a new software version is available
**Then** a non-blocking notification is displayed informing the user of the available update (FR42)

**Given** the update notification is shown
**When** the user dismisses it
**Then** it does not reappear until the next application launch or the next check interval

**Given** the update check runs
**When** it executes
**Then** no user data, project data, or telemetry is transmitted — only a version check request is sent (NFR20)
**And** no download or installation occurs automatically

**Given** the application is running with no internet connectivity
**When** the update check cannot reach the server
**Then** no error is shown — the check fails silently

**Given** the Settings tab
**When** the user navigates to the Updates sub-tab
**Then** the current application version and last update check result are visible
