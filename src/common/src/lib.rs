pub mod api_response;
pub mod common_enums;
pub mod common_error;
pub mod common_serde;
pub mod common_tracing;
pub mod constants;
pub mod method_arn;
pub mod sensitive_data;
pub mod trimmed_string;

pub use api_response::ApiResponse;
pub use common_error::CommonError;
pub use method_arn::MethodArn;
pub use sensitive_data::SensitiveData;
pub use sensitive_data::SensitiveDataNewBuilder;
pub use trimmed_string::TrimmedString;

use anyhow::{Context, Result};
use optarg2chain::optarg_fn;
use scrypt::{
    password_hash::{
        rand_core::{OsRng, RngCore},
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Scrypt,
};
use std::{
    panic::Location,
    time::{SystemTime, UNIX_EPOCH},
};

pub trait Case {
    fn convert_snake_case_to_pascal_case(&self) -> String;
}

impl Case for str {
    fn convert_snake_case_to_pascal_case(&self) -> String {
        self.split('_')
            .map(|word| {
                word.chars()
                    .nth(0)
                    .unwrap_or_default()
                    .to_uppercase()
                    .to_string()
                    + &word.chars().skip(1).collect::<String>()
            })
            .collect()
    }
}

pub trait StringExt {
    fn remove_first_and_last_chars(&self) -> &str;
}

impl StringExt for str {
    fn remove_first_and_last_chars(&self) -> &str {
        let mut chars = self.chars();
        chars.next();
        chars.next_back();
        chars.as_str()
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Unit {
    #[default]
    Seconds,
    Milliseconds,
}

#[optarg_fn(GetCurrentTimestampBuilder, call)]
pub fn get_current_timestamp(#[optarg_default] unit: Unit) -> Result<u64> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context(Location::caller())?;

    match unit {
        Unit::Seconds => Ok(now.as_secs()),
        Unit::Milliseconds => Ok(now.as_millis().try_into().context(Location::caller())?),
    }
}

#[optarg_fn(ExtendCurrentTimestampBuilder, call)]
pub fn extend_current_timestamp(
    #[optarg_default] src_timestamp: u64,
    #[optarg_default] years: u64,
    #[optarg_default] months: u64,
    #[optarg_default] days: u64,
    #[optarg_default] hours: u64,
    #[optarg_default] minutes: u64,
    #[optarg_default] seconds: u64,
    #[optarg_default] milliseconds: u64,
    #[optarg_default] unit: Unit,
) -> Result<u64> {
    // Convert and do everything in milliseconds to make logic simpler

    let src_timestamp = match unit {
        Unit::Seconds => src_timestamp * 1000, // src_timestamp is in seconds, need to convert to milliseconds
        Unit::Milliseconds => src_timestamp,
    };

    let src_timestamp = if src_timestamp == 0 {
        get_current_timestamp()
            .unit(Unit::Milliseconds)
            .call()
            .context(Location::caller())?
    } else {
        src_timestamp
    };

    let later = src_timestamp
        + years * 31_536_000_000
        + months * 2_592_000_000
        + days * 86_400_000
        + hours * 3_600_000
        + minutes * 60_000
        + seconds * 1_000
        + milliseconds;

    let later = match unit {
        Unit::Seconds => later / 1000,
        Unit::Milliseconds => later,
    };

    Ok(later)
}

pub fn hash_secret(secret: &str) -> String {
    Scrypt
        .hash_password(secret.as_bytes(), &SaltString::generate(&mut OsRng))
        .unwrap()
        .to_string()
}

pub fn verify_secret(secret: &str, hashed_secret: &str) -> bool {
    Scrypt
        .verify_password(
            secret.as_bytes(),
            &PasswordHash::new(hashed_secret).unwrap(),
        )
        .is_ok()
}

#[optarg_fn(GenSecretDigitsBuilder, call)]
pub fn gen_secret_digits(#[optarg(6)] digit_count: u32) -> String {
    format!(
        "{:0width$}",
        OsRng.next_u32() % 10i32.pow(digit_count) as u32,
        width = digit_count as usize
    )
}
