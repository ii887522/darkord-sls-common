use crate::ApiResponse;
use aws_lambda_events::http::HeaderMap;
use serde_json::json;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommonError<'a> {
    pub code: u32,
    pub message: &'a str,
}

impl<'a> CommonError<'a> {
    pub fn into_api_resp(self, request_id: &str) -> ApiResponse<'a, '_> {
        ApiResponse {
            code: self.code,
            headers: HeaderMap::new(),
            message: self.message,
            payload: json!({}),
            request_id,
        }
    }
}

impl Display for CommonError<'_> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for CommonError<'_> {}
