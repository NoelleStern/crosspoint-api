//! 
//! Web transport implementation using "gloo-net"
//! 


use url::Url;
use web_sys::{Blob, FormData};
use serde::de::DeserializeOwned;
use gloo_net::http::{Request, Response};

use crate::{Error, error::{Result, StatusCode}};


pub(crate) struct Transport {
    base_url: Url,
}
impl Transport {
    pub fn new(base_url: Url) -> Self {
        Self { base_url }
    }

    fn build_url(&self, endpoint: &str, query: &[(&str, &str)]) -> Result<String> {
        let mut url = self.base_url.join(endpoint)?;
        {
            let mut pairs = url.query_pairs_mut();
            for (k, v) in query { pairs.append_pair(k, v); }
        }
        Ok(url.to_string())
    }
    // Ownership model rules until it doesn't
    async fn check_status(response: Response) -> Result<Response> {
        let status = StatusCode(response.status());
        if !status.is_success() {
            let message = response.text().await.unwrap_or_default();
            return Err(Error::Http { status, message })
        }
        Ok(response)
    }

    pub async fn get_json<T: DeserializeOwned>(&self, endpoint: &str, query: &[(&str, &str)]) -> Result<T> {
        let response = Request::get(
            &self.build_url(endpoint, query)?
        ).send().await?;

        Ok(Self::check_status(response).await?.json().await?)
    }
    pub async fn get_bytes(&self, endpoint: &str, query: &[(&str, &str)]) -> Result<Vec<u8>> {
        let response = Request::get(
            &self.build_url(endpoint, query)?
        ).send().await?;

        Ok(Self::check_status(response).await?.binary().await?.to_vec())
    }
    pub async fn post_form(&self, endpoint: &str, query: &[(&str, &str)], form: &[(&str, &str)]) -> Result<()> {
        let body = FormData::new()
            .map_err(|e| Error::Protocol(format!("{e:?}")))?;

        for (k, v) in form {
            body.append_with_str(k, v)
                .map_err(|e| Error::Protocol(format!("{e:?}")))?;
        }

        let response = Request::post(
            &self.build_url(endpoint, query)?
        ).body(body)?.send().await?;

        Self::check_status(response).await?;
        Ok(())
    }
    /// Basically a more specific post_form()
    pub async fn upload(&self, endpoint: &str, query: &[(&str, &str)], filename: &str, bytes: &[u8]) -> Result<()> {
        let body = FormData::new().map_err(|e| Error::Protocol(format!("{e:?}")))?;
        let array = js_sys::Uint8Array::from(bytes);
        let parts = js_sys::Array::new();
        parts.push(&array.buffer());

        let blob = Blob::new_with_u8_array_sequence(&parts).map_err(|e| Error::Protocol(format!("{e:?}")))?;
        body.append_with_blob_and_filename(
            "file", &blob, filename
        ).map_err(|e| Error::Protocol(format!("{e:?}")))?;

        let response = Request::post(
            &self.build_url(endpoint, query)?
        ).body(body)?.send().await?;

        Self::check_status(response).await?;
        Ok(())
    }
}