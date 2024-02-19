pub mod user;
use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use either::Either::{self, Left, Right};
use serde::de::DeserializeOwned;
use validator::{Validate, ValidationErrors};

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
        let data = bytes.map_err(|e| MyValidationErrors(Left(e)))?;
        data.validate().map_err(|e| MyValidationErrors(Right(e)))?;
        Ok(ValidatedJson(data))
        // match bytes {
        //     Err(e) => Err(MyValidationErrors(Left(e))),
        //     Ok(data) => match data.validate() {
        //         Ok(_) => Ok(ValidatedJson(data)),
        //         Err(validationerror) => Err(MyValidationErrors(Right(validationerror))),
        //     },
        // }
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
