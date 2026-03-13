---
stepsCompleted: ['step-01-init', 'step-02-discovery', 'step-02b-vision', 'step-02c-executive-summary', 'step-03-success', 'step-04-journeys', 'step-05-domain', 'step-06-innovation', 'step-07-project-type', 'step-08-scoping', 'step-09-functional', 'step-10-nonfunctional', 'step-11-polish', 'step-12-complete']
status: complete
completedAt: '2026-03-06'
inputDocuments:
  - '_bmad-output/planning-artifacts/product-brief-ARTNET-TOOL-2026-03-06.md'
  - '_bmad-output/brainstorming/brainstorming-session-2026-03-06-001.md'
workflowType: 'prd'
briefCount: 1
researchCount: 0
brainstormingCount: 1
projectDocsCount: 0
classification:
  projectType: desktop_app
  domain: creative_technology
  complexity: medium
  projectContext: greenfield
---

# Product Requirements Document - ARTNET-TOOL

**Author:** NODATA
**Date:** 2026-03-06

## Executive Summary

ARTNET-TOOL is a standalone ArtNet recorder and playback engine for the creative technology and live performance space. It passively captures DMX/ArtNet traffic from any source software — TouchDesigner, Resolume, MadMapper, GrandMA — stores it as named, triggerable scenes, and plays it back completely independently. No source license required on the deployment machine.

The product addresses a structural gap in the lighting workflow: the space between creation and deployment. Creative technologists build shows in expensive licensed software, then face an impossible choice — leave that license running on every deployment machine, or invest in professional consoles just to play back what they already made. ARTNET-TOOL eliminates that choice. Record once during creation. Deploy anywhere without the software.

Target users: freelance light designers and creative technologists deploying immersive installations, venues and operators running autonomous lighting environments, and VJs who need a dedicated DMX layer separate from their visual performance rig.

The democratization of ArtNet-capable lighting hardware has created a generation of installations, immersive rooms, and branded environments built by individuals — not major venues with full AV teams. The deployment tooling has not kept pace. ARTNET-TOOL is that missing piece.

**Classification:** Desktop application (cross-platform: Windows, macOS, Linux ARM) · Domain: Creative technology / entertainment · Complexity: Medium · Context: Greenfield

### What Makes This Special

The magic moment is immediate and visceral: you record your show from the network, hit stop, kill your source software entirely — and the lights still play. That independence is the product's core value, and no existing tool delivers it.

ARTNET-TOOL occupies an uncontested position: it is not a DMX creator, not a lighting console, not a controller. It is a *liberator* — a bridge between creation tools and physical output that makes the playback autonomous, portable, and source-agnostic. Existing tools all live on one side of the gap (generating DMX or controlling fixtures). This tool captures the signal in transit and makes it free.

Key differentiators:
- **Passive capture** — sniffs ArtNet from any software without proxy, interruption, or integration
- **Source independence** — playback requires zero connection to the originating software or hardware
- **Creative playback controls** — speed, reverse, ping-pong, loop modes treat DMX recordings as media clips, not binary on/off signals
- **Unattended operation** — auto-starts on boot, runs headless, requires no operator presence
- **Player paradigm** — a focused deployment tool, not another creator in a market full of creators

## Success Criteria

### User Success

The product succeeds for users when:

- A creative technologist completes a full installation deployment without leaving any personal software license on the deployment machine
- The system runs unattended for the full duration of an installation (days to weeks) without requiring designer intervention — crashes that require manual restart constitute product failure; auto-recovering crashes are acceptable
- A non-technical venue operator (bar staff, house tech) independently switches between scenes for different operational moments (service, performance, cleaning) without contacting the designer
- The designer configures the system once, hands it off, and the venue owns it completely from that point forward

**The "It's Working" Moment:** A venue runs a full week of events cycling through scenes automatically — without a single call to the designer.

**Critical Failure Condition:** Any crash or failure that takes the installation dark and requires human intervention fails the product's core promise, regardless of all other features.

### Business Success

**Primary 12-Month Success Metric:** ARTNET-TOOL is known and recommended in the creative technology and lighting community — mentioned unprompted in Discord servers, Reddit threads, Instagram posts, and YouTube content as the recognized tool for ArtNet playback.

**Supporting milestones:**
- First paying customers outside creator's immediate network: 3 months post-launch
- Designers reuse it on consecutive projects (not a one-off): 6 months
- Appears in community search results and discussions for "ArtNet player" / "DMX recorder software": 6 months
- Designer-to-designer recommendation becomes the primary acquisition channel: 12 months

### Technical Success

- Runs reliably on low-spec consumer hardware: minimum target is Raspberry Pi 4 class (4GB RAM, ARM CPU) and equivalent mini PCs — no GPU required, no high-end specs assumed
- CPU and RAM footprint stays lean enough to share a machine with other processes without resource conflict
- ArtNet playback timing is frame-accurate — DMX output is indistinguishable from the original recording
- Auto-start on boot completes and reaches playback state without user interaction
- Auto-recovery from non-fatal crashes with no visible interruption to the end user

### Measurable Outcomes

| Outcome | Target | Timeframe |
|---|---|---|
| Unattended operation | Zero designer-intervention incidents per deployment | Per installation |
| Non-technical operation | Venue staff operate scene switching independently | Per deployment |
| Community visibility | Mentioned/recommended in creative tech forums and communities | 6 months |
| First external sales | Paying customers outside creator's network | 3 months |
| Deployment reuse | Used on multiple consecutive projects by same designer | 6 months |
| Community reputation | Recognized as the tool for ArtNet playback use case | 12 months |

## User Journeys

### Journey 1: The Designer — Recording and Deploying an Installation

*Marco is a freelance light designer. He's just finished a 12-minute automated light sequence for a pop-up brand activation — three days of work in TouchDesigner, two universes, 48 fixtures. The client wants it running on a mini PC in the venue for two weeks, no supervision.*

**The problem he's faced before:** He leaves his TouchDesigner license on the client's machine, the rental ends, the license is stuck. Or he buys a temp license just to play back something he already made. Every deployment eats into his stack.

**Opening scene:** Marco installs ARTNET-TOOL on his studio machine alongside TouchDesigner. He opens it, sees the Differential Monitor tab. Hits record. Runs his full sequence in TouchDesigner — the monitor lights up, channels moving, universes active. He hits stop. The scene is captured, named, saved.

**Rising action:** He closes TouchDesigner entirely. Hits play. The sequence runs — identical, frame-accurate. He configures the scene: loop mode, auto-start on boot, no schedule needed (runs all day). He exports the project file.

**Climax:** He transfers the project file to the venue's mini PC, installs ARTNET-TOOL (free to install, no license tied to playback), loads the project. Reboots the machine. ARTNET-TOOL opens automatically, loads the show, starts playing — without him touching anything.

**Resolution:** Marco leaves the venue. The show runs for two weeks. No license on the machine, no calls from the client. He uses the same workflow on the next three projects. ARTNET-TOOL is now the last step in every delivery.

*Capabilities revealed: ArtNet passive recording, scene save/load, loop playback mode, auto-start on boot, project file export/import.*

---

### Journey 2: The Venue Operator — Running the Space on a Normal Day

*Sofia is the floor manager at an immersive cocktail bar. The light show was set up three months ago by the designer. She doesn't know what ArtNet is. She knows there's a PC in the back room that runs the lights.*

**The problem she faces:** Different moments need different lighting. Bar service needs warm ambient. DJ night needs the full reactive show. Private events need something softer. She used to call the designer every time. That stopped working after the second month.

**Opening scene:** It's 9 PM, the DJ is starting. Sofia walks to the back room PC and presses the keyboard shortcut the designer wrote on a sticky note on the monitor. She presses the key labelled "Show." The lights shift — the full sequence kicks in. Later, at 2 AM, she presses "Service" for closing. She doesn't need to understand the software — she just presses a key.

**Rising action:** A full month passes. Sofia switches scenes for service, DJ nights, private events, cleaning — the same four keys, every day. Zero calls to the designer.

**Climax:** The designer checks in after a month. Sofia hasn't needed to contact him once. The venue owns the system completely.

**Resolution:** The sticky note becomes a laminated card next to the PC. The handoff is permanent.

*Capabilities revealed: Named cue pads with clear scene labels, keyboard trigger shortcuts, intuitive enough for zero training. V2: web remote for phone-based switching.*

---

### Journey 3: The Designer — Recovery After a Failure Call

*It's a Saturday night. Marco gets a text from Sofia: "lights stopped, what do I do?"*

**Opening scene:** The installation has been running for 10 days. Something happened — a power cut, a machine lockup. The lights are off. Sofia is standing in a bar full of people.

**Rising action:** Marco's first instruction: "Restart the machine." Sofia holds the power button, turns it back on. ARTNET-TOOL auto-starts on boot, loads the last project, resumes playback. Lights come back up within 60 seconds. Marco never needed to go on-site.

**Climax (failure scenario):** Auto-start doesn't recover — the project file is corrupted, or ARTNET-TOOL isn't responding. Marco remotes in via standard Remote Desktop, checks the event log, sees the error, fixes it in two minutes. Contained. Fixable remotely.

**Resolution:** The incident revealed the value of two things: auto-start as the first line of recovery, and a crash/event log so Marco can diagnose without asking Sofia to describe what she sees on screen.

*Capabilities revealed: Reliable auto-start on boot as first recovery mechanism, event/crash log for remote diagnosis, project file integrity validation on load.*

---

### Journey 4: The VJ — Live Performance Integration

*Kai is a VJ performing at a festival. Their rig: a laptop running Resolume for visuals, a second mini PC running ARTNET-TOOL for DMX lighting. They want the lighting layer fully separated from the visual layer — different machine, different failure domain.*

**Opening scene:** During soundcheck, Kai opens ARTNET-TOOL on the mini PC. They have six scenes recorded: "Intro Build," "Drop," "Breakdown," "Ambient," "Strobe," "Blackout." Each has a MIDI note mapped to a pad on their Launchpad.

**Rising action:** The set starts. Kai fires scenes from the Launchpad — ARTNET-TOOL receives the MIDI, triggers the corresponding scene. The lighting layer runs independently. If Resolume crashes, the lights keep going. If ARTNET-TOOL crashes, the visuals keep going.

**Climax:** During the drop, Kai wants to push the strobe faster than recorded. They dial speed to 2.0x on the fly — the strobe rate doubles. Creative playback control, live, from the keyboard.

**Resolution:** After the set, Kai tells two other VJs about the setup. "It's just a dedicated DMX player, MIDI-mapped to your controller. Costs nothing to run on a second machine."

*Capabilities revealed: MIDI trigger per scene, per-scene speed control adjustable live, reliable isolated operation as a separate failure domain.*

---

### Journey 5: The Remote Project Handoff — Designer to Venue Tech

*A venue in another city hires Marco remotely. He can't be on-site for the install. Their local AV tech, Denis, will handle the physical setup.*

**Opening scene:** Marco builds and records the entire show from his studio. He configures the scenes, schedules, boot behavior, and trigger mappings. He exports the project file and sends it to Denis with a one-page setup guide: "Install ARTNET-TOOL (free, link below), open this file, plug in the network cable, reboot."

**Rising action:** Denis has never used ARTNET-TOOL. He follows the guide. Opens the project file — all scenes are there, named, configured, with schedules already set. He plugs in the ArtNet cable. Reboots. The show starts automatically.

**Climax:** Denis calls Marco: "It just works." Marco wasn't there. The handoff was a file.

**Resolution:** The project file becomes the delivery artifact — the thing Marco hands to clients and venue techs. ARTNET-TOOL becomes the player they install once to receive it. The separation between creation and deployment is complete.

*Capabilities revealed: Project file as complete, portable, self-contained delivery artifact; all configuration (scenes, schedules, boot behavior, trigger mappings) embedded in the file; zero-friction install on a fresh machine.*

---

### Journey Requirements Summary

| Journey | Capabilities Required |
|---|---|
| Designer — Deployment | Passive ArtNet recording, scene save/load, loop/playback modes, auto-start, project file export/import |
| Venue Operator — Daily | Named cue pads, keyboard shortcuts, intuitive labeling; V2: web remote |
| Designer — Recovery | Auto-start on boot as recovery mechanism, event/crash logging, project file integrity validation |
| VJ — Live Performance | MIDI trigger mapping, live speed control, reliable isolated operation |
| Remote Handoff | Self-contained project file with all config embedded, clean install on new machine |

## Domain-Specific Requirements

### Protocol Compliance

- **ArtNet (Art-Net 4 spec)** — primary protocol; passive capture and playback must be fully spec-compliant. Art-Net version backward compatibility assumed standard — no special handling required.
- **sACN / E1.31** — secondary protocol; both recording and playback must support sACN in addition to ArtNet. Many modern fixtures and controllers support both; sACN support expands compatibility with professional-grade hardware.
- **DMX512 framing** — underlying DMX data must be stored and replayed with frame-accurate timing. Playback output must be indistinguishable from the original signal at the fixture level.

### Network Environment Constraints

- **Interface selection** — the deployment machine may have multiple network interfaces (ArtNet LAN + internet/management). The user must be able to specify which interface ARTNET-TOOL listens on (capture) and transmits on (playback). This is a required configuration, not an assumption.
- **Primary transport: wired ethernet** — all ArtNet/sACN capture and playback is expected to operate over wired ethernet. This is the standard deployment topology; wireless is not a supported transport for DMX output.
- **Wireless for control interfaces only** — wireless network access is acceptable and expected for control surfaces (web remote on a tablet or phone in V2). The DMX signal path must remain wired.
- **Broadcast and multicast** — ArtNet uses UDP broadcast; sACN uses UDP multicast. The tool must handle both correctly on the selected interface, including environments where broadcast is restricted (some managed venue networks).

### Technical Constraints

- **Admin privileges required on Windows** — passive ArtNet/sACN capture requires raw socket or packet capture access (WinPcap/Npcap driver). Admin privileges during install are an acceptable requirement. This must be clearly communicated during installation.
- **Firewall and port requirements** — ArtNet uses UDP port 6454; sACN uses UDP ports 5568 (unicast) and multicast addresses. The installer or first-run setup must handle Windows Firewall rules for these ports automatically, without requiring the user to configure them manually.
- **Low resource footprint** — must run on Raspberry Pi 4 class hardware (4GB RAM, ARM CPU) and equivalent x86 mini PCs. No GPU dependency. Scene data must be loaded into memory at project load time, not streamed from disk during playback.

### Risk Mitigations

| Risk | Mitigation |
|---|---|
| Network interface not found or disconnected | Clear error on startup with interface selection UI; graceful degradation if interface goes offline during playback |
| Broadcast blocked on managed venue networks | sACN unicast as fallback; document network requirements for venue IT |
| Admin/driver install fails on deployment machine | Clear install failure messaging; alternative install path documented |
| sACN multicast routing issues in multi-subnet venues | Unicast sACN mode as alternative; document configuration |

## Innovation & Novel Patterns

### Detected Innovation Areas

**1. The Player Paradigm — A New Tool Category**
ARTNET-TOOL doesn't compete in an existing category — it creates one. Every existing DMX tool is either a creator (generates DMX signals) or a controller (manages fixtures directly). ARTNET-TOOL is a *player*: it captures signals in transit and makes them autonomous. This positioning is the innovation — occupying a gap the market hasn't named yet.

**2. Source-Agnostic Passive Capture**
Capturing ArtNet/sACN by listening passively on the network — rather than integrating with or replacing the source software — means the tool works with everything by design. TouchDesigner, Resolume, GrandMA, MadMapper, any future tool: all compatible without a single integration. Zero dependencies is a structural advantage that compounds over time as the ecosystem grows.

**3. DMX-as-Media-Clip Playback Model**
Applying media player paradigms (speed control, reverse, ping-pong, loop modes) to DMX data reframes what a DMX recording *is*. Current tools treat DMX as static state snapshots or binary play/stop. Treating it as a manipulable clip — with creative controls a performer can adjust live — opens a layer of expressiveness that doesn't exist in any standalone DMX tool.

**4. Project File as Delivery Artifact**
Reframing the project file as the handoff between designer and venue transforms the deployment workflow. The file *is* the product delivery — a complete, portable, self-contained show that any machine can receive and run. This shifts the designer's relationship with clients from ongoing dependency to one-time delivery.

### Market Context & Competitive Landscape

No existing standalone tool occupies the "record once, deploy anywhere" position in the ArtNet workflow. The competitive landscape consists of:
- **Creation tools** (TouchDesigner, Resolume, MadMapper) — require full licenses on every machine; designed for real-time authoring, not deployment
- **Professional consoles** (GrandMA, ETC Eos) — overkill and cost-prohibitive for playback-only scenarios
- **DMX controllers** (QLC+, DMXControl) — generate DMX from scratch; fundamentally a different paradigm
- **Nothing** occupies the capture-and-replay position

The gap is structural, not a feature gap. Existing tools can't address it without fundamentally changing what they are.

### Validation Approach

- **Core capture/playback loop**: validated when a complete show recorded in TouchDesigner plays back identically on a separate machine with no source software present — this is the primary proof-of-concept milestone
- **Platform differentiation**: validated when a light designer reports using ARTNET-TOOL as their standard delivery workflow across multiple projects, replacing previous workarounds
- **Player paradigm adoption**: validated when users begin describing ARTNET-TOOL as a distinct category ("my DMX player") rather than comparing it to existing tools

### Risk Mitigation

| Innovation Risk | Mitigation |
|---|---|
| Market doesn't know it needs this category | The "magic moment" demo (kill the source software, lights keep playing) is self-explanatory — no education required |
| Passive capture is technically complex on some OS/network configs | Admin install requirement accepted; clear setup guidance; WinPcap/Npcap as proven solution |
| Creative playback controls feel niche vs. core capture value | Core promise is capture/playback independence; creative controls are additive, not required for MVP value |
| Competitors could add this feature | First-mover in an uncontested niche with community reputation as the moat |

## Desktop Application Specific Requirements

### Project-Type Overview

ARTNET-TOOL is a cross-platform native desktop application targeting Windows, macOS, and Linux ARM. It requires deep system integration (network packet capture, MIDI input, OS startup registration) while maintaining a low resource footprint suitable for permanently deployed mini PCs and single-board computers. The application must function fully offline with no internet dependency at any point in its operation.

### Platform Support

| Platform | Target | Architecture | Priority |
|---|---|---|---|
| Windows | Windows 10+ | x86/x64 | Primary |
| macOS | macOS 12+ (Monterey) | Intel + Apple Silicon (Universal Binary) | Required |
| Linux ARM | Debian/Ubuntu-based | ARM64 (Raspberry Pi 4+) | V4 hardware deployment |

**Platform-specific notes:**
- Windows: requires Npcap driver for packet capture; installer must handle driver installation and Windows Firewall rules automatically
- macOS: requires `libpcap` (built into macOS); app may require network permissions via macOS privacy/security prompts on first run; notarization required for distribution outside App Store
- Linux ARM: headless operation must be fully supported; no display required for playback mode; systemd service for auto-start on boot

### System Integration

- **Network packet capture** — raw socket access via Npcap (Windows) / libpcap (macOS, Linux) for passive ArtNet/sACN sniffing; user-selectable network interface
- **MIDI input** — system MIDI device enumeration and per-scene note/CC trigger mapping; must detect hot-plugged MIDI devices without restart
- **OS boot startup registration** — registers as a startup application via OS-native mechanisms (Windows Task Scheduler or registry run key; macOS LaunchAgent plist; Linux systemd service); configurable per-project
- **Firewall rule management** — installer automatically creates inbound UDP rules for ArtNet (port 6454) and sACN (port 5568 + multicast) on Windows; macOS and Linux rely on standard permissions model

### Update Strategy

- **No automatic updates** — ARTNET-TOOL never updates itself automatically. An unattended deployment machine must not change state without the designer's deliberate action.
- **Update check (notification only)** — the application may check for available updates in the background when internet is present and notify the user via a non-intrusive indicator. No download or install occurs without explicit user action.
- **Manual update process** — the designer downloads and installs updates manually. The update must not interrupt an active project or alter project file format without a migration path.
- **Version stability** — a project file created on version N must remain fully functional on version N+1 (forward-compatible project format).

### Offline Capabilities

- **Fully offline by design** — all core functionality (recording, playback, scheduling, MIDI triggering, cue pad operation) operates with zero internet connectivity. Internet access is never required.
- **No cloud dependency** — no user accounts, no license activation servers, no telemetry. The tool works on an air-gapped machine in a venue with no network access beyond the local ArtNet LAN.
- **Local-only data** — all project files, scene recordings, and configuration are stored locally. No sync, no cloud backup, no remote storage.
- **Update check is optional** — if no internet is present, the update check fails silently with no impact on operation.

### Implementation Considerations

- **Cross-platform framework**: must support Windows, macOS, and Linux ARM from a single codebase. Electron/Tauri (web-based UI) or a native cross-platform toolkit are the primary candidates — decision to be made during architecture phase, informed by the ARM/headless requirement.
- **Headless mode**: on Linux ARM deployments, the application must run without a display server (no GUI required for playback). A minimal CLI or daemon mode for pure playback is required for V4.
- **Packaging**: each platform requires native packaging — `.exe`/`.msi` installer (Windows), `.dmg`/notarized `.app` (macOS), `.deb` package or install script (Linux ARM).
- **Performance floor**: UI rendering must not compete with the ArtNet playback engine for CPU time. Playback timing must be isolated from UI thread scheduling.

## Product Scope

### MVP Strategy & Philosophy

**MVP Approach:** Problem-Solving MVP — deliver the complete core value proposition end-to-end before adding anything. The MVP proves the fundamental promise works: a designer records a show, transfers it to a separate machine, walks away, and it runs unattended. Every MVP feature is load-bearing for this proof.

**Validation Gate:** The MVP is complete when a designer can complete the full record → deploy → forget workflow on real hardware, with a non-technical person able to switch scenes independently, and the system running autonomously for multiple consecutive days without intervention.

**Resource Requirements:** Solo developer (single skilled full-stack developer with desktop app and networking experience). The 6-feature MVP scope is sized for this constraint — no feature is included that isn't essential to the core promise.

### MVP Feature Set (Phase 1)

**Core User Journeys Supported:**
- Journey 1: Designer recording and deploying an installation
- Journey 2: Venue operator running the space day-to-day
- Journey 3: Designer recovering from a remote failure
- Journey 5: Remote project file handoff to venue tech

*(Journey 4 — VJ live performance — is supported by the MIDI trigger and speed control features included in MVP)*

**Must-Have Capabilities:**

| Feature | Why It's MVP | Omitting It Fails |
|---|---|---|
| ArtNet/sACN Passive Recorder | The entire product premise | Everything |
| Scene Library / Cue Pad | Makes recordings triggerable | Venue operator journey |
| Playback Controls (loop, speed, reverse, ping-pong) | Core creative value + VJ use case | Creative differentiation |
| Differential Monitor | Record-mode: confirms capture; Playback-mode: signal debug for professionals | Recording confidence, pro debugging |
| Auto-Start on Boot | Enables unattended deployment | Installation journey |
| Scheduling | Enables autonomous venue operation without any human | Venue operator journey |
| Project File Export/Import | Enables remote handoff; project as delivery artifact | Handoff journey |
| Event/Crash Log | Enables remote diagnosis without on-site visit | Recovery journey |

**Scoping Notes:**
- The Differential Monitor is present in both modes but role-shifts: incoming signal during recording, outgoing channels during playback. Always accessible, never mandatory for the core playback flow.
- Project file portability (export/import) was not in the original feature list but is revealed as essential by the remote handoff journey — added to MVP.
- Event/crash log similarly revealed as essential by the recovery journey.

### Post-MVP Features

**Phase 2 — Live Control Layer:**
- **Web Remote** — local-network smartphone controller with scene pads, master fader, and blackout panic button; enables venue staff to switch scenes from a phone instead of the back room PC
- **Timeline / DAW View** — sequence scenes on a time axis for complex show choreography
- **Show Mode** — locked fullscreen performance state optimized for dark stage environments

**Phase 3 — Creative Power:**
- **Scene Layering Engine** — stack recorded scenes as layers with blend modes (HTP, LTP, additive), per-universe masking, independent speed/direction
- **Channel Sculptor** — post-capture curve editing for individual DMX channels
- **Portable DMX Node** — headless deployment on Raspberry Pi or mini NUC; transforms software into an embeddable hardware product

### Risk Mitigation Strategy

**Technical Risks:**

| Risk | Severity | Mitigation |
|---|---|---|
| Cross-platform passive packet capture (Npcap/libpcap) behaves differently per OS | High | Prototype this first — it's the riskiest technical assumption; validate on all 3 platforms before building UI |
| Real-time ArtNet playback timing precision on low-spec hardware | High | Isolate playback engine from UI thread from day one; test on Raspberry Pi 4 early |
| macOS notarization and network permission prompts | Medium | Build macOS packaging and notarization into CI from the start, not at release |

**Market Risks:**

| Risk | Mitigation |
|---|---|
| Market doesn't know this category exists | The "magic moment" demo is self-explanatory — show the lights playing after killing the source software |
| Early adopters need hand-holding | Build alongside real deployments; use own projects as first test cases |
| Community adoption is slow | Creator-led marketing: post the workflow publicly, demonstrate on real projects |

**Resource Risks:**

| Risk | Mitigation |
|---|---|
| Solo developer scope creep | Strict MVP boundary: the 8 features above, nothing else until the validation gate is passed |
| Platform support complexity multiplies work | Validate cross-platform compatibility early; defer Linux ARM to post-MVP if needed |
| Unattended reliability requires extensive testing | Treat reliability as a first-class requirement from day one, not a QA pass at the end |

## Functional Requirements

### Signal Capture

- **FR1:** User can start and stop passive network recording that captures ArtNet/sACN traffic from any source on the local network
- **FR2:** System captures ArtNet/sACN traffic without modifying, intercepting, or interrupting the source software or hardware
- **FR3:** User can select which network interface the system listens on for capture
- **FR4:** System captures time-based DMX animations with full temporal fidelity
- **FR5:** System captures static DMX snapshots as single-frame scenes
- **FR6:** System captures traffic from multiple ArtNet/sACN universes simultaneously in a single recording session

### Scene Management

- **FR7:** User can save a completed recording as a named scene in the scene library
- **FR8:** User can rename, reorder, and delete scenes
- **FR9:** User can configure playback mode per scene (loop, one-shot, ping-pong, reverse)
- **FR10:** User can configure playback speed per scene as a multiplier
- **FR11:** User can assign a keyboard shortcut trigger to a scene
- **FR12:** User can assign a MIDI note or CC mapping as a trigger for a scene

### Playback Engine

- **FR13:** System plays back a recorded scene as ArtNet/sACN output with frame-accurate timing
- **FR14:** System plays back scenes completely independently of the original source software or hardware
- **FR15:** User can select which network interface is used for ArtNet/sACN playback output
- **FR16:** User can adjust a scene's playback speed in real-time during active playback
- **FR17:** System plays scenes in the configured mode (loop, one-shot, ping-pong, reverse)

### Triggering & Control

- **FR18:** User can trigger a scene via its configured keyboard shortcut
- **FR19:** User can trigger a scene via its configured MIDI input
- **FR20:** User can trigger a scene manually from the scene library interface
- **FR21:** System detects hot-plugged MIDI devices and makes them available without requiring a restart
- **FR22:** User can configure a time-based schedule for a scene specifying active time windows and day recurrence
- **FR23:** System automatically switches to the scheduled scene at the configured time without user interaction

### Signal Monitoring

- **FR24:** User can view all active ArtNet/sACN universes and their DMX channel values in real-time
- **FR25:** System visually differentiates channels that are actively changing, static, and zeroed
- **FR26:** During recording, the monitor displays incoming captured signal from the network
- **FR27:** During playback, the monitor displays the outgoing DMX signal being transmitted

### Project Management

- **FR28:** User can create, save, and load projects that contain all scenes and configuration
- **FR29:** User can export a project as a single portable file containing all scenes, schedules, trigger mappings, and settings
- **FR30:** User can import a project file on a different machine and have it operate identically
- **FR31:** User can configure which scene is active and which playback mode is used when a project loads
- **FR32:** System validates project file integrity on load and reports specific errors if the file is corrupt or unreadable
- **FR33:** Project files created on earlier versions of the application remain fully loadable on later versions

### System & Boot Integration

- **FR34:** User can configure the application to launch automatically when the OS starts
- **FR35:** When configured for auto-start, the system loads the last active project automatically on launch
- **FR36:** When configured for auto-start, the system begins playback automatically on launch without user interaction
- **FR37:** Installer automatically configures OS-level requirements (firewall rules, packet capture driver) needed for operation

### Diagnostics & Recovery

- **FR38:** System logs all significant application events, errors, and crashes to a persistent local log file
- **FR39:** User can view the event and crash log from within the application
- **FR40:** System automatically recovers from non-fatal crashes and resumes playback without requiring manual intervention
- **FR41:** System displays a clear error with actionable guidance when the configured network interface is unavailable at startup
- **FR42:** System notifies the user of available software updates when internet connectivity is present, without downloading or installing automatically

## Non-Functional Requirements

### Performance

- **NFR1 — Playback timing precision:** ArtNet/sACN output during playback must be frame-accurate — DMX packet intervals must not deviate from the recorded timing by more than ±1ms under normal operating conditions
- **NFR2 — Capture timing fidelity:** Recorded scene data must preserve original packet timestamps with sufficient resolution to reproduce frame-accurate playback
- **NFR3 — Startup to playback:** When auto-start is configured, the system must reach active playback state within 30 seconds of OS boot completing, without user interaction
- **NFR4 — Resource ceiling:** Total CPU usage must remain below 15% and RAM usage below 300MB on the minimum target hardware (Raspberry Pi 4, 4GB RAM) during active playback with no UI rendering
- **NFR5 — UI thread isolation:** Playback engine timing must not be affected by UI rendering, user interactions, or background tasks — the playback loop runs on a dedicated isolated thread/process
- **NFR6 — Scene switch latency:** Triggering a scene (via keyboard, MIDI, or schedule) must result in DMX output beginning within 50ms of the trigger event

### Reliability

- **NFR7 — Continuous operation:** The system must operate without degradation for a minimum of 30 consecutive days of unattended playback under normal conditions
- **NFR8 — Crash recovery:** The system must automatically restart and resume playback within 60 seconds of a non-fatal crash, without requiring human intervention
- **NFR9 — Signal continuity:** In the event of a non-fatal crash during playback, the DMX output gap (dark period) must be less than 60 seconds before the system auto-recovers
- **NFR10 — Boot recovery:** Following an unplanned machine shutdown (power loss, forced reboot), the system must automatically restart, reload the project, and resume playback on next boot without manual intervention
- **NFR11 — Project file integrity:** The system must detect corrupt or incomplete project files at load time and report a specific, actionable error rather than silently failing or crashing

### Usability

- **NFR12 — Non-technical operation:** A venue operator with no prior training must be able to switch between scenes using configured keyboard shortcuts after a single explanation — the interaction must require no understanding of ArtNet or DMX
- **NFR13 — Setup documentation:** A venue tech unfamiliar with ARTNET-TOOL must be able to install the application and load a project file successfully by following the bundled setup guide, without requiring the designer's assistance
- **NFR14 — Error clarity:** All error messages must describe what went wrong and what the user should do next in plain language — no raw error codes, stack traces, or technical jargon visible to end users in the main UI

### Compatibility

- **NFR15 — ArtNet protocol compliance:** Captured and replayed ArtNet packets must conform to the Art-Net 4 specification; Art-Net 3 sources must be captured and replayed correctly via backward compatibility
- **NFR16 — sACN protocol compliance:** Captured and replayed sACN packets must conform to the ANSI E1.31 specification for both unicast and multicast modes
- **NFR17 — Cross-platform behavior parity:** Core functionality (recording, playback, scheduling, MIDI triggering, auto-start) must behave identically on Windows, macOS, and Linux ARM — no platform-exclusive feature degradation for MVP capabilities
- **NFR18 — Project file portability:** A project file created on Windows must load and operate correctly on macOS and Linux ARM without modification
- **NFR19 — MIDI spec compliance:** MIDI input handling must conform to the MIDI 1.0 specification for Note On/Off and Control Change messages across all supported platforms

### Security

- **NFR20 — Local-only operation:** The application must not transmit any user data, project data, telemetry, or usage analytics to any remote server. All data remains on the local machine.
- **NFR21 — Project file safety:** The application must not execute arbitrary code from project files — project file parsing must be sandboxed and validated against a known schema before any data is processed
- **NFR22 — No persistent network services:** The application must not expose any open network ports or services on the local machine beyond what is required for ArtNet/sACN operation on the configured interface
