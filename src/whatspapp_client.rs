use serde::{de::DeserializeOwned, Serialize};

use crate::{models::Response, WhatsappError};

pub struct WhatasppClient {
    access_token: String,
    url: String,
}

impl WhatasppClient {
    pub fn new(access_token: &str, url: &str) -> Self {
        Self {
            access_token: access_token.into(),
            url: url.into(),
        }
    }

    pub async fn send<T, U>(&self, message: T) -> Result<Response<U>, WhatsappError>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        http_client::post(&self.url, &self.access_token, &message).await
    }
}

mod http_client {
    use reqwest::StatusCode;
    use serde::{de::DeserializeOwned, Serialize};

    use crate::{models::Response, WhatsappError};

    pub async fn post<T, U>(
        url: &str,
        bearer_token: &str,
        data: &T,
    ) -> Result<Response<U>, WhatsappError>
    where
        T: Serialize + ?Sized,
        U: DeserializeOwned,
    {
        let client = reqwest::Client::new();
        let resp = client
            .post(url)
            .bearer_auth(&bearer_token)
            .json(&data)
            .send()
            .await?;
        let status: u16 = resp.status().as_u16();
        match resp.status() {
            StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => {
                let json = resp.json::<U>().await?;
                let wrapper = Response {
                    status_code: status,
                    data: json,
                };
                Ok(wrapper)
            }
            _ => {
                log::warn!("{:?}", &resp);
                let error_text = &resp.text().await?;
                log::warn!("{:?}", &error_text);
                Err(WhatsappError::UnexpectedError(error_text.to_string()))
            }
        }
    }
}
