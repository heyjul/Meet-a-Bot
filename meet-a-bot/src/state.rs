use teams_api::client::TeamsBotClient;

#[derive(Clone)]
pub struct AppState {
    pub client: TeamsBotClient,
}
