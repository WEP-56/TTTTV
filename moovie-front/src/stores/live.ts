import { defineStore } from 'pinia';
import { ref } from 'vue';
import { apiClient } from '../api/client';
import type { LivePlatformInfo, LivePlayQuality, LivePlayUrl, LiveRoomDetail, LiveRoomItem } from '../types';

const API_BASE = 'http://127.0.0.1:5007';

export const useLiveStore = defineStore('live', () => {
  const platforms = ref<LivePlatformInfo[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  function getApiErrorMessage(err: any): string {
    return (
      err?.response?.data?.error ||
      err?.response?.data?.message ||
      err?.message ||
      '未知错误'
    );
  }

  async function loadPlatforms() {
    try {
      const res = await apiClient.getLivePlatforms();
      if (res.success && res.data) {
        platforms.value = res.data;
      }
    } catch (e) {
      console.error('加载直播平台失败:', e);
    }
  }

  async function recommend(platform: string, page = 1): Promise<LiveRoomItem[]> {
    loading.value = true;
    error.value = null;
    try {
      const res = await apiClient.liveRecommend(platform, page);
      if (res.success && res.data) return res.data;
      error.value = res.error || res.message || '获取推荐失败';
      return [];
    } catch (e: any) {
      console.error('获取推荐失败:', e);
      error.value = getApiErrorMessage(e) || '获取推荐失败';
      return [];
    } finally {
      loading.value = false;
    }
  }

  async function search(platform: string, kw: string, page = 1): Promise<LiveRoomItem[]> {
    loading.value = true;
    error.value = null;
    try {
      const res = await apiClient.liveSearch(platform, kw, page);
      if (res.success && res.data) return res.data;
      error.value = res.error || res.message || '搜索失败';
      return [];
    } catch (e: any) {
      console.error('搜索失败:', e);
      error.value = getApiErrorMessage(e) || '搜索失败';
      return [];
    } finally {
      loading.value = false;
    }
  }

  async function getDetail(platform: string, roomId: string): Promise<LiveRoomDetail | null> {
    try {
      const res = await apiClient.liveRoomDetail(platform, roomId);
      if (res.success && res.data) return res.data;
      return null;
    } catch (e) {
      console.error('获取直播间详情失败:', e);
      return null;
    }
  }

  async function getQualities(platform: string, roomId: string): Promise<LivePlayQuality[]> {
    try {
      const res = await apiClient.liveQualities(platform, roomId);
      if (res.success && res.data) return res.data;
      return [];
    } catch (e) {
      console.error('获取清晰度失败:', e);
      return [];
    }
  }

  async function getPlay(platform: string, roomId: string, qualityId: string): Promise<LivePlayUrl | null> {
    try {
      const res = await apiClient.livePlay(platform, roomId, qualityId);
      if (res.success && res.data) return res.data;
      return null;
    } catch (e) {
      console.error('获取播放地址失败:', e);
      return null;
    }
  }

  function toProxyUrl(platform: string, url: string): string {
    const encoded = encodeURIComponent(url);
    return `${API_BASE}/api/live/proxy?platform=${encodeURIComponent(platform)}&url=${encoded}`;
  }

  function danmakuWsUrl(platform: string, roomId: string): string {
    const params = new URLSearchParams({ room_id: roomId });
    return `ws://127.0.0.1:5007/api/live/${encodeURIComponent(platform)}/room/danmaku?${params.toString()}`;
  }

  return {
    platforms,
    loading,
    error,
    loadPlatforms,
    recommend,
    search,
    getDetail,
    getQualities,
    getPlay,
    toProxyUrl,
    danmakuWsUrl,
  };
});
