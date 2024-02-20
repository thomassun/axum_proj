use axum::{
    http::{header, HeaderValue, StatusCode},
    response::IntoResponse,
};
use serde_json::Value;

#[derive(Debug)]
pub enum MessageBody {
    String(String),
    JSON(Value),
}
pub enum AppError {
    Anyhow(anyhow::Error),
    Unauthorized(MessageBody),
    None,
}
impl From<Value> for MessageBody {
    fn from(value: Value) -> Self {
        MessageBody::JSON(value)
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(value: E) -> Self {
        AppError::Anyhow(value.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Anyhow(anyhow) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("unexpected error:{anyhow}"),
            )
                .into_response(),
            Self::Unauthorized(MessageBody::JSON(json)) => (
                StatusCode::UNAUTHORIZED,
                [(
                    header::CONTENT_TYPE,
                    "application/json".parse::<HeaderValue>().unwrap(),
                )],
                json.to_string(),
            )
                .into_response(),
            Self::Unauthorized(MessageBody::String(msg)) => {
                (StatusCode::UNAUTHORIZED, msg).into_response()
            }
            Self::None => (StatusCode::NO_CONTENT).into_response(),
        }
    }
}

impl From<&str> for MessageBody {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}
