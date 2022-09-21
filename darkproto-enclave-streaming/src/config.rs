use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub file_for_handling: std::path::PathBuf,
    pub pwd_file: std::path::PathBuf,
    pub initial_random_seed_file: std::path::PathBuf,
}
