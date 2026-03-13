// subsystems/playback/ — Frame-accurate ArtNet/sACN playback engine
// Isolated OS thread with spin_sleep for ±1ms timing (NFR1, NFR5)
// Receives commands via Tokio mpsc channel — never via shared mutable state
// Populated in Story 1.5 (thread isolation scaffold) and Story 3.2 (full engine)
