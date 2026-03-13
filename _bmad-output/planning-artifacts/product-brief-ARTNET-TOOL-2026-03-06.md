---
stepsCompleted: [1, 2, 3, 4, 5, 6]
inputDocuments: ['_bmad-output/brainstorming/brainstorming-session-2026-03-06-001.md']
date: 2026-03-06
author: NODATA
---

# Product Brief: ARTNET-TOOL

<!-- Content will be appended sequentially through collaborative workflow steps -->

## Executive Summary

ARTNET-TOOL is a standalone ArtNet recorder and playback engine that fills a
critical gap in the lighting workflow: the space between creation and deployment.
It passively captures DMX/ArtNet signals from any source software, stores them as
replayable scenes, and plays them back independently — no source license required
on the deployment machine. Built by a light designer for creative technologists,
light artists, and venues that need autonomous or operator-controlled light playback
without the cost and complexity of professional consoles or licensed VJ software.

---

## Core Vision

### Problem Statement

Creative technologists and light designers working with ArtNet-based systems are
forced to keep expensive licensed software (TouchDesigner, Resolume, MadMapper)
installed on every deployment machine, or invest in professional consoles (GrandMA)
just to play back pre-recorded scenes. There is no dedicated tool for the simple but
critical task of capturing and replaying DMX/ArtNet data independently of its source.

The result: personal licenses are consumed by installations and venues instead of
creative work, and there is no affordable, lightweight option for spaces that simply
need lights to play — automatically, reliably, without a lighting designer present.

### Problem Impact

- License costs are duplicated across every deployment machine
- Personal creative licenses are tied up in venues and installations
- Immersive spaces and venues without dedicated LDs have no simple, affordable
  playback solution
- Playback machines are permanently coupled to complex, heavyweight software
- The barrier to autonomous light automation is disproportionately high for the
  simplicity of the task

### Why Existing Solutions Fall Short

- **TouchDesigner / Resolume / MadMapper**: Require full paid licenses on each
  playback machine; designed for real-time creation, not scene deployment
- **GrandMA / Professional consoles**: Overkill and prohibitively expensive for
  playback-only scenarios; require trained operators
- **QLC+ / DMXControl**: DMX creators and controllers — they generate DMX, they
  don't record and replay it; fundamentally a different paradigm
- **No existing tool** occupies the "record once, deploy anywhere" gap in the
  ArtNet workflow

### Proposed Solution

A lightweight, cross-platform ArtNet recorder and playback engine. It passively
sniffs ArtNet traffic from any source software during the creation phase, captures
scenes and time-based animations, and plays them back completely independently.

Playback includes creative controls — speed adjustment, reverse, loop modes,
ping-pong — treating DMX recordings like audio or video clips rather than static
snapshots. Scenes can be triggered live (keyboard, MIDI, web remote) or run
autonomously for unattended installations. Designed as the last piece of the chain:
plug in, configure once, press play — or walk away entirely.

### Key Differentiators

1. **Source-agnostic passive capture** — records ArtNet from any software with no
   proxy, no interruption, no integration required
2. **Separation of creation and playback** — expensive licenses are only needed
   during recording, never during deployment
3. **Creative playback manipulation** — speed, reverse, ping-pong, loop modes treat
   DMX like a media clip, not a binary play/stop signal
4. **Unattended operation** — runs headless for permanent installations; no operator
   required
5. **Player paradigm, not creator** — occupies an uncontested position in the
   workflow that no existing tool owns
6. **Built from real practice** — designed by a working light designer and creative
   technologist, reflecting genuine workflow needs

---

## Target Users

### Primary Users

#### Persona 1: The Creative Technologist / Light Designer

**Profile: Someone like NODATA**
A freelance light designer and creative technologist working across live events,
immersive installations, and branded experiences. Fluent in TouchDesigner, Resolume,
or MadMapper — uses these tools to create DMX/ArtNet animations and light shows.
Technically capable, but not necessarily a software developer.

**Their Problem:**
They create a show (e.g. a 13-minute automated light sequence for a brand activation
or car fair) and then face a dilemma: leave an expensive personal license running
on the client's machine for the duration of the installation, or buy a temporary
license just for playback. Every deployment eats into their license pool. Manual
boot scripts, auto-launch hacks, and watchdog processes become part of every
installation just to make something "just loop reliably."

**What They Want:**
Record the show once in their creation tool. Hand off a lightweight, self-contained
player to the client or venue. Walk away knowing it will run — automatically, on
boot, without their software or their presence.

**Their 'Aha' Moment:**
First time they deploy a full installation without leaving a single license behind.

---

#### Persona 2: The VJ / Live Performance Artist

**Profile: A VJ working in clubs, festivals, or immersive events**
Uses Resolume or similar tools to create live AV experiences where DMX-controlled
lighting is synced to visuals. Wants to hand off lighting cues to a dedicated player
so they can focus on the visual performance without also managing lights.

**Their Problem:**
Running lights from the same machine as the visual performance creates conflicts
and complexity. They want a separate, dedicated playback engine for the DMX layer
that they can trigger from MIDI or a phone — without maintaining a full second
software license.

**What They Want:**
A focused, reliable DMX player they can fire scenes from during a live set.
Lightweight enough to run on a secondary machine or mini PC in the rack.

---

### Secondary Users

#### Persona 3: The House Technician / Venue Operator

**Profile: A non-specialist employee at a venue, store, or immersive experience**
Manages the day-to-day operation of a space where a light show runs as part of the
environment — a retail flagship store, an immersive dining experience, an event
venue, or a branded pop-up. Not a lighting professional. Comfortable with
smartphones and basic UIs, but not with DMX protocols or lighting consoles.

**Their Role:**
They didn't design the light show — a creative technologist did. Their job is to
make sure it runs. They may need to start a scene, switch between modes, or trigger
a specific cue before an event opens. They should never need to understand what
ArtNet is.

**What They Need:**
A UI so simple it feels like pressing play on a media player. Ideally, the system
auto-starts on boot and requires zero daily interaction. If they do need to act,
a web remote on their phone is enough.

---

#### Persona 4: The Client / Venue Owner (Buyer)

**Profile: A brand, event organizer, or venue owner commissioning a light installation**
Paying for a bespoke light experience to be built and deployed. Not technical.
Cares about reliability, cost, and not being dependent on the designer's ongoing
presence or expensive software subscriptions.

**Their Role:**
They are often the reason the tool needs to exist on a machine permanently. They
don't operate it — they just need it to work without calling the designer every week.
They may purchase or license the tool as part of the project delivery.

**What They Need:**
A one-time setup they can forget about. A system that boots, plays, and doesn't
require a subscription to the designer's creative software stack.

---

### User Journey

#### Primary Journey: Creative Technologist deploying an installation

**Discovery:**
Searching for "ArtNet player", "DMX recorder software", "how to play DMX without
TouchDesigner" — or discovering through YouTube tutorials, Instagram posts from
the creative tech community, or word-of-mouth in Discord/Reddit spaces.

**Onboarding:**
Downloads and installs ARTNET-TOOL on their creation machine. Opens it alongside
their existing software. Hits record, runs their show in TouchDesigner/Resolume,
stops recording. Scene is captured. They configure playback mode, trigger settings,
and boot behavior. Exports or transfers the project to the deployment machine.

**Core Usage:**
On the deployment machine (which may be a mini PC or Raspberry Pi), ARTNET-TOOL
auto-starts on boot, loads the last project, and begins playback. The designer
has left the building. The show runs.

**Success Moment:**
The installation runs for days or weeks unattended. No license alerts. No software
crashes requiring a designer callout. The client doesn't need to call anyone.

**Long-term:**
ARTNET-TOOL becomes the standard final step in every installation delivery.
The designer builds a library of recorded scenes they can remix and redeploy
across projects.

---

## Success Metrics

### User Success

The product succeeds for users when:

- **Zero deployment licensing cost** — the creative technologist delivers a
  complete installation without leaving any personal software license behind
- **Unattended reliability** — the system runs continuously for days or weeks
  without crashing, dropping signal, or requiring a designer callout
- **Autonomous venue operation** — a non-technical venue operator (bar owner,
  house tech) can independently switch between scenes for different moments:
  service lighting, show mode, DJ set, live performance, cleaning — without
  needing to contact the designer
- **Frictionless handoff** — the designer configures the system once, hands it
  off, and the venue owns it completely from that point forward

**The 'It's Working' Moment:**
A venue runs a full week of events — bar service, DJ nights, a live show — cycling
through the right scenes at the right moments, without a single call to the designer.

**The Critical Failure Condition:**
If playback is unreliable or crashes under continuous unattended operation, the
product fails its core promise regardless of any other feature.

---

### Business Objectives

**Primary Model:** One-time purchase per version (perpetual license)

**12-Month Objectives:**
- Achieve meaningful adoption within the creative technologist / light designer
  community (Discord servers, Reddit communities, Instagram, YouTube)
- Generate enough sales to validate the product as a viable commercial offering
- Establish ARTNET-TOOL as the recognized tool for the "ArtNet playback" use case —
  the first result when someone searches for this specific workflow
- Build a reputation for reliability strong enough that designers recommend it to
  clients as the standard deployment solution

**Long-term Strategic Objective:**
Become the default deployment tool in the creative lighting workflow — the last
piece of every installation chain, recommended designer-to-designer as standard
practice.

---

### Key Performance Indicators

| KPI | Target | Timeframe |
|---|---|---|
| Successful unattended deployments | Users report 0 crashes in continuous operation | Per installation |
| Scene switching by non-technical users | Venue staff operate independently without designer support | Per deployment |
| Community presence | Mentioned/recommended in creative tech forums and communities | 6 months |
| Sales traction | First paying customers outside the creator's immediate network | 3 months |
| Word-of-mouth | Users actively recommend it to other designers | 12 months |
| Deployment reuse | Designers use it on multiple consecutive projects | 6 months |

---

## MVP Scope

### Core Features (V1)

The MVP delivers the complete "record, deploy, and forget" workflow:

**1. ArtNet Passive Recorder**
- Network listener that passively sniffs ArtNet traffic from any source software
- Non-intrusive — source software runs unmodified alongside the recorder
- Captures both time-based animations and static snapshots
- Multi-universe support

**2. Scene Library / Cue Pad**
- Recorded scenes organized as named, triggerable cue pads
- Keyboard trigger per scene (configurable key bindings)
- MIDI trigger per scene (configurable MIDI note/CC mapping)
- Scene management: name, rename, delete, reorder

**3. Playback Controls**
- Loop, one-shot, ping-pong, and reverse playback modes
- Basic speed control per scene
- Per-scene playback mode configuration

**4. Differential Monitor**
- Real-time view of all active ArtNet universes and DMX channels
- Highlights only channels that are changing — active channels prominent,
  static channels faded, zeroed channels invisible
- Essential for confirming capture is working and signal is live

**5. Auto-Start on Boot**
- ARTNET-TOOL launches automatically when the machine powers on
- Loads the last active project automatically
- Begins playback in the configured default mode without user interaction
- Core requirement for unattended installation deployment

**6. Scheduling**
- Time-based playback scheduling per scene or scene group
- Define active time windows (e.g. 09:00–17:00 daily)
- Recurring schedule support (daily, specific days of week, custom intervals)
- Automatic scene switching based on time of day
- Enables fully autonomous venue operation without any human trigger

---

### Out of Scope for MVP

These features are intentionally deferred to keep V1 focused and deliverable:

| Feature | Reason for Deferral |
|---|---|
| Timeline / DAW view | Cue pads cover the core use case; timeline adds complexity |
| Web Remote | Keyboard/MIDI covers live triggering; web remote is a convenience layer |
| Scene Layering Engine | High complexity; post-V1 creative power feature |
| Channel Sculptor | Post-capture editing is advanced; not needed for playback |
| Show Mode (locked fullscreen) | Polish feature; deferred to after core is stable |
| Portable DMX Node (RPi/NUC) | Hardware product; requires dedicated V2 effort |

---

### MVP Success Criteria

The MVP is validated when:

- A creative technologist records a complete show and deploys it on a separate
  machine with zero source software present
- The system auto-starts on boot and begins playback without human interaction
- Scheduling runs a scene autonomously within a defined time window for multiple
  consecutive days without failure
- A non-technical venue operator switches scenes using the cue pad interface
  without any instructions from the designer

---

### Future Vision

**V2 — Live Control Layer**
- Web Remote: local-network smartphone controller with scene pads, master fader,
  and blackout panic button — the LD's pocket controller during a show
- Timeline / DAW view: sequence scenes on a time axis for complex choreography
- Show Mode: locked fullscreen performance state optimized for dark environments

**V3 — Creative Power**
- Scene Layering Engine: stack multiple recorded scenes as layers with blend modes
  (HTP, LTP, additive), per-universe masking, and independent speed/direction —
  the Photoshop layer paradigm applied to DMX
- Channel Sculptor: post-capture curve editing for individual channels

**V4 — Hardware Product**
- Portable DMX Node: headless deployment on Raspberry Pi or mini NUC
- Powers on, auto-loads last show, runs forever with no monitor required
- Controlled entirely via Web Remote
- Transforms the software into an embeddable hardware product for venues,
  touring rigs, and permanent installations
