use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FreshChatMessageResponse {
    pub request_id: String,
    pub request_process_time: String,
    pub link: Link,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    rel: String,
    href: String,
}
