import axios from 'axios';
import type {
  VodItem,
  SiteWithStatus,
  SearchResult,
  ApiResponse,
  RemoteSourcesResponse,
  RemoteSource,
  AddSourcesBatchResult,
} from '../types';

const API_BASE = 'http://127.0.0.1:5007';

const api = axios.create({
  baseURL: API_BASE,
  timeout: 30000,
});

export interface PlayResult {
  url: string;
  headers?: Record<string, string>;
}

export interface DoubanSubject {
  id?: string;
  title: string;
  cover?: string;
  cover_url?: string;
  rate?: string;
  year?: string;
  url?: string;
}

export interface DoubanSearchResponse {
  subjects: DoubanSubject[];
  total?: number;
}

export const apiClient = {
  get: api.get.bind(api),
  post: api.post.bind(api),
  delete: api.delete.bind(api),
  
  async health(): Promise<ApiResponse<any>> {
    const res = await api.get('/health');
    return res.data;
  },

  async search(keyword: string, bypass = false): Promise<ApiResponse<SearchResult>> {
    const res = await api.get('/api/search', {
      params: { kw: keyword, bypass },
    });
    return res.data;
  },

  async getDetail(sourceKey: string, vodId: string): Promise<ApiResponse<VodItem>> {
    const res = await api.get('/api/detail', {
      params: { source_key: sourceKey, vod_id: vodId },
    });
    return res.data;
  },

  async parsePlayUrl(playUrl: string): Promise<ApiResponse<PlayResult>> {
    const res = await api.get('/api/play/parse', {
      params: { play_url: playUrl },
    });
    return res.data;
  },

  async getSites(): Promise<ApiResponse<SiteWithStatus[]>> {
    const res = await api.get('/api/sources');
    return res.data;
  },

  async toggleSite(key: string, enabled: boolean): Promise<ApiResponse<void>> {
    const res = await api.post('/api/sources/toggle', null, {
      params: { key, enabled },
    });
    return res.data;
  },

  async checkSites(key?: string): Promise<ApiResponse<SiteWithStatus[]>> {
    const res = await api.get('/api/sources/check', {
      params: key ? { key } : {},
    });
    return res.data;
  },

  async addSource(source: {
    key: string;
    name: string;
    api: string;
    detail: string;
    group?: string;
    r18?: boolean;
    comment?: string;
  }): Promise<ApiResponse<void>> {
    const res = await api.post('/api/sources/add', source);
    return res.data;
  },

  async addSourcesBatch(sources: RemoteSource[]): Promise<ApiResponse<AddSourcesBatchResult>> {
    const res = await api.post('/api/sources/add_batch', sources);
    return res.data;
  },

  async deleteSource(key: string): Promise<ApiResponse<void>> {
    const res = await api.delete('/api/sources/delete', {
      params: { key },
    });
    return res.data;
  },

  async getRemoteSources(params?: { url?: string }): Promise<ApiResponse<RemoteSourcesResponse>> {
    const res = await api.get('/api/sources/remote', { params });
    return res.data;
  },

  async doubanSearch(params: {
    type?: string;
    tag?: string;
    sort?: string;
    page_limit?: number;
    page_start?: number;
    start?: string;
    range?: string;
    genres?: string;
    countries?: string;
    tags?: string;
  }): Promise<ApiResponse<DoubanSearchResponse>> {
    const res = await api.get('/api/douban/search', { params });
    return res.data;
  },

  async doubanChart(params: {
    type?: string;
    interval_id?: string;
    action?: string;
    start?: string;
    limit?: string;
  }): Promise<ApiResponse<DoubanSearchResponse>> {
    const res = await api.get('/api/douban/chart', { params });
    return res.data;
  },
};
