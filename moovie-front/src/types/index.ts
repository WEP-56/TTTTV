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
