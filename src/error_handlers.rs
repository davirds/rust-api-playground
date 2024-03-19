use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use serde::Deserialize;
use serde_json::json;
use std::fmt;
use std::fmt::{Debug, Formatter};

#[derive(Debug, Deserialize)]
pub struct CustomError {
    code: u16,
    message: String,
}

impl CustomError {
    pub fn new(code: u16, message: String) -> Self {
        Self { code, message }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl From<DieselError> for CustomError {
    fn from(error: DieselError) -> CustomError {
        match error {
            DieselError::DatabaseError(_, err) => CustomError::new(409, err.message().to_string()),
            DieselError::NotFound => {
                CustomError::new(404, "The employee record not found".to_string())
            }
            err => CustomError::new(500, format!("Unknown Diesel error: {}", err)),
        }
    }
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        let status_code =
            StatusCode::from_u16(self.code).unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR);

        let error_message = match status_code.as_u16() < 500 {
            true => self.message.clone(),
            false => "Internal server error".to_string(),
        };

        HttpResponse::build(status_code).json(json!({ "message": error_message }))
    }
}
