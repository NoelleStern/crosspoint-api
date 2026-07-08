#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use thiserror::Error;


pub type Result<T> = std::result::Result<T, Error>;


/// Possible errors
#[derive(Debug, Error)]
pub enum Error {
    #[error("network error: {0}")]
    Network(String),
    #[error("Invalid URL")]
    Url(#[from] url::ParseError),
    #[error("Serialization error")]
    Json(#[from] serde_json::Error),
    #[error("HTTP {status}: {message}")]
    Http {
        status: StatusCode,
        message: String,
    },
    #[error("Device protocol error: {0}")]
    Protocol(String),
    #[error("Javascript error: {0}")]
    Js(String),
}
#[cfg(not(target_arch = "wasm32"))]
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Network(err.to_string())
    }
}
#[cfg(target_arch = "wasm32")]
impl From<gloo_net::Error> for Error {
    fn from(err: gloo_net::Error) -> Self {
        Self::Network(err.to_string())
    }
}
#[cfg(target_arch = "wasm32")]
impl From<wasm_bindgen::JsValue> for Error {
    fn from(value: wasm_bindgen::JsValue) -> Self {
        Error::Js(
            value.as_string().unwrap_or_else(|| format!("{value:?}"))
        )
    }
}
#[cfg(target_arch = "wasm32")]
impl From<Error> for JsValue {
    fn from(err: Error) -> JsValue {
        JsValue::from_str(&err.to_string())
    }
}

/// Minimal HTTP status code implementation
#[derive(Debug, Clone, Copy)]
pub struct StatusCode(pub u16);
impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl StatusCode {
    pub fn new(code: u16) -> Self { Self(code) }
    pub fn as_u16(self) -> u16 { self.0 }
    pub fn is_success(self) -> bool { (200..300).contains(&self.0) }
}