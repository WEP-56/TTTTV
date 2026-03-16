use async_trait::async_trait;
use serde_json::Value;

use crate::utils::error::{MoovieError, Result};
use super::super::models::{LivePlayQuality, LivePlayUrl, LiveRoomDetail, LiveRoomItem};
use super::LiveProvider;

pub struct DouyuProvider {
    client: reqwest::Client,
}

impl DouyuProvider {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    fn user_agent() -> &'static str {
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36"
    }

    fn parse_hot_num(s: &str) -> i64 {
        let s = s.trim();
        if s.is_empty() {
            return 0;
        }
        let is_wan = s.contains('万');
        let num_str = s.replace('万', "");
        if let Ok(mut num) = num_str.parse::<f64>() {
            if is_wan {
                num *= 10000.0;
            }
            return num.round() as i64;
        }
        s.parse::<i64>().unwrap_or(0)
    }

    fn random_hex(len: usize) -> String {
        use rand::RngCore;
        let mut bytes = vec![0u8; (len + 1) / 2];
        rand::thread_rng().fill_bytes(&mut bytes);
        let mut out = String::with_capacity(len);
        for b in bytes {
            out.push_str(&format!("{:02x}", b));
        }
        out.truncate(len);
        out
    }
}

#[async_trait]
impl LiveProvider for DouyuProvider {
    fn id(&self) -> &'static str {
        "douyu"
    }

    fn name(&self) -> &'static str {
        "斗鱼"
    }

    async fn recommend_rooms(&self, page: i32) -> Result<Vec<LiveRoomItem>> {
        let page = if page <= 0 { 1 } else { page };
        let url = format!("https://www.douyu.com/japi/weblist/apinc/allpage/6/{}", page);
        let json = self
            .client
            .get(url)
            .header("user-agent", Self::user_agent())
            .header("referer", "https://www.douyu.com/")
            .send()
            .await?
            .json::<Value>()
            .await?;

        if json["error"].as_i64().unwrap_or(0) != 0 {
            return Err(MoovieError::SourceSearchError(
                json["msg"].as_str().unwrap_or("斗鱼推荐获取失败").to_string(),
            ));
        }

        let mut items = Vec::new();
        let list = json["data"]["rl"].as_array().cloned().unwrap_or_default();
        for item in list {
            if item["type"].as_i64().unwrap_or(1) != 1 {
                continue;
            }
            items.push(LiveRoomItem {
                platform: self.id().to_string(),
                room_id: item["rid"].as_i64().unwrap_or(0).to_string(),
                title: item["rn"].as_str().unwrap_or("").to_string(),
                cover: item["rs16"].as_str().unwrap_or("").to_string(),
                user_name: item["nn"].as_str().unwrap_or("").to_string(),
                online: item["ol"].as_i64().unwrap_or(0),
            });
        }

        Ok(items)
    }

    async fn search_rooms(&self, keyword: &str, page: i32) -> Result<Vec<LiveRoomItem>> {
        if keyword.trim().is_empty() {
            return Ok(Vec::new());
        }
        let page = if page <= 0 { 1 } else { page };

        let did = Self::random_hex(32);
        let json = self
            .client
            .get("https://www.douyu.com/japi/search/api/searchShow")
            .query(&[
                ("kw", keyword),
                ("page", &page.to_string()),
                ("pageSize", "20"),
            ])
            .header("user-agent", Self::user_agent())
            .header("referer", "https://www.douyu.com/search/")
            .header("cookie", format!("dy_did={};acf_did={}", did, did))
            .send()
            .await?
            .json::<Value>()
            .await?;

        if json["error"].as_i64().unwrap_or(0) != 0 {
            return Err(MoovieError::SourceSearchError(
                json["msg"].as_str().unwrap_or("斗鱼搜索失败").to_string(),
            ));
        }

        let mut items = Vec::new();
        let list = json["data"]["relateShow"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        for item in list {
            items.push(LiveRoomItem {
                platform: self.id().to_string(),
                room_id: item["rid"].as_i64().unwrap_or(0).to_string(),
                title: item["roomName"].as_str().unwrap_or("").to_string(),
                cover: item["roomSrc"].as_str().unwrap_or("").to_string(),
                user_name: item["nickName"].as_str().unwrap_or("").to_string(),
                online: Self::parse_hot_num(item["hot"].as_str().unwrap_or("0")),
            });
        }

        Ok(items)
    }

    async fn room_detail(&self, room_id: &str) -> Result<LiveRoomDetail> {
        let room_id = room_id.trim();
        if room_id.is_empty() {
            return Err(MoovieError::InvalidParameter("room_id 不能为空".to_string()));
        }

        let room_text = self
            .client
            .get(format!("https://www.douyu.com/betard/{}", room_id))
            .header("user-agent", Self::user_agent())
            .header("referer", format!("https://www.douyu.com/{}", room_id))
            .send()
            .await?
            .text()
            .await?;

        let parsed: Value = serde_json::from_str(&room_text)
            .map_err(|e| MoovieError::DetailError(format!("斗鱼房间信息解析失败: {}", e)))?;

        let room_obj = if parsed.is_object() {
            parsed.get("room").cloned().unwrap_or(Value::Null)
        } else if let Some(inner) = parsed.as_str() {
            let parsed2: Value = serde_json::from_str(inner)
                .map_err(|e| MoovieError::DetailError(format!("斗鱼房间信息解析失败: {}", e)))?;
            parsed2.get("room").cloned().unwrap_or(Value::Null)
        } else {
            Value::Null
        };

        if room_obj.is_null() {
            return Err(MoovieError::DetailError("斗鱼房间信息解析失败".to_string()));
        }

        let h5_json = self
            .client
            .get(format!("https://www.douyu.com/swf_api/h5room/{}", room_id))
            .header("user-agent", Self::user_agent())
            .header("referer", format!("https://www.douyu.com/{}", room_id))
            .send()
            .await?
            .json::<Value>()
            .await?;
        let show_time = h5_json["data"]["show_time"]
            .as_str()
            .map(|s| s.to_string())
            .filter(|s| !s.trim().is_empty());

        let room_real_id = room_obj["room_id"]
            .as_str()
            .map(|s| s.to_string())
            .unwrap_or_else(|| room_id.to_string());

        let online = room_obj["room_biz_all"]["hot"]
            .as_i64()
            .or_else(|| room_obj["room_biz_all"]["hot"].as_str().map(|s| Self::parse_hot_num(s)))
            .unwrap_or(0);

        let show_status = room_obj["show_status"].as_i64().unwrap_or(0) == 1;
        let is_record = room_obj["videoLoop"].as_i64().unwrap_or(0) == 1;
        let status = show_status && !is_record;

        Ok(LiveRoomDetail {
            platform: self.id().to_string(),
            room_id: room_real_id.clone(),
            title: room_obj["room_name"].as_str().unwrap_or("").to_string(),
            cover: room_obj["room_pic"].as_str().unwrap_or("").to_string(),
            user_name: room_obj["owner_name"].as_str().unwrap_or("").to_string(),
            user_avatar: room_obj["owner_avatar"].as_str().unwrap_or("").to_string(),
            online,
            introduction: room_obj["show_details"]
                .as_str()
                .map(|s| s.to_string())
                .filter(|s| !s.trim().is_empty()),
            notice: None,
            status,
            is_record,
            url: format!("https://www.douyu.com/{}", room_real_id),
            show_time,
        })
    }

    async fn play_qualities(&self, _room_id: &str) -> Result<Vec<LivePlayQuality>> {
        Err(MoovieError::InvalidParameter(
            "斗鱼播放与画质选择开发中".to_string(),
        ))
    }

    async fn play_urls(&self, _room_id: &str, _quality_id: &str) -> Result<LivePlayUrl> {
        Err(MoovieError::InvalidParameter(
            "斗鱼播放与画质选择开发中".to_string(),
        ))
    }
}
