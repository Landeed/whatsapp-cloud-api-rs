use serde::{Deserialize, Serialize};

use super::Language;

const WHATSAPP: &str = "whatsapp";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct From {
    pub phone_number: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct To {
    pub phone_number: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub data: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Body {
    pub params: Vec<Params>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichTemplateData {
    pub body: Body,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageTemplate {
    pub template_name: String,
    pub namespace: String,
    pub language: Language,
    pub storage: String,
    pub rich_template_data: RichTemplateData,
}

impl MessageTemplate {
    pub fn new(
        template_name: &str,
        namespace: &str,
        language: &str,
        storage: &str,
        rich_template_data: RichTemplateData,
    ) -> Self {
        let language = Language::new(language);
        Self {
            template_name: template_name.into(),
            namespace: namespace.into(),
            language,
            storage: storage.into(),
            rich_template_data: rich_template_data,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Data {
    pub message_template: MessageTemplate,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FreshChatMessage {
    pub from: From,
    pub provider: String,
    pub to: Vec<To>,
    pub data: Data,
}

impl FreshChatMessage {
    pub fn from_text(from: From, to: Vec<To>, data: Data) -> Self {
        Self {
            from: from,
            provider: WHATSAPP.into(),
            to: to,
            data: data,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FreshChatMessageStatus {
    Accepted,
    InProgress,
    Sent,
    Delivered,
    Failed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RetrievedFreshChatMessage {
    pub message_id: String,
    pub from: From,
    pub provider: String,
    pub to: To,
    pub data: Data,
    pub request_id: String,
    pub status: FreshChatMessageStatus,
    pub created_on: u64,
    pub failure_code: Option<String>,
    pub failure_reason: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RetrievedFreshChatResponse {
    pub outbound_messages: Vec<RetrievedFreshChatMessage>,
}
