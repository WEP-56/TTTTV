use async_trait::async_trait;
use regex::Regex;
use serde_json::Value;

use crate::utils::error::{MoovieError, Result};
use super::super::models::{LivePlayQuality, LivePlayUrl, LiveRoomDetail, LiveRoomItem};
use super::LiveProvider;

pub struct HuyaProvider {
    client: reqwest::Client,
}

impl HuyaProvider {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    fn user_agent() -> &'static str {
        // mimic a mobile UA similar to sample project
        "Mozilla/5.0 (Linux; Android 11; Pixel 5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.91 Mobile Safari/537.36"
    }

    async fn get_room_info_raw(&self, room_id: &str) -> Result<(Value, i64, i64)> {
        let html = self
            .client
            .get(format!("https://m.huya.com/{}", room_id))
            .header("user-agent", Self::user_agent())
            .send()
            .await?
            .text()
            .await?;

        let re = Regex::new(r"window\.HNF_GLOBAL_INIT\s*=\s*(\{[\s\S]*?\})[\s\S]*?</script>")
            .map_err(|e| MoovieError::ConfigError(e.to_string()))?;
        let caps = re
            .captures(&html)
            .ok_or_else(|| MoovieError::DetailError("虎牙房间页面解析失败".to_string()))?;
        let mut json_text = caps
            .get(1)
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();

        // remove function bodies that break JSON parsing
        let func_re = Regex::new(r"function.*?\(.*?\).*?\{[\s\S]*?\}")
            .map_err(|e| MoovieError::ConfigError(e.to_string()))?;
        json_text = func_re.replace_all(&json_text, "\"\"").to_string();

        let obj: Value = serde_json::from_str(&json_text)
            .map_err(|e| MoovieError::DetailError(format!("虎牙房间 JSON 解析失败: {}", e)))?;

        let top_re = Regex::new(r#"lChannelId":([0-9]+)"#)
            .map_err(|e| MoovieError::ConfigError(e.to_string()))?;
        let sub_re = Regex::new(r#"lSubChannelId":([0-9]+)"#)
            .map_err(|e| MoovieError::ConfigError(e.to_string()))?;
        let top_sid = top_re
            .captures(&html)
            .and_then(|c| c.get(1))
            .and_then(|m| m.as_str().parse::<i64>().ok())
            .unwrap_or(0);
        let sub_sid = sub_re
            .captures(&html)
            .and_then(|c| c.get(1))
            .and_then(|m| m.as_str().parse::<i64>().ok())
            .unwrap_or(0);

        Ok((obj, top_sid, sub_sid))
    }
}

#[async_trait]
impl LiveProvider for HuyaProvider {
    fn id(&self) -> &'static str {
        "huya"
    }

    fn name(&self) -> &'static str {
        "虎牙"
    }

    async fn recommend_rooms(&self, page: i32) -> Result<Vec<LiveRoomItem>> {
        let page = if page <= 0 { 1 } else { page };
        let text = self
            .client
            .get("https://www.huya.com/cache.php")
            .query(&[
                ("m", "LiveList"),
                ("do", "getLiveListByPage"),
                ("tagAll", "0"),
                ("page", &page.to_string()),
            ])
            .header("user-agent", Self::user_agent())
            .send()
            .await?
            .text()
            .await?;

        let json: Value = serde_json::from_str(&text)
            .map_err(|e| MoovieError::SourceSearchError(format!("虎牙推荐解析失败: {}", e)))?;

        let mut items = Vec::new();
        let list = json["data"]["datas"].as_array().cloned().unwrap_or_default();
        for item in list {
            let mut cover = item["screenshot"].as_str().unwrap_or("").to_string();
            if !cover.contains('?') {
                cover.push_str("?x-oss-process=style/w338_h190&");
            }
            let mut title = item["introduction"].as_str().unwrap_or("").to_string();
            if title.trim().is_empty() {
                title = item["roomName"].as_str().unwrap_or("").to_string();
            }
            items.push(LiveRoomItem {
                platform: self.id().to_string(),
                room_id: item["profileRoom"].as_i64().unwrap_or(0).to_string(),
                title,
                cover,
                user_name: item["nick"].as_str().unwrap_or("").to_string(),
                online: item["totalCount"]
                    .as_i64()
                    .or_else(|| item["totalCount"].as_str().and_then(|s| s.parse().ok()))
                    .unwrap_or(0),
            });
        }

        Ok(items)
    }

    async fn search_rooms(&self, keyword: &str, page: i32) -> Result<Vec<LiveRoomItem>> {
        if keyword.trim().is_empty() {
            return Ok(Vec::new());
        }
        let page = if page <= 0 { 1 } else { page };
        let start = (page - 1) * 20;

        let text = self
            .client
            .get("https://search.cdn.huya.com/")
            .query(&[
                ("m", "Search"),
                ("do", "getSearchContent"),
                ("q", keyword),
                ("uid", "0"),
                ("v", "4"),
                ("typ", "-5"),
                ("livestate", "0"),
                ("rows", "20"),
                ("start", &start.to_string()),
            ])
            .header("user-agent", Self::user_agent())
            .send()
            .await?
            .text()
            .await?;

        let json: Value = serde_json::from_str(&text)
            .map_err(|e| MoovieError::SourceSearchError(format!("虎牙搜索解析失败: {}", e)))?;

        let mut items = Vec::new();
        let docs = json["response"]["3"]["docs"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        for item in docs {
            let mut cover = item["game_screenshot"].as_str().unwrap_or("").to_string();
            if !cover.contains('?') {
                cover.push_str("?x-oss-process=style/w338_h190&");
            }
            let mut title = item["game_introduction"].as_str().unwrap_or("").to_string();
            if title.trim().is_empty() {
                title = item["game_roomName"].as_str().unwrap_or("").to_string();
            }

            items.push(LiveRoomItem {
                platform: self.id().to_string(),
                room_id: item["room_id"].as_i64().unwrap_or(0).to_string(),
                title,
                cover,
                user_name: item["game_nick"].as_str().unwrap_or("").to_string(),
                online: item["game_total_count"]
                    .as_i64()
                    .or_else(|| item["game_total_count"].as_str().and_then(|s| s.parse().ok()))
                    .unwrap_or(0),
            });
        }

        Ok(items)
    }

    async fn room_detail(&self, room_id: &str) -> Result<LiveRoomDetail> {
        let (obj, _top_sid, _sub_sid) = self.get_room_info_raw(room_id).await?;

        let t_live = &obj["roomInfo"]["tLiveInfo"];
        let t_profile = &obj["roomInfo"]["tProfileInfo"];

        let mut title = t_live["sIntroduction"].as_str().unwrap_or("").to_string();
        if title.trim().is_empty() {
            title = t_live["sRoomName"].as_str().unwrap_or("").to_string();
        }

        let cover = t_live["sScreenshot"].as_str().unwrap_or("").to_string();
        let online = t_live["lTotalCount"]
            .as_i64()
            .or_else(|| t_live["lTotalCount"].as_str().and_then(|s| s.parse().ok()))
            .unwrap_or(0);

        let profile_room = t_live["lProfileRoom"]
            .as_i64()
            .or_else(|| t_live["lProfileRoom"].as_str().and_then(|s| s.parse().ok()))
            .unwrap_or_else(|| room_id.parse::<i64>().unwrap_or(0))
            .to_string();

        let user_name = t_profile["sNick"].as_str().unwrap_or("").to_string();
        let user_avatar = t_profile["sAvatar180"].as_str().unwrap_or("").to_string();
        let notice = obj["welcomeText"].as_str().map(|s| s.to_string()).filter(|s| !s.trim().is_empty());
        let introduction = t_live["sIntroduction"].as_str().map(|s| s.to_string()).filter(|s| !s.trim().is_empty());
        let status = obj["roomInfo"]["eLiveStatus"].as_i64().unwrap_or(0) == 2;

        Ok(LiveRoomDetail {
            platform: self.id().to_string(),
            room_id: profile_room.clone(),
            title,
            cover,
            user_name,
            user_avatar,
            online,
            introduction,
            notice,
            status,
            is_record: false,
            url: format!("https://www.huya.com/{}", room_id),
            show_time: None,
        })
    }

    async fn play_qualities(&self, _room_id: &str) -> Result<Vec<LivePlayQuality>> {
        Err(MoovieError::InvalidParameter(
            "虎牙播放与画质选择开发中".to_string(),
        ))
    }

    async fn play_urls(&self, _room_id: &str, _quality_id: &str) -> Result<LivePlayUrl> {
        Err(MoovieError::InvalidParameter(
            "虎牙播放与画质选择开发中".to_string(),
        ))
    }
}

