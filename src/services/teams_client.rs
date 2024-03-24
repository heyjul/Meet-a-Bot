use std::ops::Deref;

use reqwest::Method;

use crate::{
    error::{Error, Result},
    models::{
        activity::Activity, ConversationParameters, ConversationResourceResponse, ResourceResponse,
    },
};

use super::BearerClient;

const BASE_URL: &str = "https://smba.trafficmanager.net/teams";

#[derive(Clone)]
pub struct TeamsClient {
    client: BearerClient,
}

impl Deref for TeamsClient {
    type Target = BearerClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl TeamsClient {
    pub fn new(client: reqwest::Client, client_id: &str, client_secret: &str) -> Self {
        let data = format!("grant_type=client_credentials&client_id={client_id}&client_secret={client_secret}&scope=https%3A%2F%2Fapi.botframework.com%2F.default");
        let token_url =
            "https://login.microsoftonline.com/botframework.com/oauth2/v2.0/token".to_owned();

        Self {
            client: BearerClient::new(client, token_url, data, None),
        }
    }

    #[tracing::instrument(skip(self, body))]
    pub async fn create_conversation(
        &self,
        base_url: Option<&str>,
        body: &ConversationParameters,
    ) -> Result<ConversationResourceResponse> {
        let result = self
            .create_request(
                Method::POST,
                &format!(
                    "{base_url}/v3/conversations",
                    base_url = base_url.map_or(BASE_URL, |x| x.trim_end_matches('/'))
                ),
            )
            .await?
            .json(body)
            .send()
            .await?;

        match result.status().is_success() {
            false => Err(Error::Service(result.json().await?)),
            true => Ok(result.json().await?),
        }
    }

    /// Sends an activity (message) to the specified conversation. The activity will be appended to the end of the conversation according to the timestamp or semantics of the channel. To reply to a specific message within the conversation, use Reply to Activity instead.
    #[tracing::instrument(skip(self, body))]
    pub async fn send_to_conversation(
        &self,
        base_url: Option<&str>,
        conversation_id: &str,
        body: &Activity,
    ) -> Result<ResourceResponse> {
        let result = self
            .create_request(
                Method::POST,
                &format!(
                    "{base_url}/v3/conversations/{conversation_id}/activities",
                    base_url = base_url.map_or(BASE_URL, |x| x.trim_end_matches('/'))
                ),
            )
            .await?
            .json(body)
            .send()
            .await?;

        match result.status().is_success() {
            false => Err(Error::Service(result.json().await?)),
            true => Ok(result.json().await?),
        }
    }

    /// Some channels allow you to edit an existing activity to reflect the new state of a bot conversation. For example, you might remove buttons from a message in the conversation after the user has clicked one of the buttons. If successful, this operation updates the specified activity within the specified conversation.
    #[tracing::instrument(skip(self, body))]
    pub async fn update_activity(
        &self,
        base_url: Option<&str>,
        conversation_id: &str,
        activity_id: &str,
        body: &Activity,
    ) -> Result<ResourceResponse> {
        let result = self
            .create_request(
                Method::PUT,
                &format!(
                    "{base_url}/v3/conversations/{conversation_id}/activities/{activity_id}",
                    base_url = base_url.map_or(BASE_URL, |x| x.trim_end_matches('/'))
                ),
            )
            .await?
            .json(body)
            .send()
            .await?;

        match result.status().is_success() {
            false => Err(Error::Service(result.json().await?)),
            true => Ok(result.json().await?),
        }
    }
}
