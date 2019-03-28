use std::io::Read;

use hyper::client::Response;
use hyper::header;
use hyper::header::Headers;
use hyper::mime::{Attr, Mime, SubLevel, TopLevel, Value};
use hyper::net::HttpsConnector;
use hyper::status::{StatusClass, StatusCode};
use hyper::Client;
use hyper_native_tls::NativeTlsClient;

use gcm_util;
use message::response::{GcmError, GcmResponse};
use message::Message;

type FcmResult = Result<GcmResponse, GcmError>;

#[allow(dead_code)]
pub struct FcmSender {
    google_api: String,
    api_key: String,
    client: Client,
    headers: Headers,
}

impl FcmSender {
    pub fn new(google_api: String, api_key: String) -> FcmSender {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);

        let mut headers = Headers::new();
        let mime = Mime(
            TopLevel::Application,
            SubLevel::Json,
            vec![(Attr::Charset, Value::Utf8)],
        );

        headers.set(header::Authorization("key=".to_string() + &api_key));
        headers.set(header::ContentType(mime));

        FcmSender {
            google_api,
            api_key,
            client,
            headers,
        }
    }

    // Todo : Have to add retry logic here
    pub fn send(&self, msg: Message) -> FcmResult {
        let parsed_msg = gcm_util::to_json(&msg)?;
        let mut result = self.post(&parsed_msg)?;
        self.parse_response(&mut result)
    }

    fn parse_response(&self, response: &mut Response) -> FcmResult {
        let mut body = String::new();
        let resp_code = response.status;

        let response_result = response.read_to_string(&mut body);
        match response_result {
            Ok(_) => self.parse_gcm_result(resp_code, &body),
            Err(_) => self.parse_gcm_result(StatusCode::InternalServerError, "Server Error"),
        }
    }

    fn post(&self, json_request: &String) -> Result<Response, GcmError> {
        let response = self
            .client
            .post(&self.google_api)
            .body(json_request.as_bytes())
            .headers(self.headers.clone())
            .send();

        match response {
            Ok(response) => Ok(response),
            Err(_) => Err(GcmError::ServerError),
        }
    }

    fn parse_gcm_result(&self, status: StatusCode, body: &str) -> FcmResult {
        //200 Ok: Request was successful!
        if status == StatusCode::Ok {
            return serde_json::from_str(body).or_else(|_| Err(GcmError::InvalidJsonBody));
        }

        //check for server error (5xx)
        if status.class() == StatusClass::ServerError {
            return Err(GcmError::ServerError);
        }
        //match remaining status codes
        match status {
            StatusCode::Unauthorized => Err(GcmError::Unauthorized),
            StatusCode::BadRequest => Err(GcmError::InvalidMessage(body.to_string())),
            _ => Err(GcmError::InvalidMessage("Unknown Error".to_string())),
        }
    }
}
