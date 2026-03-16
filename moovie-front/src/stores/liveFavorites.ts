import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { apiClient } from '../api/client';
import type { LiveFavoriteItem } from '../types';

export const useLiveFavoritesStore = defineStore('liveFavorites', () => {
  const favorites = ref<LiveFavoriteItem[]>([]);
  const loading = ref(false);

  const hasFavorites = computed(() => favorites.value.length > 0);

  async function fetchFavorites() {
    loading.value = true;
    try {
      const res = await apiClient.getLiveFavorites();
      if (res.success && res.data) {
        favorites.value = res.data;
      } else {
        favorites.value = [];
      }
    } catch (e) {
      console.error('获取直播收藏失败:', e);
      favorites.value = [];
    } finally {
      loading.value = false;
    }
  }

  async function addFavorite(item: {
    platform: string;
    room_id: string;
    title: string;
    cover?: string;
    user_name?: string;
    user_avatar?: string;
  }) {
    try {
      const res = await apiClient.addLiveFavorite(item);
      if (res.success) {
        await fetchFavorites();
        return true;
      }
      return false;
    } catch (e) {
      console.error('添加直播收藏失败:', e);
      return false;
    }
  }

  async function deleteFavorite(platform: string, roomId: string) {
    try {
      const res = await apiClient.deleteLiveFavorite({ platform, room_id: roomId });
      if (res.success) {
        await fetchFavorites();
        return true;
      }
      return false;
    } catch (e) {
      console.error('删除直播收藏失败:', e);
      return false;
    }
  }

  async function checkFavorite(platform: string, roomId: string): Promise<boolean> {
    try {
      const res = await apiClient.checkLiveFavorite({ platform, room_id: roomId });
      if (res.success && res.data) return !!res.data.is_favorited;
      return false;
    } catch (e) {
      console.error('检查直播收藏失败:', e);
      return false;
    }
  }

  async function clearFavorites() {
    try {
      const res = await apiClient.clearLiveFavorites();
      if (res.success) {
        await fetchFavorites();
        return true;
      }
      return false;
    } catch (e) {
      console.error('清空直播收藏失败:', e);
      return false;
    }
  }

  function isFavoriteLocally(platform: string, roomId: string) {
    return favorites.value.some((f) => f.platform === platform && f.room_id === roomId);
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
    isFavoriteLocally,
  };
});

