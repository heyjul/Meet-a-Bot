use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use reqwest::{header, Method, RequestBuilder};
use serde::Deserialize;

use crate::models::{requests::*, responses::*, Activity};

#[derive(Clone)]
pub struct TeamsBotClient {
    client: reqwest::Client,
    client_id: String,
    client_secret: String,
    token: Option<Arc<Token>>,
}

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
                .unwrap_or(Duration::default())
    }
}

impl TeamsBotClient {
    pub fn new(client: reqwest::Client, client_id: &str, client_secret: &str) -> Self {
        Self {
            client,
            client_id: client_id.to_owned(),
            client_secret: client_secret.to_owned(),
            token: None,
        }
    }

    async fn fetch_token(&mut self) -> Token {
        let data = format!("grant_type=client_credentials&client_id={client_id}&client_secret={client_secret}&scope=https%3A%2F%2Fapi.botframework.com%2F.default", client_id = self.client_id, client_secret = self.client_secret);

        let result = self
            .client
            .post("https://login.microsoftonline.com/botframework.com/oauth2/v2.0/token")
            .header(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("application/x-www-form-urlencoded"),
            )
            .body(data)
            .send()
            .await
            .expect("Failed to fetch bearer token");

        assert!(result.status().is_success());

        result.json().await.expect("Failed to deserialize token")
    }

    async fn create_request(
        &mut self,
        method: Method,
        base_url: &str,
        url: &str,
    ) -> RequestBuilder {
        if let Some(ref token) = self.token {
            if !token.is_valid() {
                self.token = Some(Arc::new(self.fetch_token().await));
            }
        } else {
            self.token = Some(Arc::new(self.fetch_token().await));
        }

        self.client
            .request(
                method,
                format!("{base_url}{url}", base_url = base_url.trim_end_matches('/')),
            )
            .bearer_auth(&self.token.as_ref().unwrap().access_token)
    }

    /// Creates a new conversation.
    pub async fn create_conversation(
        &mut self,
        base_url: &str,
        body: &ConversationParameters,
    ) -> ConversationResourceResponse {
        let result = self
            .create_request(Method::POST, base_url, "/v3/conversations")
            .await
            .json(body)
            .send()
            .await
            .expect("Failed to send request");

        assert!(result.status().is_success());

        result.json().await.expect("Failed to deserialize response")
    }

    /// Sends an activity (message) to the specified conversation. The activity will be appended to the end of the conversation according to the timestamp or semantics of the channel. To reply to a specific message within the conversation, use Reply to Activity instead.
    pub async fn send_to_conversation(
        &mut self,
        base_url: &str,
        conversation_id: &str,
        body: &Activity,
    ) -> ResourceResponse {
        let result = self
            .create_request(
                Method::POST,
                base_url,
                &format!("/v3/conversations/{conversation_id}/activities"),
            )
            .await
            .json(body)
            .send()
            .await
            .expect("Failed to send request");

        assert!(result.status().is_success());

        result.json().await.expect("Failed to deserialize response")
    }
}
