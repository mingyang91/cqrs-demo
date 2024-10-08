use async_trait::async_trait;
use axum::body::{Body, Bytes};
use axum::extract::FromRequest;
use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Response};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

// This is a custom Axum extension that builds metadata from the inbound request
// and parses and deserializes the body as the command payload.
pub struct CommandExtractor<T>(pub HashMap<String, String>, pub T);

const USER_AGENT_HDR: &str = "User-Agent";

#[async_trait]
impl<S, T> FromRequest<S> for CommandExtractor<T>
where
    S: Send + Sync,
    T: DeserializeOwned
{
    type Rejection = CommandExtractionError;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        // Here we are including the current date/time, the uri that was called and the user-agent
        // in a HashMap that we will submit as metadata with the command.
        let mut metadata = HashMap::default();
        metadata.insert("time".to_string(), chrono::Utc::now().to_rfc3339());
        metadata.insert("uri".to_string(), req.uri().to_string());
        if let Some(user_agent) = req.headers().get(USER_AGENT_HDR) {
            if let Ok(value) = user_agent.to_str() {
                metadata.insert(USER_AGENT_HDR.to_string(), value.to_string());
            }
        }

        // Parse and deserialize the request body as the command payload.
        let body = Bytes::from_request(req, state).await?;
        let command: T = serde_json::from_slice(body.as_ref())?;
        Ok(CommandExtractor(metadata, command))
    }
}

pub struct CommandExtractionError;

impl IntoResponse for CommandExtractionError {
    fn into_response(self) -> Response {
        (
            StatusCode::BAD_REQUEST,
            "command could not be read".to_string(),
        )
            .into_response()
    }
}

impl From<axum::extract::rejection::BytesRejection> for CommandExtractionError {
    fn from(_: axum::extract::rejection::BytesRejection) -> Self {
        CommandExtractionError
    }
}

impl From<serde_json::Error> for CommandExtractionError {
    fn from(_: serde_json::Error) -> Self {
        CommandExtractionError
    }
}
