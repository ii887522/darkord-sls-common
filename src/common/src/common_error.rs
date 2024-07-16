use crate::ApiResponse;
use aws_lambda_events::http::HeaderMap;
use serde_json::json;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommonError {
    pub code: u32,
    pub message: String,
}

impl CommonError {
    pub fn into_api_resp(self, request_id: &str) -> ApiResponse<'_> {
        ApiResponse {
            code: self.code,
            headers: HeaderMap::new(),
            message: self.message,
            payload: json!({}),
            request_id,
        }
    }
}

impl Display for CommonError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for CommonError {}
