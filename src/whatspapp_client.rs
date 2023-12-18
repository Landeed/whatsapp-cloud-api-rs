use serde::{de::DeserializeOwned, Serialize};

use crate::{models::Response, WhatsappError};

pub struct WhatsappClient {
    access_token: String,
    base_url: String,
}

impl WhatsappClient {
    pub fn new(access_token: String, base_url: String) -> Self {
        Self {
            access_token,
            base_url,
        }
    }

    pub async fn send<T, U>(&self, path: &str, message: T) -> Result<Response<U>, WhatsappError>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        http_client::post(
            url::Url::parse(&self.base_url)?.join(path)?,
            &self.access_token,
            &message,
        )
        .await
    }

    pub async fn retrieve<T, U>(&self, path: &str, params: T) -> Result<Response<U>, WhatsappError>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        http_client::get(
            url::Url::parse(&self.base_url)?.join(path)?,
            &self.access_token,
            &params,
        )
        .await
    }
}

mod http_client {
    use reqwest::StatusCode;
    use serde::{de::DeserializeOwned, Serialize};

    use crate::{models::Response, WhatsappError};

    pub async fn post<T, U>(
        url: url::Url,
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
    pub async fn get<T, U>(
        url: url::Url,
        bearer_token: &str,
        params: &T,
    ) -> Result<Response<U>, WhatsappError>
    where
        T: Serialize + ?Sized,
        U: DeserializeOwned,
    {
        let client = reqwest::Client::new();
        let resp = client
            .get(url)
            .bearer_auth(&bearer_token)
            .query(params)
            .send()
            .await?;
        let status: u16 = resp.status().as_u16();
        match resp.status() {
            StatusCode::OK => {
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
