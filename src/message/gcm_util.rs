use http::StatusCode as HttpStatusCode;
use hyper::status::StatusClass;
use hyper::status::StatusCode;

use message::Message;
use message::response::GcmError;

pub fn to_json(msg: &Message) -> Result<String, GcmError> {
    match serde_json::to_string(msg) {
        Ok(parsed_json) => Ok(parsed_json),
        Err(_) => Err(GcmError::InvalidJsonBody)
    }
}

// Todo : documentation
pub fn parse_error_status_code(http_status_option: Option<HttpStatusCode>) -> GcmError {
    match http_status_option {
        None => GcmError::ServerError,
        Some(http_status) => parse_error_status(http_status)
    }
}

fn parse_error_status(http_status: HttpStatusCode) -> GcmError {
    let hyper_status_code = StatusCode::from_u16(http_status.as_u16());

    //check for server error (5xx)
    if hyper_status_code.class() == StatusClass::ServerError {
        return GcmError::ServerError;
    }
    //match remaining status codes
    match hyper_status_code {
        StatusCode::Unauthorized => GcmError::Unauthorized,
        StatusCode::BadRequest => GcmError::InvalidMessage(http_status.to_string()),
        _ => GcmError::InvalidMessage("Unknown Error".to_string())
    }
}


