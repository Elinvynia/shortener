use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct User {
    pub logged_in: bool,
    pub id: u64,
    pub username: String,
}
