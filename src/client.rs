#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use url::Url;
use serde::{Deserialize, Serialize};
use std::{net::Ipv4Addr, path::{Path, PathBuf}};

use crate::{error::Error, Result, filesystem::{RemoteFile, safe_join}, transport::Transport};


/// Device info
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Device {
    /// Firmware version
    version: String,
    /// Device IP address
    ip: Ipv4Addr,
    /// "STA" for joined Wi-Fi or "AP" for hotspot mode
    mode: String,
    /// Signal strength in dBm; 0 in AP mode
    rssi: i32,
    /// Free heap in bytes
    #[serde(rename = "freeHeap")]
    free_heap: u32,
    /// Seconds since boot
    uptime: u64,
    /// Device name
    device: String,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct CrossPointClient {
    /// Transport depends on the platform
    /// Native uses "reqwest" and Web uses "gloo-net"
    transport: Transport,
}

// Common
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl CrossPointClient {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(base: Option<String>) -> Result<Self> {
        let b = base.unwrap_or("http://crosspoint.local".to_owned()); // Realistically the best default address
        let transport = Transport::new(Url::parse(&b)?);
        Ok( Self { transport } )
    }
}
// Internal
impl CrossPointClient {
    /// Equivalent of:
    ///    curl "http://crosspoint.local/api/status"
    async fn status_internal(&self) -> Result<Device> {
        self.transport
            .get_json("api/status", &[])
            .await
    }
    /// Equivalent of:
    ///    curl "http://crosspoint.local/api/files?path={dir}"
    async fn list_internal<T: AsRef<str>>(&self, dir: T) -> Result<Vec<RemoteFile>> {
        self.transport
            .get_json("api/files", &[("path", dir.as_ref())])
            .await
    }
    /// Equivalent of:
    ///    curl -X POST "http://crosspoint.local/mkdir" -d "path={parent}" -d "name={name}"
    async fn mkdir_internal<T: AsRef<str>>(&self, path: T) -> Result<()> {
        let (parent, name) = path.as_ref()
            .rsplit_once('/')
            .ok_or(Error::Protocol("invalid path".into()))?;

        let parent = if parent.is_empty() { "/" } else { parent };

        self.transport
            .post_form(
                "mkdir",
                &[],
                &[
                    ("path", parent),
                    ("name", name),
                ],
            )
            .await
    }
    /// Equivalent of:
    ///    curl -X POST "http://crosspoint.local/delete" -d "path={path}"
    async fn delete_internal<T: AsRef<str>>(&self, path: T) -> Result<()> {
        self.transport
            .post_form(
                "delete",
                &[],
                &[("path", path.as_ref())],
            )
            .await
    }
    /// Equivalent of:
    ///    curl -X POST "http://crosspoint.local/delete" -d 'paths=["/file1.ext", "/file2.ext", ...]'
    async fn delete_multiple_internal<T: AsRef<str> + Serialize>(&self, paths: Vec<T>) -> Result<()> {
        let json_string = serde_json::to_string(&paths)?;
        self.transport
            .post_form(
                "delete",
                &[],
                &[("paths", &json_string)],
            )
            .await
    }
    /// Gathers all of the files in a folder, then deletes them and then the folder.
    /// It's non-recursive though
    async fn force_delete_directory_internal<T: AsRef<str>>(&self, dir: T) -> Result<()> {
        // A little bit of fool-proofing is always good
        assert_ne!(dir.as_ref(), "/", "{}", Error::Protocol("cannot delete the root".into()));
        assert_ne!(dir.as_ref(), "\\", "{}", Error::Protocol("cannot delete the root".into()));

        // Gather all of the file paths
        let base_path = Path::new(dir.as_ref());
        let files_and_dirs= self.list_internal(dir.as_ref()).await?;
        let path_buffers: Vec<PathBuf> = files_and_dirs.iter().filter_map(|f|
            if f.is_directory { None } else { safe_join(base_path, &f.name) }
        ).collect(); // Exclude dirs and  make full paths
        let mut paths: Vec<String> = path_buffers.iter().map(|f| {
            f.to_string_lossy().replace("\\", "/") // It doesn't like backslashes
        }).collect(); // Convert paths to strings
        
        paths.push(dir.as_ref().to_owned()); // Add the dir itself

        if !paths.is_empty() { self.delete_multiple_internal(paths).await? }
        Ok(())
    }
    /// Equivalent of:
    ///    curl -X POST "http://crosspoint.local/upload?path={dir}" -F "file=@file.ext"
    async fn upload_internal<T: AsRef<str>, U: AsRef<str>>(&self, dir: T, name: U, bytes: &[u8]) -> Result<()> {
        self.transport
            .upload(
                "upload",
                &[("path", dir.as_ref())],
                name.as_ref(),
                bytes,
            )
            .await
    }
    /// Equivalent of:
    ///    curl -OJ "http://crosspoint.local/download?path={path}"
    async fn download_internal<T: AsRef<str>>(&self, path: T) -> Result<Vec<u8>> {
        self.transport
            .get_bytes(
                "download",
                &[("path", path.as_ref())],
            )
            .await
    }
    /// Equivalent of:
    ///    curl -X POST "http://crosspoint.local/rename" -d "path=/{path}&name={name}"
    async fn rename_internal<T: AsRef<str>, U: AsRef<str>>(&self, path: T, name: U) -> Result<()> {
        self.transport
            .post_form(
                "rename",
                &[],
                &[
                    ("path", path.as_ref()),
                    ("name", name.as_ref())
                ],
            )
            .await
    }
}
// Expose to native
#[cfg(not(target_arch = "wasm32"))]
impl CrossPointClient {
    pub async fn status(&self) -> Result<Device> {
        self.status_internal().await
    }
    pub async fn list<T: AsRef<str>>(&self, dir: T) -> Result<Vec<RemoteFile>> {
        self.list_internal(dir).await
    }
    pub async fn mkdir<T: AsRef<str>>(&self, path: T) -> Result<()> {
        self.mkdir_internal(path).await
    }
    pub async fn delete<T: AsRef<str>>(&self, path: T) -> Result<()> {
        self.delete_internal(path).await
    }
    pub async fn delete_multiple<T: AsRef<str> + Serialize>(&self, paths: Vec<T>) -> Result<()> {
        self.delete_multiple_internal(paths).await
    }
    pub async fn force_delete_directory<T: AsRef<str>>(&self, dir: T) -> Result<()> {
       self.force_delete_directory_internal(dir).await
    }
    pub async fn upload<T: AsRef<str>, U: AsRef<str>>(&self, dir: T, name: U, bytes: &[u8]) -> Result<()> {
        self.upload_internal(dir, name, bytes).await
    }
    pub async fn download<T: AsRef<str>>(&self, path: T) -> Result<Vec<u8>> {
        self.download_internal(path).await
    }
    pub async fn rename<T: AsRef<str>, U: AsRef<str>>(&self, path: T, name: U) -> Result<()> {
        self.rename_internal(path, name).await
    }
}
// Expose to web
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl CrossPointClient {
    pub async fn status(&self) -> Result<Device> {
        self.status_internal().await
    }
    pub async fn list(&self, dir: String) -> Result<JsValue> {
        let files = self.list_internal(dir).await?;
        let js_array = serde_wasm_bindgen::to_value(&files)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(js_array)
    }
    pub async fn mkdir(&self, path: String) -> Result<()> {
        self.mkdir_internal(path).await
    }
    pub async fn delete(&self, path: String) -> Result<()> {
        self.delete_internal(path).await
    }
    #[wasm_bindgen(js_name = deleteMultiple)]
    pub async fn delete_multiple(&self, paths: Vec<String>) -> Result<()> {
        self.delete_multiple_internal(paths).await
    }
    #[wasm_bindgen(js_name = forceDeleteDirectory)]
    pub async fn force_delete_directory(&self, dir: String) -> Result<()> {
       self.force_delete_directory_internal(dir).await
    }
    pub async fn upload(&self, dir: String, name: String, bytes: &[u8]) -> Result<()> {
        self.upload_internal(dir, name, bytes).await
    }
    pub async fn download(&self, path: String) -> Result<Vec<u8>> {
        self.download_internal(path).await
    }
    pub async fn rename(&self, path: String, name: String) -> Result<()> {
        self.rename_internal(path, name).await
    }
}