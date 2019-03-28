gcm
===

- [x] Reuse single client across all messages
- [ ] Add keep alive
- [ ] Add retry for failed registration ids



## Usage

Add this to `Cargo.toml`:

```rust
[dependencies]
gcm = { git = "https://github.com/ss025/gcm" , branch = "fcm" }
```

## Examples:
 
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

