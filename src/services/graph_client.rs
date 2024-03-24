use reqwest::Method;
use serde::Deserialize;
use std::ops::Deref;

use super::BearerClient;
use crate::error::{Error, Result};

#[derive(Clone)]
pub struct GraphClient {
    client: BearerClient,
}

impl Deref for GraphClient {
    type Target = BearerClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl GraphClient {
    pub fn new(
        client: reqwest::Client,
        client_id: &str,
        client_secret: &str,
        tenant_id: &str,
    ) -> Self {
        let data = format!("grant_type=client_credentials&client_id={client_id}&client_secret={client_secret}&scope=https%3A%2F%2Fgraph.microsoft.com%2F.default");
        let token_url = format!("https://login.microsoftonline.com/{tenant_id}/oauth2/v2.0/token");
        let base_url = Some("https://graph.microsoft.com/v1.0".to_owned());

        Self {
            client: BearerClient::new(client, token_url, data, base_url),
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_chat(&self, chat_id: &str) -> Result<GetChatResponse> {
        let result = self
            .create_request(Method::GET, &format!("chats/{chat_id}"))
            .await?
            .send()
            .await?;

        match result.status().is_success() {
            false => Err(Error::Service(result.json().await?)),
            true => Ok(result.json().await?),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GetChatResponse {
    pub topic: String,
}
