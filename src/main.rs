use std::sync::{Arc, Mutex};

use axum::{
    extract::State,
    http::Response,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router, Server,
};
use sysinfo::{CpuExt, System, SystemExt};

#[tokio::main]
async fn main() {
    let app_state = AppState::default();

    let router = Router::new()
        .route("/", get(root_get))
        .route("/index.mjs", get(indexmjs_get))
        .route("/index.css", get(indexcss_get))
        .route("/api/cpus", get(cpus_get))
        .with_state(app_state.clone());
    let app_state_for_bg = app_state.clone();

    // Update CPU usage in the background. spawn_blocking receive a functions as parameter that can block
    tokio::task::spawn_blocking(move || {
        let mut sys = System::new();
        loop {
            sys.refresh_cpu();
            let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
            {
                let mut cpus = app_state_for_bg.cpus.lock().unwrap();
                *cpus = v;
            }
            std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
        }
    });

    let server = Server::bind(&"0.0.0.0:7032".parse().unwrap()).serve(router.into_make_service());
    let addr = server.local_addr();
    println!("Listening #{addr}");
    server.await.unwrap();
}

#[derive(Clone, Default)]
struct AppState {
    cpus: Arc<Mutex<Vec<f32>>>,
}

#[axum::debug_handler]
async fn root_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.html").await.unwrap();
    Html(markup)
}

#[axum::debug_handler]
async fn indexcss_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.css").await.unwrap();

    Response::builder()
        .header("content-type", "text/css;charset=utf-8")
        .body(markup)
        .unwrap()
}

#[axum::debug_handler]
async fn indexmjs_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.mjs").await.unwrap();

    Response::builder()
        .header("content-type", "application/javascript;charset=utf-8")
        .body(markup)
        .unwrap()
}

#[axum::debug_handler]
async fn cpus_get(State(state): State<AppState>) -> impl IntoResponse {
    let v = state.cpus.lock().unwrap().clone();
    Json(v)
}
