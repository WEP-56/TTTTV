import { defineStore } from 'pinia';
import { ref } from 'vue';
import { apiClient } from '../api/client';
import type { VodItem } from '../types';

export const useSearchStore = defineStore('search', () => {
  const keyword = ref('');
  const results = ref<VodItem[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const filteredCount = ref(0);

  async function search(query: string, bypass = false) {
    if (!query.trim()) return;
    
    keyword.value = query;
    loading.value = true;
    error.value = null;
    results.value = [];

    try {
      const res = await apiClient.search(query, bypass);
      if (res.success && res.data) {
        results.value = res.data.items;
        filteredCount.value = res.data.filtered_count;
      } else {
        error.value = res.error || '搜索失败';
      }
    } catch (err) {
      error.value = '网络错误，请检查后端是否启动';
      console.error(err);
    } finally {
      loading.value = false;
    }
  }

  function clearResults() {
    results.value = [];
    error.value = null;
  }

  return {
    keyword,
    results,
    loading,
    error,
    filteredCount,
    search,
    clearResults,
  };
});
