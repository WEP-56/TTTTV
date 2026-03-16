export interface VodItem {
  source_key: string;
  vod_id: string;
  vod_name: string;
  vod_pic?: string;
  vod_play_url?: string;
  vod_remarks?: string;
  vod_actor?: string;
  vod_director?: string;
  vod_content?: string;
  avg_speed_ms?: number;
}

export interface SiteWithStatus {
  key: string;
  name: string;
  base_url: string;
  enabled: boolean;
  last_check?: number;
  is_healthy?: boolean;
  comment?: string;
  r18?: boolean;
  group?: string;
}

export interface RemoteSource {
  key: string;
  name: string;
  api: string;
  detail: string;
  group?: string;
  r18?: boolean;
  comment?: string;
}

export interface RemoteSourcesResponse {
  url: string;
  sources: RemoteSource[];
}

export interface AddSourcesBatchFailure {
  key: string;
  error: string;
}

export interface AddSourcesBatchResult {
  added: string[];
  skipped_existing: string[];
  failed: AddSourcesBatchFailure[];
}

export interface LivePlatformInfo {
  id: string;
  name: string;
}

export interface LiveRoomItem {
  platform: string;
  room_id: string;
  title: string;
  cover: string;
  user_name: string;
  online: number;
}

export interface LiveRoomDetail {
  platform: string;
  room_id: string;
  title: string;
  cover: string;
  user_name: string;
  user_avatar: string;
  online: number;
  introduction?: string;
  notice?: string;
  status: boolean;
  is_record: boolean;
  url: string;
  show_time?: string;
}

export interface LivePlayQuality {
  id: string;
  name: string;
  sort: number;
}

export interface LivePlayUrl {
  urls: string[];
  headers?: Record<string, string>;
  url_type?: string;
  expires_at?: number;
}

export interface LiveHistoryItem {
  platform: string;
  room_id: string;
  title: string;
  cover?: string;
  user_name?: string;
  user_avatar?: string;
  last_watch_time: number;
}

export interface LiveFavoriteItem {
  platform: string;
  room_id: string;
  title: string;
  cover?: string;
  user_name?: string;
  user_avatar?: string;
  created_time: number;
}

export interface LiveMessageColor {
  r: number;
  g: number;
  b: number;
}

export type LiveMessageType = 'chat' | 'gift' | 'online' | 'superChat';

export interface LiveMessage {
  type: LiveMessageType;
  user_name: string;
  message: string;
  color: LiveMessageColor;
  data?: any;
}

export interface BilibiliAuthStatusResponse {
  logged_in: boolean;
}

export interface BilibiliQrCodeResponse {
  qrcode_key: string;
  url: string;
  svg: string;
}

export interface BilibiliQrPollResponse {
  code: number;
  status: string;
  message: string;
}

export interface SearchResult {
  items: VodItem[];
  filtered_count: number;
}

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  message?: string;
  error?: string;
}
