use axum::{Json, Router, extract::Query, http::StatusCode, routing::get};
use chrono::{DateTime, Utc};
use mazeh::generators::dfs::{GenAlgorithm, Maze};
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use tracing::{error, warn};

use std::sync::LazyLock;

const MAX_BOUND: usize = 10000;

static SERVER_START_TIME: LazyLock<DateTime<Utc>> = LazyLock::new(Utc::now);

#[derive(Deserialize)]
struct MazeRequest {
    algorithm: GenAlgorithm,
    height: usize,
    width: usize,
}

#[derive(Serialize)]
struct ServerHealth {
    start_time: String,
    running_time: i64,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let app = Router::new()
        .route("/health", get(health))
        .route("/algorithms", get(algorithms))
        .route("/maze", get(generate))
        .layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> Json<ServerHealth> {
    Json(ServerHealth {
        start_time: SERVER_START_TIME.format("%d/%m/%y %H:%M:%S").to_string(),
        running_time: (Utc::now() - *SERVER_START_TIME).num_seconds(),
    })
}

async fn algorithms() -> Json<Vec<GenAlgorithm>> {
    Json(GenAlgorithm::all().to_vec())
}

async fn generate(
    Query(MazeRequest {
        algorithm,
        height,
        width,
    }): Query<MazeRequest>,
) -> Result<Json<Maze>, StatusCode> {
    if width == 0 || height == 0 || width > MAX_BOUND || height > MAX_BOUND {
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut maze: Maze = Maze::new(height, width);
    algorithm.generate_static(&mut maze);
    Ok(Json(maze))
}
