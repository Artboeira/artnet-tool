// subsystems/monitor/ — DMX output/input sampling for real-time UI display
// Tokio task; emits monitor-update events to frontend at ~30fps
// During capture: shows incoming signal; during playback: shows outgoing signal
// Populated in Story 2.3 (Real-Time DMX Monitor)
