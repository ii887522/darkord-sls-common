use crate::{constants, SensitiveData};
use aws_lambda_events::apigw::{
    ApiGatewayCustomAuthorizerRequest, ApiGatewayCustomAuthorizerRequestTypeRequest,
    ApiGatewayProxyRequest,
};
use lambda_runtime::tracing::{
    info,
    subscriber::{self, EnvFilter},
};
use serde_json::{Error, Map, Value};
use std::mem;

pub fn init() {
    subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .without_time()
        .init();
}

pub trait Logger {
    fn log(&mut self) -> Result<(), Error>;
}

impl Logger for ApiGatewayProxyRequest {
    fn log(&mut self) -> Result<(), Error> {
        // Hide sensitive input, assume caller no longer needs these keys for their logic
        for &sensitive_key in constants::SENSITIVE_KEYS {
            self.headers.remove(sensitive_key);
            self.multi_value_headers.remove(sensitive_key);
            self.path_parameters.remove(sensitive_key);
        }
        self.request_context.identity.api_key = None;
        self.request_context.identity.api_key_id = None;
        self.request_context.identity.access_key = None;

        let event_body = serde_json::from_str::<Map<String, Value>>(
            self.body.as_ref().unwrap_or(&"{}".to_string()),
        )?;

        // Hide sensitive input, but later need to reveal back for caller logic
        let mut sensitive_event_body = SensitiveData::new(event_body).call();
        sensitive_event_body.hide();
        self.body = Some(serde_json::to_string(sensitive_event_body.get())?);

        // Log the event
        info!(event = serde_json::to_string(self)?);

        // Reveal back the sensitive input for caller logic
        sensitive_event_body.show();
        self.body = Some(serde_json::to_string(sensitive_event_body.get())?);

        Ok(())
    }
}

impl Logger for ApiGatewayCustomAuthorizerRequest {
    fn log(&mut self) -> Result<(), Error> {
        // Hide sensitive input, but later need to reveal back for caller logic
        let auth_token = self.authorization_token.take();

        // Log the event
        info!(event = serde_json::to_string(self)?);

        // Reveal back the sensitive input for caller logic
        self.authorization_token = auth_token;

        Ok(())
    }
}

impl Logger for ApiGatewayCustomAuthorizerRequestTypeRequest {
    fn log(&mut self) -> Result<(), Error> {
        // Hide sensitive input, assume caller no longer needs these keys for their logic
        for &sensitive_key in constants::SENSITIVE_KEYS {
            self.headers.remove(sensitive_key);
            self.multi_value_headers.remove(sensitive_key);
            self.path_parameters.remove(sensitive_key);
        }
        if let Some(identity) = &mut self.request_context.identity {
            identity.api_key = None;
            identity.api_key_id = None;
        }

        // Hide sensitive input, but later need to reveal back for caller logic
        let auth_token = self.headers.remove("Authorization");

        // Log the event
        info!(event = serde_json::to_string(self)?);

        // Reveal back the sensitive input for caller logic
        if let Some(auth_token) = auth_token {
            self.headers.append("Authorization", auth_token);
        }

        Ok(())
    }
}

impl Logger for Value {
    fn log(&mut self) -> Result<(), Error> {
        // Hide sensitive input, but later need to reveal back for caller logic
        let mut sensitive_event_body =
            SensitiveData::new(mem::take(self.as_object_mut().unwrap_or(&mut Map::new()))).call();

        sensitive_event_body.hide();

        // Log the event
        info!(event = serde_json::to_string(sensitive_event_body.get())?);

        // Reveal back the sensitive input for caller logic
        sensitive_event_body.show();
        *self = Value::Object(sensitive_event_body.into_data());

        Ok(())
    }
}
