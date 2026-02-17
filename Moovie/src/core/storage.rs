use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use crate::utils::error::{Result, MoovieError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteState {
    pub enabled: bool,
    pub last_check: Option<i64>,
    pub is_healthy: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchHistoryItem {
    pub vod_id: String,
    pub source_key: String,
    pub vod_name: String,
    pub vod_pic: Option<String>,
    pub last_play_time: i64,
    pub progress: f64,
    pub episode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteItem {
    pub vod_id: String,
    pub source_key: String,
    pub vod_name: String,
    pub vod_pic: Option<String>,
    pub vod_remarks: Option<String>,
    pub vod_actor: Option<String>,
    pub vod_director: Option<String>,
    pub vod_content: Option<String>,
    pub created_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageData {
    pub site_states: HashMap<String, SiteState>,
    pub watch_history: Vec<WatchHistoryItem>,
    pub favorites: Vec<FavoriteItem>,
}

impl Default for StorageData {
    fn default() -> Self {
        StorageData {
            site_states: HashMap::new(),
            watch_history: Vec::new(),
            favorites: Vec::new(),
        }
    }
}

pub struct LocalStorage {
    path: PathBuf,
    data: StorageData,
}

impl LocalStorage {
    pub fn new(path: PathBuf) -> Result<Self> {
        let data = if path.exists() {
            let content = std::fs::read_to_string(&path)
                .map_err(|e| MoovieError::IoError(e))?;
            serde_json::from_str(&content)
                .map_err(|e| MoovieError::JsonError(e))?
        } else {
            StorageData::default()
        };

        Ok(LocalStorage { path, data })
    }

    pub fn save(&self) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| MoovieError::IoError(e))?;
        }
        let content = serde_json::to_string_pretty(&self.data)
            .map_err(|e| MoovieError::JsonError(e))?;
        std::fs::write(&self.path, content)
            .map_err(|e| MoovieError::IoError(e))?;
        Ok(())
    }

    pub fn get_site_state(&self, key: &str) -> SiteState {
        self.data.site_states.get(key).cloned().unwrap_or(SiteState {
            enabled: true,
            last_check: None,
            is_healthy: None,
        })
    }

    pub fn set_site_state(&mut self, key: &str, state: SiteState) -> Result<()> {
        self.data.site_states.insert(key.to_string(), state);
        self.save()
    }

    pub fn get_all_site_states(&self) -> &HashMap<String, SiteState> {
        &self.data.site_states
    }

    pub fn add_watch_history(&mut self, item: WatchHistoryItem) -> Result<()> {
        self.data.watch_history.retain(|h| h.vod_id != item.vod_id || h.source_key != item.source_key);
        self.data.watch_history.insert(0, item);
        self.data.watch_history.truncate(100);
        self.save()
    }

    pub fn remove_watch_history(&mut self, vod_id: &str, source_key: &str) -> Result<()> {
        self.data.watch_history.retain(|h| h.vod_id != vod_id || h.source_key != source_key);
        self.save()
    }

    pub fn clear_watch_history(&mut self) -> Result<()> {
        self.data.watch_history.clear();
        self.save()
    }

    pub fn get_watch_history(&self) -> &[WatchHistoryItem] {
        &self.data.watch_history
    }

    pub fn add_favorite(&mut self, item: FavoriteItem) -> Result<()> {
        self.data.favorites.retain(|f| f.vod_id != item.vod_id || f.source_key != item.source_key);
        self.data.favorites.insert(0, item);
        self.save()
    }

    pub fn remove_favorite(&mut self, vod_id: &str, source_key: &str) -> Result<()> {
        self.data.favorites.retain(|f| f.vod_id != vod_id || f.source_key != source_key);
        self.save()
    }

    pub fn clear_favorites(&mut self) -> Result<()> {
        self.data.favorites.clear();
        self.save()
    }

    pub fn is_favorited(&self, vod_id: &str, source_key: &str) -> bool {
        self.data.favorites.iter().any(|f| f.vod_id == vod_id && f.source_key == source_key)
    }

    pub fn get_favorites(&self) -> &[FavoriteItem] {
        &self.data.favorites
    }
}
