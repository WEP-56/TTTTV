import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { apiClient } from '../api/client';

export interface WatchHistoryItem {
  vod_id: string;
  source_key: string;
  vod_name: string;
  vod_pic: string | null;
  last_play_time: number;
  progress: number;
  episode: string | null;
}

export const useHistoryStore = defineStore('history', () => {
  const history = ref<WatchHistoryItem[]>([]);
  const loading = ref(false);

  const hasHistory = computed(() => history.value.length > 0);

  async function fetchHistory() {
    try {
      loading.value = true;
      const response = await apiClient.get('/api/history');
      const apiResponse = response.data;
      if (apiResponse.success && apiResponse.data) {
        history.value = apiResponse.data;
      } else {
        history.value = [];
      }
    } catch (error) {
      console.error('获取观影历史失败:', error);
      history.value = [];
    } finally {
      loading.value = false;
    }
  }

  async function addHistory(item: {
    vod_id: string;
    source_key: string;
    vod_name: string;
    vod_pic?: string | null;
    progress: number;
    episode?: string | null;
  }) {
    try {
      const response = await apiClient.post('/api/history', {
        vod_id: item.vod_id,
        source_key: item.source_key,
        vod_name: item.vod_name,
        vod_pic: item.vod_pic,
        progress: item.progress,
        episode: item.episode
      });
      if (response.data.success) {
        await fetchHistory();
      }
    } catch (error) {
      console.error('保存观影历史失败:', error);
    }
  }

  async function deleteHistory(vodId: string, sourceKey: string) {
    try {
      const response = await apiClient.delete('/api/history', {
        params: {
          vod_id: vodId,
          source_key: sourceKey
        }
      });
      if (response.data.success) {
        await fetchHistory();
      }
    } catch (error) {
      console.error('删除观影历史失败:', error);
    }
  }

  async function clearHistory() {
    try {
      const response = await apiClient.delete('/api/history/clear');
      if (response.data.success) {
        await fetchHistory();
      }
    } catch (error) {
      console.error('清除观影历史失败:', error);
    }
  }

  function getHistoryByVodId(vodId: string, sourceKey?: string) {
    return history.value.find(item => {
      if (sourceKey) {
        return item.vod_id === vodId && item.source_key === sourceKey;
      }
      return item.vod_id === vodId;
    });
  }

  return {
    history,
    loading,
    hasHistory,
    fetchHistory,
    addHistory,
    deleteHistory,
    clearHistory,
    getHistoryByVodId
  };
});
