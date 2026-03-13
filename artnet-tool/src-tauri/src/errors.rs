// errors.rs — UserMessage trait and per-subsystem error enums
// All command handlers return Result<T, String> by calling .map_err(|e| e.to_user_message())
// This ensures users NEVER see raw Rust errors, stack traces, or error codes.

/// Converts a domain error into a plain-language user-facing message.
/// Every error type used at the Tauri command boundary must implement this.
pub trait UserMessage {
    fn to_user_message(&self) -> String;
}
