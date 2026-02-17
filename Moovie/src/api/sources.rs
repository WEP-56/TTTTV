use axum::{
    extract::{State, Query},
    routing::{get, post, delete},
    Json, Router,
};
use crate::core::{AppState, SiteWithStatus};
use crate::utils::response::{ApiResponse, ApiResult};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ToggleSiteQuery {
    pub key: String,
    pub enabled: bool,
}

#[derive(Deserialize)]
pub struct CheckSiteQuery {
    pub key: Option<String>,
}

#[derive(Deserialize)]
pub struct AddSourceRequest {
    pub key: String,
    pub name: String,
    pub api: String,
    pub detail: String,
    pub group: Option<String>,
    pub r18: Option<bool>,
}

#[derive(Deserialize)]
pub struct DeleteSourceRequest {
    pub key: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_sites))
        .route("/toggle", get(toggle_site))
        .route("/check", get(check_sites))
        .route("/add", post(add_source))
        .route("/delete", delete(delete_source))
}

pub async fn get_sites(
    State(state): State<AppState>,
) -> ApiResult<Vec<SiteWithStatus>> {
    let sites = state.get_all_sites();
    Ok(Json(ApiResponse::success(sites)))
}

pub async fn toggle_site(
    State(state): State<AppState>,
    Query(query): Query<ToggleSiteQuery>,
) -> ApiResult<()> {
    state.set_site_enabled(&query.key, query.enabled)?;
    Ok(Json(ApiResponse::success(())))
}

pub async fn check_sites(
    State(state): State<AppState>,
    Query(query): Query<CheckSiteQuery>,
) -> ApiResult<Vec<SiteWithStatus>> {
    let mut sites = state.get_all_sites();
    
    if let Some(key) = query.key {
        sites.retain(|s| s.key == key);
    }

    Ok(Json(ApiResponse::success(sites)))
}

pub async fn add_source(
    State(state): State<AppState>,
    Json(request): Json<AddSourceRequest>,
) -> ApiResult<()> {
    state.add_custom_source(request)?;
    Ok(Json(ApiResponse::success(())))
}

pub async fn delete_source(
    State(state): State<AppState>,
    Query(query): Query<DeleteSourceRequest>,
) -> ApiResult<()> {
    state.delete_custom_source(&query.key)?;
    Ok(Json(ApiResponse::success(())))
}
