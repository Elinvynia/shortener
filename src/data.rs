use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct User {
    pub logged_in: bool,
    pub id: u64,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Url {
    pub id: String,
    pub destination: String,
    pub created_at: String,
    pub delete_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterData {
    pub username: String,
    pub password: String,
    pub password_confirm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlData {
    pub url: String,
}
