// subsystems/boot/ — OS auto-start and crash recovery (watchdog) registration
// Platform dispatch: windows.rs / macos.rs / linux.rs
// Windows: windows-service crate or registry Run key
// macOS: LaunchAgent plist
// Linux: systemd user service unit
// Populated in Story 6.1 (OS Auto-Start) and Story 6.2 (Crash Recovery)
