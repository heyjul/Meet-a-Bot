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

#[tracing::instrument(skip(client, activity), fields(activity = %activity))]
async fn handle(State(AppState { client }): State<AppState>, Json(activity): Json<Activity>) {
    if let Some(Type::ConversationUpdate) = activity.r#type {
        send_greetings(&client, &activity).await;
    }
}

#[tracing::instrument(skip_all)]
async fn send_greetings(client: &TeamsBotClient, activity: &Activity) {
    let members_added = &activity.members_added;
    let recipient = &activity.recipient;

    match (members_added, recipient) {
        (Some(ref members_added), Some(ref recipient)) => {
            if let Some(ref id) = recipient.id {
                if !members_added
                    .iter()
                    .any(|x| x.id.as_deref().map_or(false, |x| x == id))
                {
                    return;
                }
            } else {
                return;
            }
        }
        _ => return,
    }

    let (base_url, mut response) = activity.create_response();
    response.r#type = Some(Type::Message);
    response.text = Some("Salut".to_owned());

    if let Some(ref conversation) = activity.conversation {
        if let Some(ref conversation_id) = conversation.id {
            client
                .send_to_conversation(base_url, conversation_id, &response)
                .await;
        }
    }
}

/*

{
    "type": "AdaptiveCard",
    "$schema": "http://adaptivecards.io/schemas/adaptive-card.json",
    "version": "1.5",
    "body": [
        {
            "type": "TextBlock",
            "text": "Spank me daddy !",
            "wrap": true,
            "style": "heading",
            "color": "Accent"
        },
        {
            "type": "Input.Text",
            "placeholder": "Was I a good boy ?",
            "id": "comment",
            "isMultiline": true
        },
        {
            "type": "ActionSet",
            "actions": [
                {
                    "type": "Action.Submit",
                    "iconUrl": "https://external-content.duckduckgo.com/iu/?u=http%3A%2F%2Fpluspng.com%2Fimg-png%2Fstar-png-star-vector-png-transparent-image-2000.png&f=1&nofb=1&ipt=24e63d9cad1555be8ff2f18a33a1c1cefbe8d77a137ded45ad3125344c92aef0&ipo=images",
                    "id": "star1"
                },
                {
                    "type": "Action.Submit",
                    "iconUrl": "https://external-content.duckduckgo.com/iu/?u=http%3A%2F%2Fpluspng.com%2Fimg-png%2Fstar-png-star-vector-png-transparent-image-2000.png&f=1&nofb=1&ipt=24e63d9cad1555be8ff2f18a33a1c1cefbe8d77a137ded45ad3125344c92aef0&ipo=images",
                    "id": "star2"
                },
                {
                    "type": "Action.Submit",
                    "iconUrl": "https://external-content.duckduckgo.com/iu/?u=http%3A%2F%2Fpluspng.com%2Fimg-png%2Fstar-png-star-vector-png-transparent-image-2000.png&f=1&nofb=1&ipt=24e63d9cad1555be8ff2f18a33a1c1cefbe8d77a137ded45ad3125344c92aef0&ipo=images",
                    "id": "star3"
                },
                {
                    "type": "Action.Submit",
                    "iconUrl": "https://external-content.duckduckgo.com/iu/?u=http%3A%2F%2Fpluspng.com%2Fimg-png%2Fstar-png-star-vector-png-transparent-image-2000.png&f=1&nofb=1&ipt=24e63d9cad1555be8ff2f18a33a1c1cefbe8d77a137ded45ad3125344c92aef0&ipo=images",
                    "id": "star4"
                },
                {
                    "type": "Action.Submit",
                    "iconUrl": "https://external-content.duckduckgo.com/iu/?u=http%3A%2F%2Fpluspng.com%2Fimg-png%2Fstar-png-star-vector-png-transparent-image-2000.png&f=1&nofb=1&ipt=24e63d9cad1555be8ff2f18a33a1c1cefbe8d77a137ded45ad3125344c92aef0&ipo=images",
                    "id": "star5"
                }
            ]
        }
    ]
}

*/
