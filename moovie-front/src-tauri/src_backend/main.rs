mod api;
mod core;
mod models;
mod services;
mod utils;

use axum::{
    routing::get,
    Router,
};
use core::{AppState, SourceConfig};
use std::net::SocketAddr;
use std::path::PathBuf;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config_path = PathBuf::from("config/sources.json");
    let source_config = SourceConfig::load_from_file(&config_path)
        .expect("无法加载资源配置文件");

    tracing::info!("加载了 {} 个资源站配置", source_config.api_site.len());

    let sites = source_config.to_sites();
    let enabled_count = sites.iter().filter(|s| s.enabled).count();
    tracing::info!("启用了 {} 个资源站", enabled_count);

    let app_state = AppState::new(sites).await;

    let app = Router::new()
        .route("/health", get(api::health::health_check))
        .route("/api/search", get(api::search::search))
        .route("/api/detail", get(api::search::get_detail))
        .route("/api/play/parse", get(api::play::parse_play_url))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 5007));
    
    tracing::info!("服务器启动于 http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

