use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use tokio::sync::Mutex;

use super::board::Board;

#[derive(Clone)]
pub struct AppState {
    board: Arc<Mutex<Board>>,
}

fn new_board() -> Board {
    let mut board = Board::new();
    board.reset();
    board
}

pub fn get_routes() -> Router {
    let board = AppState {
        board: Arc::new(Mutex::new(new_board())),
    };

    Router::new()
        .route("/12/board", get(show_board))
        .route("/12/reset", post(reset_board))
        .with_state(Arc::new(board))
}

pub async fn show_board(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let board = state.board.lock().await;
    println!("show board");

    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(board.to_string()))
        .unwrap()
}

pub async fn reset_board(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut lock = state.board.lock().await;
    *lock = new_board();

    println!("reset board");

    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(lock.to_string()))
        .unwrap()
}
