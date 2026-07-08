//!
//! Native transport implementation using "reqwest"
//! 


use url::Url;
use serde::de::DeserializeOwned;
use reqwest::{Response, multipart::{Form, Part}};

use crate::{Error, Result, error::StatusCode};


pub(crate) struct Transport {
    client: reqwest::Client,
    base_url: Url,
}
impl Transport {
    pub fn new(base_url: Url) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
        }
    }

    fn build_url(&self, endpoint: &str) -> Result<Url> {
        Ok(self.base_url.join(endpoint)?)
    }
    // Ownership model rules until it doesn't
    async fn check_status(response: Response) -> Result<Response> {
        let status = StatusCode(response.status().as_u16());
        if !status.is_success() {
            let message = response.text().await.unwrap_or_default().trim().to_owned();
            return Err(Error::Http { status, message })
        }
        Ok(response)
    }
 
    pub async fn get_json<T: DeserializeOwned>(&self, endpoint: &str, query: &[(&str, &str)]) -> Result<T> {
        let response = self
            .client
            .get(self.build_url(endpoint)?)
            .query(query)
            .send()
            .await?;

        Ok(Self::check_status(response).await?.json().await?)
    }
    pub async fn get_bytes(&self, endpoint: &str, query: &[(&str, &str)]) -> Result<Vec<u8>> {
        let response = self
            .client
            .get(self.build_url(endpoint)?)
            .query(query)
            .send()
            .await?;

        Ok(Self::check_status(response).await?.bytes().await?.to_vec())
    }
    pub async fn post_form(&self, endpoint: &str, query: &[(&str, &str)], form: &[(&str, &str)]) -> Result<()> {
        let response = self
            .client
            .post(self.build_url(endpoint)?)
            .query(query)
            .form(form)
            .send()
            .await?;

        Self::check_status(response).await?;
        Ok(())
    }
    /// Basically a more specific post_form()
    pub async fn upload(&self, endpoint: &str, query: &[(&str, &str)], filename: &str, bytes: &[u8]) -> Result<()> {
        let form = Form::new().part(
            "file", Part::bytes(bytes.to_vec()).file_name(filename.to_owned())
        );

        let response = self
            .client
            .post(self.build_url(endpoint)?)
            .query(query)
            .multipart(form)
            .send()
            .await?;

        Self::check_status(response).await?;
        Ok(())
    }
}