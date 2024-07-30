use anyhow::Result;
use aws_lambda_events::apigw::ApiGatewayProxyRequest;
use serde::de::DeserializeOwned;
use serde_json::{json, Map, Value};
use validator::Validate;

pub trait Request
where
    Self: DeserializeOwned + Validate,
{
    fn load(event: &ApiGatewayProxyRequest) -> Result<Self> {
        let mut event_body = serde_json::from_str::<Map<String, Value>>(
            event.body.as_ref().unwrap_or(&"{}".to_string()),
        )?;

        for (k, v) in &event.path_parameters {
            event_body.insert(k.to_string(), json!(v));
        }

        for (k, v) in event.query_string_parameters.iter() {
            event_body.insert(k.to_string(), json!(v));
        }

        let req = serde_json::from_value::<Self>(json!(event_body))?;
        req.validate()?;
        Ok(req)
    }
}
