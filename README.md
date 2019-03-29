gcm-rust-client
===


- This is Rust client to talk to GCM / FCM using GCM/FCM rest apis. 
- This library is inspired by [gcm](https://github.com/vishy1618/gcm) and **added following modifications on top off that** 
  - Single client for multiple messages
  - Parsing of message id if request is successful for id in Result array object
  - Support FCM and GCM both rest apis
- **New Features**
  - [x] Reuse single client to send multiple messages
  - [x] Support async client backed by [reqwest](https://github.com/seanmonstar/reqwest) async http client
  - [ ] Add client config object (e.g. keep alive,pool,read timeout)
  - [ ] Parse failed resgisteration ids in response
  - [ ] Exhaustive response code handling 


## Usage

Add this to `Cargo.toml`:

```rust
[dependencies]
gcm = { git = "https://github.com/ss025/gcm-rust-client"}
```

## Examples:


### Sync


Here is an example to send out a GCM/FCM Message with some custom data:
 
```rust
extern crate gcm;

use std::collections::HashMap;

use gcm::Message;
use gcm::sender::FcmSender;

fn main() {
    let mut data = HashMap::new();
    data.insert("message", "Howdy!");

    let google_api = "https://fcm.googleapis.com/fcm/send".to_string();
    let api_key = "<api-key>".to_string();
    let registration_ids = vec!["registration-id-1", "registration-id-1"];

    let fcm_sender = FcmSender::new(google_api, api_key);

    let msg = Message::new(registration_ids).data(data).build();
    let result = fcm_sender.send(msg);

    match result {
        Ok(resp) => println!("response {:?}", serde_json::to_string(&resp)),
        Err(err) => println!("Error : {:?}", err)
    }
}
```



### Async

```rust
extern crate futures;
extern crate gcm;

use std::collections::HashMap;

use futures::Future;
use futures::future::ok;

use gcm::async_sender::{AsyncFsmSender, GcmResponseFuture};
use gcm::Message;

fn main() {
    tokio::run(process());
}


fn process() -> impl Future<Item=(), Error=()> {
    send_to_gcm()
        .map_err(|e| {
            println!("Got Gcm Error {:?}", e);
            ()
        })

        .and_then(|res| {
            println!("Got Gcm Response {:?}", res);
            ok(())
        })
}


fn send_to_gcm() -> GcmResponseFuture {
    let mut data = HashMap::new();
    data.insert("message", "Howdy!");

    let fcm_url = "https://fcm.googleapis.com/fcm/send".to_string();
    let api_key = "<api-key>".to_string();
    let registration_ids = vec!["registration-id-1", "registration-id-1"];
    let msg = Message::new(registration_ids).data(data).build();
    let build_ids_by_error_map = true ;
    let sender = AsyncFsmSender::new(api_key, fcm_url,build_ids_by_error_map);
    sender.send(msg)
}

```


### Sample Response 

```json
{
  "message_id": null,
  "error": null,
  "multicast_id": 5552427494506560000,
  "success": 1,
  "failure": 1,
  "canonical_ids": 0,
  "results": [
    {
      "message_id": "0:1553856661919282%313d616af9fd7ecd",
      "registration_id": null,
      "error": null
    },
    {
      "message_id": null,
      "registration_id": null,
      "error": "InvalidRegistration"
    }
  ],
  "ids_by_error": {
    "InvalidRegistration": [
      "abc"
    ]
  }
}


```
