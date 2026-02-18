mod api;
mod core;
mod models;
mod services;
mod utils;

use axum::{
    routing::{get, post},
    Router,
};
use core::{AppState, SourceConfig, LocalStorage};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;
use directories::ProjectDirs;

const DEFAULT_SOURCES_JSON: &str = include_str!("../config/sources.json");

fn get_config_path() -> PathBuf {
    let exe_path = std::env::current_exe().ok();
    let exe_dir = exe_path.as_ref().and_then(|p| p.parent());

    // 1. Try relative to CWD (development mode)
    let cwd_config = PathBuf::from("config/sources.json");
    if cwd_config.exists() {
        return cwd_config;
    }

    // 2. Try relative to executable (portable/sidecar)
    if let Some(dir) = exe_dir {
        let sibling_config = dir.join("config/sources.json");
        if sibling_config.exists() {
            return sibling_config;
        }
        
        // 3. Try resources directory (Tauri bundle structure)
        let resources_config = dir.join("../resources/config/sources.json"); 
        if resources_config.exists() {
            return resources_config;
        }

        let resources_config_flat = dir.join("resources/config/sources.json");
        if resources_config_flat.exists() {
            return resources_config_flat;
        }
    }

    // 4. Try user config dir (AppData) - THIS IS THE SAFE FALLBACK
    if let Some(proj_dirs) = ProjectDirs::from("com", "ttttv", "app") {
        let config_dir = proj_dirs.config_dir();
        // Ensure config dir exists
        if !config_dir.exists() {
            std::fs::create_dir_all(config_dir).ok();
        }
        return config_dir.join("sources.json");
    }

    // Final fallback
    PathBuf::from("config/sources.json")
}

fn get_storage_path() -> PathBuf {
    // 1. Try local data folder first (portable mode preference)
    let local_data = PathBuf::from("data/storage.json");
    if local_data.exists() {
        return local_data;
    }

    // 2. Use system data directory (AppData/Roaming/...)
    if let Some(proj_dirs) = ProjectDirs::from("com", "ttttv", "app") {
        let data_dir = proj_dirs.data_dir();
        if !data_dir.exists() {
            std::fs::create_dir_all(data_dir).ok();
        }
        return data_dir.join("storage.json");
    }

    // Fallback to local
    local_data
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config_path = get_config_path();
    tracing::info!("Using config path: {:?}", config_path);

    let source_config = match SourceConfig::load_from_file(&config_path) {
        Ok(config) => config,
        Err(e) => {
            tracing::warn!("Failed to load config from {:?}: {}", config_path, e);
            tracing::info!("Creating default config at {:?}", config_path);
            
            // Ensure parent directory exists
            if let Some(parent) = config_path.parent() {
                if !parent.exists() {
                    let _ = std::fs::create_dir_all(parent);
                }
            }

            // Write default config
            if let Err(write_err) = std::fs::write(&config_path, DEFAULT_SOURCES_JSON) {
                tracing::error!("Failed to write default config: {}", write_err);
            }

            // Load default from memory
            serde_json::from_str(DEFAULT_SOURCES_JSON).expect("Default config is invalid")
        }
    };

    tracing::info!("加载了 {} 个资源站配置", source_config.api_site.len());

    let storage_path = get_storage_path();
    tracing::info!("Using storage path: {:?}", storage_path);

    // Ensure directory exists for storage
    if let Some(parent) = storage_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).expect("无法创建数据目录");
        }
    }

    let storage = LocalStorage::new(storage_path.clone())
        .expect("无法初始化本地存储");

    let sites = source_config.to_sites();
    let enabled_count = sites.iter().filter(|s| s.enabled).count();
    tracing::info!("启用了 {} 个资源站", enabled_count);

    let app_state = AppState::new(sites, source_config, storage, config_path).await;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(api::health::health_check))
        .route("/api/search", get(api::search::search))
        .route("/api/detail", get(api::search::get_detail))
        .route("/api/play/parse", get(api::play::parse_play_url))
        .nest("/api/sources", api::sources::router())
        .nest("/api/history", api::history::router())
        .nest("/api/favorites", api::favorites::router())
        .route("/api/douban/search", get(api::douban::douban_search))
        .route("/api/douban/chart", get(api::douban::douban_chart_top_list))
        .layer(cors)
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 5007));
    
    tracing::info!("服务器启动于 http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
