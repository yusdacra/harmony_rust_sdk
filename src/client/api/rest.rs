use super::*;
use crate::client::error::ClientError;

use prost::bytes::Bytes;
use reqwest::{multipart::*, Response};
use serde::Deserialize;

pub use crate::api::rest::*;

/// Uploads a file to the homeserver.
///
/// This endpoint requires authentication.
/// See [API documentation](https://github.com/harmony-development/protocol/blob/master/rest/rest.md#post-_harmonymediaupload).
pub async fn upload(
    client: &Client,
    filename: String,
    content_type: String,
    data: Vec<u8>,
) -> ClientResult<Response> {
    if !client.data.auth_status.lock().is_authenticated() {
        return Err(ClientError::Unauthenticated);
    }

    // This unwrap is safe, since our client's homeserver url is valid, and the path we create is also checked at compile time.
    let uri = client
        .homeserver_url()
        .join("/_harmony/media/upload")
        .unwrap();

    let form = Form::new().part("file", Part::bytes(data));

    let request = client
        .data
        .http
        .post(uri.as_str())
        .multipart(form)
        .header(http::header::AUTHORIZATION, unsafe {
            // This is safe on the assumption that servers will never send session tokens
            // with invalid-byte(s). If they do, they aren't respecting the protocol
            http::HeaderValue::from_maybe_shared_unchecked(client.data.token_bytes.lock().clone())
        })
        .query(&[("filename", &filename), ("contentType", &content_type)])
        .build()?;
    #[cfg(debug_assertions)]
    tracing::debug!("Sending HTTP request: {:?}", request);

    let response = client.data.http.execute(request).await?;
    #[cfg(debug_assertions)]
    tracing::debug!("Got HTTP response: {:?}", response);

    response.error_for_status().map_err(Into::into)
}

/// Downloads a file using a file ID.
///
/// This endpoint does not require authentication.
/// See [API documentation](https://github.com/harmony-development/protocol/blob/master/rest/rest.md#get-_harmonymediadownloadfile_id).
pub async fn download(client: &Client, file_id: impl Into<FileId>) -> ClientResult<Response> {
    const ENDPOINT: &str = "/_harmony/media/download/";

    let uri = match file_id.into() {
        FileId::Hmc(hmc) => format!(
            "https://{}:{}{}{}",
            hmc.server(),
            hmc.port(),
            ENDPOINT,
            hmc.id()
        )
        .parse()
        .unwrap(),
        FileId::Id(id) => client
            .homeserver_url()
            .join(ENDPOINT)
            .unwrap()
            .join(&id)
            .unwrap(),
        FileId::External(uri) => client
            .homeserver_url()
            .join(ENDPOINT)
            .unwrap()
            .join(urlencoding::encode(uri.as_str()).as_ref())
            .unwrap(),
    };

    let request = client.data.http.get(uri.as_str()).build()?;
    #[cfg(debug_assertions)]
    tracing::debug!("Sending HTTP request: {:?}", request);

    let response = client.data.http.execute(request).await?;
    #[cfg(debug_assertions)]
    tracing::debug!("Got HTTP response: {:?}", response);

    response.error_for_status().map_err(Into::into)
}

/// Uploads a file and then extracts the file ID from the returned response.
///
/// Also see [`upload()`].
pub async fn upload_extract_id(
    client: &Client,
    filename: String,
    content_type: String,
    data: Vec<u8>,
) -> ClientResult<String> {
    #[derive(Debug, Deserialize)]
    struct FileId {
        id: String,
    }

    let resp = upload(client, filename, content_type, data).await?;
    let file_id: FileId = resp.json().await?;

    Ok(file_id.id)
}

/// A downloaded file.
#[derive(Debug, Clone)]
pub struct DownloadedFile {
    data: Bytes,
    mimetype: String,
    kind: FileKind,
    name: String,
}

impl DownloadedFile {
    /// Get the file kind.
    pub fn kind(&self) -> FileKind {
        self.kind
    }

    /// Get the file name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the mimetype.
    pub fn mimetype(&self) -> &str {
        &self.mimetype
    }

    /// Get the data.
    pub fn data(&self) -> &Bytes {
        &self.data
    }
}

/// Downloads a file then extracts file information from it.
///
/// Also see [`download()`].
pub async fn download_extract_file(
    client: &Client,
    file_id: impl Into<FileId>,
) -> ClientResult<DownloadedFile> {
    let resp = download(client, file_id).await?;

    let (name, mimetype, kind) = extract_file_info_from_download_response(resp.headers())
        .map(|(name, mimetype, kind)| {
            (
                name.to_owned(),
                mimetype
                    .to_str()
                    .map(ToOwned::to_owned)
                    .map_err(|_| ClientError::unexpected("Content-Type is not ASCII")),
                kind,
            )
        })
        .map_err(ClientError::unexpected)?;

    let data = resp.bytes().await?;

    Ok(DownloadedFile {
        data,
        mimetype: mimetype?,
        kind,
        name,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn parse_id() {
        const ID: &str = "654624512343";
        let file_id = FileId::from_str(ID).expect("file id parse");
        assert!(matches!(file_id, FileId::Id(_)));
        assert_eq!(ID.to_string(), file_id.to_string());
    }

    #[test]
    fn parse_hmc() {
        const HMC: &str = "hmc://chat.harmonyapp.io/654624512343";
        let file_id = FileId::from_str(HMC).expect("file id parse");
        assert!(matches!(file_id, FileId::Hmc(_)));
        assert_eq!(HMC.to_string(), file_id.to_string());
    }

    #[test]
    fn parse_uri() {
        const URI: &str = "https://media.discordapp.net/attachments/330412938277945345/801119250269339728/unknown.png";
        let file_id = FileId::from_str(URI).expect("file id parse");
        assert!(matches!(file_id, FileId::External(_)));
        assert_eq!(URI.to_string(), file_id.to_string());
    }

    #[test]
    #[should_panic(expected = "InvalidFileId")]
    fn parse_empty() {
        const EMPTY: &str = "";
        FileId::from_str(EMPTY).expect("file id parse");
    }
}
