use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use either::Either::{self, Left, Right};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use validator::{Validate, ValidationError, ValidationErrors};

// the input to our `create_user` handler
#[derive(Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 5, max = 15), custom = "validate_username_is_not_blank")]
    pub username: String,
    #[validate(range(min = 0, max = 120))]
    age: Option<u8>,
}

fn validate_username_is_not_blank(username: &str) -> Result<(), ValidationError> {
    if username.trim().is_empty() {
        Err(ValidationError::new("it's all space, not allowed"))
    } else {
        Ok(())
    }
}
// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
}

pub struct ValidatedJson<T>(pub Json<T>);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = MyValidationErrors;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let bytes: Result<Json<T>, JsonRejection> = Json::from_request(req, state).await;
        match bytes {
            Err(e) => Err(MyValidationErrors(Left(e))),
            Ok(data) => match data.validate() {
                Ok(_) => Ok(ValidatedJson(data)),
                Err(validationerror) => Err(MyValidationErrors(Right(validationerror))),
            },
        }
    }
}

pub struct MyValidationErrors(Either<JsonRejection, ValidationErrors>);

impl IntoResponse for MyValidationErrors {
    fn into_response(self) -> Response {
        match self.0 {
            Left(jsonerror) => jsonerror.into_response(),
            Right(validatorerros) => {
                (StatusCode::UNPROCESSABLE_ENTITY, validatorerros.to_string()).into_response()
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_basic_ee() {
        let user = User {
            id: 3333,
            username: String::from("John"),
        };
        assert_eq!(
            serde_json::to_string(&user).unwrap(),
            r#"{"id":3333,"username":"John"}"#.to_owned()
        );
    }
    #[test]
    fn test_de_validation() {
        let input_too_short = r#"{"username":"S"}"#;
        let input_correct = r#"{"username":"Correct Name"}"#;
        let input_too_long = r#"{"username":"It's a very long name which exceed the expectation"}"#;
        let create_user: CreateUser = serde_json::from_str(input_too_short).unwrap();
        assert!(create_user.validate().is_err());
        let create_user: CreateUser = serde_json::from_str(input_correct).unwrap();
        assert!(create_user.validate().is_ok());
        let create_user: CreateUser = serde_json::from_str(input_too_long).unwrap();
        assert!(create_user.validate().is_err());
    }
}
