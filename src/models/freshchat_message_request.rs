use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FreshChatMessageRequest {
    pub request_id: String,
}
