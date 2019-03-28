extern crate http;
extern crate reqwest;

use futures::future::{err, ok};
use futures::Future;
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use http::HeaderMap;
use reqwest::async::{Client, ClientBuilder, Response};

use gcm_util;
use message::response::{GcmError, GcmResponse};
use message::Message;

pub type GcmResponseFuture = Box<Future<Item = GcmResponse, Error = GcmError> + Send>;

pub struct AsyncFsmSender {
    client: Client,
    fcm_url: String,
}

impl AsyncFsmSender {
    pub fn new(api_key: String, fcm_url: String) -> AsyncFsmSender {
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
        AsyncFsmSender { client, fcm_url }
    }

    pub fn send(&self, msg: Message) -> GcmResponseFuture {
        let result = gcm_util::to_json(&msg);

        match result {
            Err(e) => Box::new(err(e)),
            Ok(body) => {
                let and_then = self
                    .client
                    .post(&self.fcm_url)
                    .body(body)
                    .send()
                    .map_err(|err| gcm_util::parse_error_status_code(err.status()))
                    .and_then(|res| AsyncFsmSender::parse(res));
                Box::new(and_then)
            }
        }
    }

    fn parse(mut res: Response) -> GcmResponseFuture {
        let status_code = res.status().as_u16();

        match status_code {
            200 => {
                let then = res
                    .json::<GcmResponse>()
                    .map_err(|_| GcmError::InvalidJsonBody)
                    .and_then(|gcm_resp| ok(gcm_resp));

                Box::new(then)
            }
            _ => Box::new(err(gcm_util::parse_error_status_code(Some(res.status())))),
        }
    }
}
