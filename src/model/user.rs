use serde::{de::value::UsizeDeserializer, Deserialize, Serialize};
use validator::{Validate, ValidationError};

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


#[derive(Deserialize)]
pub struct LoginUserSchema{
    pub user:String,
    pub pwd: String,
}
#[derive(Serialize,)]
pub struct JWTToken{
    pub sub:String,
    pub exp:usize,
    pub iat: usize,
    pub payload:Option<String>
}
// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
}
#[cfg(test)]
mod tests {
    use uuid::Uuid;

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
        let uuid = uuid::Uuid::new_v3(&Uuid::NAMESPACE_URL, b"jj");
        println!("{uuid}");
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
