use std::sync::Arc;

use axum::{
    extract::State,
    routing::{get, patch},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, sync::Mutex};
use tracing::{info, instrument, level_filters::LevelFilter};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    name: String,
    age: u8,
    skills: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserUpdate {
    name: Option<String>,
    age: Option<u8>,
    skills: Option<Vec<String>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let console = fmt::Layer::new()
        .pretty()
        .with_span_events(FmtSpan::CLOSE)
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry().with(console).init();

    let user = User {
        name: "Alice".to_string(),
        age: 30,
        skills: vec!["Rust".to_string(), "Python".to_string()],
    };

    let user = Arc::new(Mutex::new(user));

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on: {}", addr);

    let app = Router::new()
        .route("/", get(user_handler))
        .route("/", patch(update_handler))
        .with_state(user);

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[instrument]
async fn user_handler(State(user): State<Arc<Mutex<User>>>) -> Json<User> {
    let user = user.lock().await;
    Json(user.clone())
}

#[instrument]
async fn update_handler(
    State(user): State<Arc<Mutex<User>>>,
    Json(payload): Json<UserUpdate>,
) -> Json<User> {
    let mut user = user.lock().await;
    if let Some(age) = payload.age {
        user.age = age;
    }
    if let Some(name) = payload.name {
        user.name = name;
    }
    if let Some(skills) = payload.skills {
        user.skills = skills;
    }
    user.clone().into()
}
