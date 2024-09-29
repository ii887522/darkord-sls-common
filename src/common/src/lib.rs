#![deny(elided_lifetimes_in_paths)]

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

use anyhow::{Context as _, Result};
use lambda_runtime::Context;
use optarg2chain::optarg_fn;
use scrypt::{
    password_hash::{
        rand_core::{OsRng, RngCore},
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Params, Scrypt,
};
use std::{
    hash::{DefaultHasher, Hash, Hasher},
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
    #[optarg_default] years: i64,
    #[optarg_default] months: i64,
    #[optarg_default] days: i64,
    #[optarg_default] hours: i64,
    #[optarg_default] minutes: i64,
    #[optarg_default] seconds: i64,
    #[optarg_default] milliseconds: i64,
    #[optarg_default] unit: Unit,
) -> Result<u64> {
    // Convert and do everything in milliseconds to make logic simpler
    let src_timestamp_ms = match unit {
        Unit::Seconds => src_timestamp * 1000, // src_timestamp is in seconds, need to convert to milliseconds
        Unit::Milliseconds => src_timestamp,
    };

    let src_timestamp_ms = if src_timestamp_ms == 0 {
        get_current_timestamp()
            .unit(Unit::Milliseconds)
            .call()
            .context(Location::caller())?
    } else {
        src_timestamp_ms
    };

    let years_ms = years * 31_536_000_000;
    let months_ms = months * 2_592_000_000;
    let days_ms = days * 86_400_000;
    let hours_ms = hours * 3_600_000;
    let minutes_ms = minutes * 60_000;
    let seconds_ms = seconds * 1_000;
    let milliseconds_ms = milliseconds;

    let result_ms = src_timestamp_ms;

    let result_ms = if years_ms >= 0 {
        result_ms + years_ms as u64
    } else {
        result_ms - (-years_ms) as u64
    };

    let result_ms = if months_ms >= 0 {
        result_ms + months_ms as u64
    } else {
        result_ms - (-months_ms) as u64
    };

    let result_ms = if days_ms >= 0 {
        result_ms + days_ms as u64
    } else {
        result_ms - (-days_ms) as u64
    };

    let result_ms = if hours_ms >= 0 {
        result_ms + hours_ms as u64
    } else {
        result_ms - (-hours_ms) as u64
    };

    let result_ms = if minutes_ms >= 0 {
        result_ms + minutes_ms as u64
    } else {
        result_ms - (-minutes_ms) as u64
    };

    let result_ms = if seconds_ms >= 0 {
        result_ms + seconds_ms as u64
    } else {
        result_ms - (-seconds_ms) as u64
    };

    let result_ms = if milliseconds_ms >= 0 {
        result_ms + milliseconds_ms as u64
    } else {
        result_ms - (-milliseconds_ms) as u64
    };

    let result = match unit {
        Unit::Seconds => result_ms / 1000,
        Unit::Milliseconds => result_ms,
    };

    Ok(result)
}

pub fn hash_secret(secret: &str) -> String {
    Scrypt
        .hash_password_customized(
            secret.as_bytes(),
            None,
            None,
            Params::new(15, 8, 1, 32).unwrap(),
            &SaltString::generate(&mut OsRng),
        )
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

pub fn hash(value: &impl Hash) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

#[optarg_fn(IsAlmostTimeoutBuilder, call)]
pub fn is_almost_timeout<'a>(
    context: &'a Context,
    #[optarg(1)] seconds_before_timeout: i64,
) -> Result<bool> {
    let now = get_current_timestamp()
        .unit(Unit::Milliseconds)
        .call()
        .context(Location::caller())?;

    let almost_deadline = extend_current_timestamp()
        .src_timestamp(context.deadline)
        .unit(Unit::Milliseconds)
        .seconds(-seconds_before_timeout)
        .call()
        .context(Location::caller())?;

    Ok(now >= almost_deadline)
}
