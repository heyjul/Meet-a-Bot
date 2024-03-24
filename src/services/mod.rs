pub mod graph_client;
pub mod teams_client;

pub use graph_client::GraphClient;
pub use teams_client::TeamsClient;

use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use reqwest::{header, Method, RequestBuilder};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::error::{Error, Result};

#[derive(Deserialize, Debug)]
struct Token {
    expires_in: usize,
    access_token: String,
    #[serde(skip, default = "Instant::now")]
    acquired: Instant,
}

impl Token {
    fn is_valid(&self) -> bool {
        let elapsed = self.acquired.elapsed();
        elapsed
            < Duration::from_secs(self.expires_in as u64)
                .checked_sub(Duration::from_secs(60))
                .unwrap_or_default()
    }
}

#[derive(Clone)]
pub struct BearerClient {
    client: reqwest::Client,
    token_url: String,
    form_data: String,
    base_url: Option<String>,
    token: Arc<Mutex<Option<Token>>>,
}

impl BearerClient {
    pub fn new(
        client: reqwest::Client,
        token_url: String,
        form_data: String,
        base_url: Option<String>,
    ) -> Self {
        Self {
            client,
            token_url: token_url.to_owned(),
            form_data: form_data.to_owned(),
            base_url: base_url.to_owned(),
            token: Arc::new(Mutex::new(None)),
        }
    }

    #[tracing::instrument(skip(self))]
    async fn fetch_token(&self) -> Result<Token> {
        let result = self
            .client
            .post(&self.token_url)
            .header(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("application/x-www-form-urlencoded"),
            )
            .body(self.form_data.clone())
            .send()
            .await?;

        match result.status().is_success() {
            false => Err(Error::Service(result.json().await?)),
            true => Ok(result.json().await?),
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn create_request(&self, method: Method, url: &str) -> Result<RequestBuilder> {
        let mut token = self.token.lock().await;

        match *token {
            Some(ref t) if !t.is_valid() => *token = Some(self.fetch_token().await?),
            None => *token = Some(self.fetch_token().await?),
            _ => (),
        }

        let url = match self.base_url {
            Some(ref base_url) => format!(
                "{base_url}/{url}",
                base_url = base_url.trim_end_matches('/'),
                url = url.trim_start_matches('\\')
            ),
            None => url.to_owned(),
        };

        let request = self
            .client
            .request(method, url)
            .bearer_auth(&token.as_ref().unwrap().access_token);

        Ok(request)
    }
}
