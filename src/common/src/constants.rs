use once_cell::sync::Lazy;
use std::{collections::HashMap, env};

pub const SENSITIVE_KEYS: &[&str] = &[
    "postman-token",
    "x-api-key",
    "password",
    "session_token",
    "code",
    "refresh_token",
    "access_token",
    "jti",
    "verification_code",
    "authorization_token",
    "Authorization",
];

// Environment variables
pub static LOG_LEVEL: Lazy<String> = Lazy::new(|| env::var("LOG_LEVEL").unwrap());
pub static REGION: Lazy<String> = Lazy::new(|| env::var("REGION").unwrap());
pub static STAGE: Lazy<String> = Lazy::new(|| env::var("STAGE").unwrap());
pub static STAGE_PREFIX: Lazy<String> = Lazy::new(|| env::var("STAGE_PREFIX").unwrap());
pub static STAGE_DASH_PREFIX: Lazy<String> = Lazy::new(|| env::var("STAGE_DASH_PREFIX").unwrap());

// API response error messages
pub static API_ERR_MSG_MAP: Lazy<HashMap<i64, &str>> = Lazy::new(|| {
    HashMap::from_iter([
        (400, "Bad request"),
        (401, "Unauthorized"),
        (403, "Forbidden"),
        (404, "Data was not found"),
        (409, "Conflict"),
        (500, "Internal server error"),
    ])
});
