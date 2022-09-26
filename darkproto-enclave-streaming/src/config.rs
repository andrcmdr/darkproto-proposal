use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub in_file: String,
    pub out_file: String,
    pub init_enc_pwd_file: String,
    pub re_enc_pwd_file: String,
    pub init_random_seed_file: String,
    pub re_enc_random_seed_file: String,
}
