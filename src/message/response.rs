use std::collections::{HashMap, HashSet};
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
    pub results: Option<Vec<MessageResult>>,
    pub ids_by_error: Option<HashMap<String, Vec<String>>>,
    pub ids_by_success : HashSet<String>,
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
            ids_by_error: None,
            ids_by_success: HashSet::new()
        }
    }

    pub fn build_reg_ids_by_error_map(&mut self, ids: Vec<String>) {
        if self.failure.is_none() || self.results.is_none() {
            return;
        }

        let message_results = self.results.as_ref().unwrap();
        let mut ids_by_error: HashMap<String, Vec<String>> = HashMap::new();

        for (i, v) in message_results.iter().enumerate() {
            let id = ids.get(i).unwrap();
            match v.error {
                None => {
                    self.ids_by_success.insert(id.clone());
                },
                Some(ref err_name) => {

                    ids_by_error
                        .entry(err_name.to_string())
                        .and_modify(|v| v.push(id.to_string()))
                        .or_insert_with(|| vec![id.to_string()]);
                }
            }
        }

        self.ids_by_error = Some(ids_by_error);
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct MessageResult {
    pub message_id: Option<String>,
    pub registration_id: Option<u64>,
    pub error: Option<String>,
}

#[derive(PartialEq, Debug)]
pub enum GcmError {
    Unauthorized,
    InvalidMessage(String),
    ServerError,
    InvalidJsonBody,
}

impl Display for GcmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GcmError::Unauthorized => write!(f, "UnauthorizedError"),
            GcmError::ServerError => write!(f, "ServerError"),
            GcmError::InvalidMessage(ref message) => write!(f, "InvalidMessage: {}", message),
            GcmError::InvalidJsonBody => write!(f, "InvalidJsonBody"),
        }
    }
}

impl error::Error for GcmError {
    fn description(&self) -> &str {
        match *self {
            GcmError::Unauthorized => "UnauthorizedError",
            GcmError::ServerError => "ServerError",
            GcmError::InvalidMessage(_) => "InvalidMessage",
            GcmError::InvalidJsonBody => "InvalidJsonBody",
        }
    }
}
