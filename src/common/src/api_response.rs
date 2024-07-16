use crate::constants;
use aws_lambda_events::{
    apigw::ApiGatewayProxyResponse,
    http::{header::CONTENT_TYPE, HeaderMap, HeaderValue},
};
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Debug, PartialEq, Serialize)]
pub struct ApiResponse<'a> {
    pub code: u32,

    #[serde(skip)]
    pub headers: HeaderMap<HeaderValue>,

    pub message: String,
    pub payload: Value,
    pub request_id: &'a str,
}

impl Default for ApiResponse<'_> {
    fn default() -> Self {
        Self {
            code: 2000,
            headers: HeaderMap::new(),
            message: "".to_string(),
            payload: json!({}),
            request_id: "",
        }
    }
}

impl From<ApiResponse<'_>> for ApiGatewayProxyResponse {
    fn from(mut api_resp: ApiResponse<'_>) -> Self {
        let status_code = api_resp.code.to_string()[..3].parse().unwrap();

        if api_resp.message.is_empty() {
            api_resp.message = constants::API_ERR_MSG_MAP
                .get(&status_code)
                .unwrap_or(&"")
                .to_string();
        }

        let body = serde_json::to_string(&api_resp).unwrap().into();

        let headers = HeaderMap::from_iter(
            [(CONTENT_TYPE, HeaderValue::from_static("application/json"))]
                .into_iter()
                .chain(api_resp.headers.into_iter().map(|(k, v)| (k.unwrap(), v))),
        );

        ApiGatewayProxyResponse {
            status_code,
            headers,
            body: Some(body),
            ..Default::default()
        }
    }
}
