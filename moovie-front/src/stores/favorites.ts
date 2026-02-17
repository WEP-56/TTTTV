import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { apiClient } from '../api/client';

export interface FavoriteItem {
  vod_id: string;
  source_key: string;
  vod_name: string;
  vod_pic: string | null;
  vod_remarks: string | null;
  vod_actor: string | null;
  vod_director: string | null;
  vod_content: string | null;
  created_time: number;
}

export const useFavoritesStore = defineStore('favorites', () => {
  const favorites = ref<FavoriteItem[]>([]);
  const loading = ref(false);

  const hasFavorites = computed(() => favorites.value.length > 0);

  async function fetchFavorites() {
    try {
      loading.value = true;
      const response = await apiClient.get('/api/favorites');
      const apiResponse = response.data;
      if (apiResponse.success && apiResponse.data) {
        favorites.value = apiResponse.data;
      } else {
        favorites.value = [];
      }
    } catch (error) {
      console.error('获取收藏失败:', error);
      favorites.value = [];
    } finally {
      loading.value = false;
    }
  }

  async function addFavorite(item: {
    vod_id: string;
    source_key: string;
    vod_name: string;
    vod_pic?: string | null;
    vod_remarks?: string | null;
    vod_actor?: string | null;
    vod_director?: string | null;
    vod_content?: string | null;
  }) {
    try {
      const response = await apiClient.post('/api/favorites', item);
      if (response.data.success) {
        await fetchFavorites();
        return true;
      }
      return false;
    } catch (error) {
      console.error('添加收藏失败:', error);
      return false;
    }
  }

  async function deleteFavorite(vodId: string, sourceKey: string) {
    try {
      const response = await apiClient.delete('/api/favorites', {
        params: {
          vod_id: vodId,
          source_key: sourceKey
        }
      });
      if (response.data.success) {
        await fetchFavorites();
        return true;
      }
      return false;
    } catch (error) {
      console.error('删除收藏失败:', error);
      return false;
    }
  }

  async function checkFavorite(vodId: string, sourceKey: string): Promise<boolean> {
    try {
      const response = await apiClient.get('/api/favorites/check', {
        params: {
          vod_id: vodId,
          source_key: sourceKey
        }
      });
      if (response.data.success && response.data.data) {
        return response.data.data.is_favorited;
      }
      return false;
    } catch (error) {
      console.error('检查收藏状态失败:', error);
      return false;
    }
  }

  async function clearFavorites() {
    try {
      const response = await apiClient.delete('/api/favorites/clear');
      if (response.data.success) {
        await fetchFavorites();
        return true;
      }
      return false;
    } catch (error) {
      console.error('清除收藏失败:', error);
      return false;
    }
  }

  function isFavoriteLocally(vodId: string, sourceKey: string): boolean {
    return favorites.value.some(item => 
      item.vod_id === vodId && item.source_key === sourceKey
    );
  }

  return {
    favorites,
    loading,
    hasFavorites,
    fetchFavorites,
    addFavorite,
    deleteFavorite,
    checkFavorite,
    clearFavorites,
    isFavoriteLocally
  };
});