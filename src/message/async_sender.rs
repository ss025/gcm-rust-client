extern crate http;
extern crate reqwest;

use futures::future::{err, ok};
use futures::Future;
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use http::HeaderMap;
use reqwest::async::{Client, ClientBuilder, Response};

use gcm_util;
use message::Message;
use message::response::{GcmError, GcmResponse};

pub type GcmResponseFuture = Box<Future<Item=GcmResponse, Error=GcmError> + Send>;

pub struct AsyncFsmSender {
    client: Client,
    fcm_url: String,
    ids_by_error: bool,
}

impl AsyncFsmSender {

    /// Create new Async FCM/GCM Sender
    ///
    /// api_key  => api key given gcm or fcm
    ///
    /// fcm_url  => gcm/fcm api e.g https://fcm.googleapis.com/fcm/send
    ///
    /// ids_by_error => flag to build map of <error,vec<registration_ids>> in GCM Response . If this flag is false , no
    /// map will be prepared.
    pub fn new(api_key: String, fcm_url: String,ids_by_error :bool) -> AsyncFsmSender {
        // set up headers
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            ("key=".to_string() + &api_key).parse().unwrap(),
        );
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .expect("new async client");
        AsyncFsmSender { client, fcm_url ,ids_by_error}
    }

    pub fn send(&self, msg: Message) -> GcmResponseFuture {
        let result = gcm_util::to_json(&msg);
        let reg_ids = msg.registration_ids;
        let should_build_error_map = self.ids_by_error;

        match result {
            Err(e) => Box::new(err(e)),
            Ok(body) => {
                let and_then = self
                    .client
                    .post(&self.fcm_url)
                    .body(body)
                    .send()
                    .map_err(|err| gcm_util::parse_error_status_code(err.status()))
                    .and_then(move |res| AsyncFsmSender::parse(res, reg_ids.unwrap(),should_build_error_map));
                Box::new(and_then)
            }
        }
    }

    fn parse(mut res: Response, ids: Vec<String>,should_build_error_map : bool) -> GcmResponseFuture {
        let status_code = res.status().as_u16();

        match status_code {
            200 => {
                let then = res
                    .json::<GcmResponse>()
                    .map_err(|_| GcmError::InvalidJsonBody)
                    .and_then(move |mut gcm_resp| {
                        if should_build_error_map && gcm_resp.results.is_some() {
                            gcm_resp.build_reg_ids_by_error_map(ids);
                        }
                        ok(gcm_resp)
                    });

                Box::new(then)
            }
            _ => Box::new(err(gcm_util::parse_error_status_code(Some(res.status())))),
        }
    }


}
