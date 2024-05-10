use std::env;

use axum::{routing::post, Router};
use meet_a_bot::{
    routes::message_route,
    services::{GraphClient, TeamsClient},
    state::AppState,
};
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt()
        // .with_span_events(tracing_subscriber::fmt::format::FmtSpan::NEW)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to register tracing");

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 7071,
    };

    let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    let client_id = env::var("TEAMS_CLIENT_ID").expect("Missing TEAMS_CLIENT_ID");
    let client_secret = env::var("TEAMS_CLIENT_SECRET").expect("Missing TEAMS_CLIENT_SECRET");
    let client_tenant = env::var("TEAMS_TENANT_ID").expect("Missing TEAMS_TENANT_ID");
    let db_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL");

    let client = reqwest::Client::new();
    let teams_client = TeamsClient::new(client.clone(), &client_id, &client_secret);
    let graph_client = GraphClient::new(client, &client_id, &client_secret, &client_tenant);

    let pool = PgPool::connect_lazy(&db_url).expect("Failed to connect to the database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Unable to apply migration");

    let state = AppState {
        teams_client,
        graph_client,
        pool,
    };

    let app = Router::new()
        .route("/api/messages", post(message_route::handle))
        .with_state(state);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
