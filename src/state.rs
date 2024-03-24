use sqlx::SqlitePool;

use crate::services::{graph_client::GraphClient, teams_client::TeamsClient};

#[derive(Clone)]
pub struct AppState {
    pub teams_client: TeamsClient,
    pub graph_client: GraphClient,
    pub pool: SqlitePool,
}
