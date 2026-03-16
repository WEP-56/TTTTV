import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { apiClient } from '../api/client';
import type { LiveHistoryItem } from '../types';

export const useLiveHistoryStore = defineStore('liveHistory', () => {
  const history = ref<LiveHistoryItem[]>([]);
  const loading = ref(false);

  const hasHistory = computed(() => history.value.length > 0);

  async function fetchHistory() {
    loading.value = true;
    try {
      const res = await apiClient.getLiveHistory();
      if (res.success && res.data) {
        history.value = res.data;
      } else {
        history.value = [];
      }
    } catch (e) {
      console.error('获取直播历史失败:', e);
      history.value = [];
    } finally {
      loading.value = false;
    }
  }

  async function addHistory(item: {
    platform: string;
    room_id: string;
    title: string;
    cover?: string;
    user_name?: string;
    user_avatar?: string;
  }) {
    try {
      const res = await apiClient.addLiveHistory(item);
      if (res.success) {
        await fetchHistory();
        return true;
      }
      return false;
    } catch (e) {
      console.error('添加直播历史失败:', e);
      return false;
    }
  }

  async function deleteHistory(platform: string, roomId: string) {
    try {
      const res = await apiClient.deleteLiveHistory({ platform, room_id: roomId });
      if (res.success) {
        await fetchHistory();
        return true;
      }
      return false;
    } catch (e) {
      console.error('删除直播历史失败:', e);
      return false;
    }
  }

  async function clearHistory() {
    try {
      const res = await apiClient.clearLiveHistory();
      if (res.success) {
        await fetchHistory();
        return true;
      }
      return false;
    } catch (e) {
      console.error('清空直播历史失败:', e);
      return false;
    }
  }

  return {
    history,
    loading,
    hasHistory,
    fetchHistory,
    addHistory,
    deleteHistory,
    clearHistory,
  };
});

