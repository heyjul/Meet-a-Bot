use std::env;

use axum::{extract::State, routing::post, Json, Router};
use teams_api::{
    client::TeamsBotClient,
    models::{activity::Type, Activity},
};

#[derive(Clone)]
struct AppState {
    client: TeamsBotClient,
}

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::NEW)
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

    let state = AppState {
        client: TeamsBotClient::new(reqwest::Client::new(), &client_id, &client_secret),
    };

    let app = Router::new()
        .route("/api/messages", post(handle))
        .with_state(state);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[tracing::instrument(skip(client, activity))]
async fn handle(State(AppState { client }): State<AppState>, Json(activity): Json<Activity>) {
    let base_url = activity
        .service_url
        .as_deref()
        .unwrap_or("https://smba.trafficmanager.net/teams/");

    client
        .send_to_conversation(
            base_url,
            activity
                .conversation
                .as_ref()
                .unwrap()
                .id
                .as_deref()
                .unwrap(),
            &Activity {
                r#type: Some(Type::Message),
                text: Some("Salut !".to_owned()),
                from: activity.recipient.clone(),
                conversation: activity.conversation.clone(),
                recipient: activity.from.clone(),
                ..Default::default()
            },
        )
        .await;
}
