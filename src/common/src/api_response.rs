use crate::constants;
use aws_lambda_events::{
    apigw::ApiGatewayProxyResponse,
    http::{header::CONTENT_TYPE, HeaderMap, HeaderValue},
};
use serde::Serialize;
use serde_json::{json, Value};
use std::mem;

#[derive(Debug, PartialEq, Serialize)]
pub struct ApiResponse<'a, 'b> {
    pub code: u32,

    #[serde(skip)]
    pub headers: HeaderMap<HeaderValue>,

    pub message: &'a str,
    pub payload: Value,
    pub request_id: &'b str,
}

impl Default for ApiResponse<'_, '_> {
    fn default() -> Self {
        Self {
            code: 2000,
            headers: HeaderMap::new(),
            message: "",
            payload: json!({}),
            request_id: "",
        }
    }
}

impl From<ApiResponse<'_, '_>> for ApiGatewayProxyResponse {
    fn from(mut api_resp: ApiResponse<'_, '_>) -> Self {
        let status_code = api_resp.code.to_string()[..3].parse().unwrap();

        if api_resp.message.is_empty() {
            api_resp.message = constants::API_ERR_MSG_MAP.get(&status_code).unwrap_or(&"");
        }

        let headers = HeaderMap::from_iter(
            [(CONTENT_TYPE, HeaderValue::from_static("application/json"))]
                .into_iter()
                .chain(
                    mem::take(&mut api_resp.headers)
                        .into_iter()
                        .map(|(k, v)| (k.unwrap(), v)),
                ),
        );

        let body = serde_json::to_string(&api_resp).unwrap().into();

        ApiGatewayProxyResponse {
            status_code,
            headers,
            body: Some(body),
            ..Default::default()
        }
    }
}
