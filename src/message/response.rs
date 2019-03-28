use std::error;
use std::fmt::{self, Display};

#[derive(Deserialize, Debug, Serialize)]
pub struct GcmResponse {
  pub message_id: Option<u64>,
  pub error: Option<String>,
  pub multicast_id: Option<i64>,
  pub success: Option<u64>,
  pub failure: Option<u64>,
  pub canonical_ids: Option<u64>,
  pub results: Option<Vec<MessageResult>>
}

impl GcmResponse {
  pub fn default() -> GcmResponse {
    GcmResponse {
      message_id: None,
      error: None,
      multicast_id: None,
      success: None,
      failure: None,
      canonical_ids: None,
      results: None,
    }
  }
}


#[derive(Deserialize, Debug, Serialize)]
pub struct MessageResult {
  pub message_id: Option<String>,
  pub registration_id: Option<u64>,
  pub error: Option<String>
}


#[derive(PartialEq, Debug)]
pub enum GcmError {
  Unauthorized,
  InvalidMessage(String),
  ServerError,
  InvalidJsonBody
}

impl Display for GcmError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      GcmError::Unauthorized => write!(f, "UnauthorizedError"),
      GcmError::ServerError => write!(f, "ServerError"),
      GcmError::InvalidMessage(ref message) => write!(f, "InvalidMessage: {}", message),
      GcmError::InvalidJsonBody => write!(f, "InvalidJsonBody")
    }
  }
}

impl error::Error for GcmError {
  fn description(&self) -> &str {
    match *self {
      GcmError::Unauthorized => "UnauthorizedError",
      GcmError::ServerError => "ServerError",
      GcmError::InvalidMessage(_) => "InvalidMessage",
      GcmError::InvalidJsonBody => "InvalidJsonBody"
    }
  }
}
