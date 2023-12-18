use landeed_whatsapp_cloud_api::models::{
    Body, Data, FreshChatMessage, FreshChatMessageRequest, FreshChatMessageResponse, From,
    MessageTemplate, Params, RetrievedFreshChatResponse, RichTemplateData, To,
};
use landeed_whatsapp_cloud_api::{models::Response, WhatsappClient, WhatsappError};

#[tokio::test]
async fn send_retrieve_freshchat_message_works() -> Result<(), WhatsappError> {
    setup();
    let access_token = std::env::var("WHATSAPP_ACCESS_TOKEN")
        .expect("Missing environment variable WHATSAPP_ACCESS_TOKEN");
    let namespace = std::env::var("WHATSAPP_NAMESPACE_ID")
        .expect("Missing environment variable WHATSAPP_NAMESPACE_ID");
    let url =
        std::env::var("WHATSAPP_BASE_URL").expect("Missing environment variable WHATSAPP_BASE_URL");
    let template =
        std::env::var("WHATSAPP_TEMPLATE").expect("Missing environment variable WHATSAPP_TEMPLATE");
    let params: Vec<String> = std::env::var("WHATSAPP_PARAMS")
        .expect("Missing environment variable WHATSAPP_PARAMS")
        .split(',')
        .map(|s| s.to_string())
        .collect();

    let from = std::env::var("WHATSAPP_SEND_FROM")
        .expect("Missing environment variable WHATSAPP_SEND_FROM");
    let to =
        std::env::var("WHATSAPP_SEND_TO").expect("Missing environment variable WHATSAPP_SEND_TO");

    let rich_template_data = RichTemplateData {
        body: Body {
            params: params.into_iter().map(|data| Params { data }).collect(),
        },
    };
    let data = Data {
        message_template: MessageTemplate::new(
            &template,
            &namespace,
            "en",
            "conversation",
            rich_template_data,
        ),
    };
    let from = From {
        phone_number: from.to_string(),
    };
    let to = vec![to.to_string()]
        .into_iter()
        .map(|phone_number| To { phone_number })
        .collect();

    let client = WhatsappClient::new(access_token, url);
    let message = FreshChatMessage::from_text(from, to, data);
    let response: Response<FreshChatMessageResponse> = client
        .send("/v2/outbound-messages/whatsapp", message)
        .await?;
    //println!("{:?}", response);

    let message = FreshChatMessageRequest {
        request_id: response.data.request_id,
    };
    let response: Response<RetrievedFreshChatResponse> =
        client.retrieve("v2/outbound-messages", message).await?;
    println!("{:?}", response);

    Ok(())
}

fn setup() {
    dotenv::dotenv().ok();
    let _ = env_logger::builder().is_test(true).try_init();
}
