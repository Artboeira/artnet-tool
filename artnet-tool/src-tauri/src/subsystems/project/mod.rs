// subsystems/project/ — Project file load/save/export/import/migrate
// All JSON fields snake_case via #[serde(rename_all = "snake_case")]
// schema_version: u32 for forward-compatible migration chain
// #[serde(deny_unknown_fields)] for safe parsing of untrusted files (NFR21)
// Populated in Story 5.1-5.3
