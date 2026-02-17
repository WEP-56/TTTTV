import { defineStore } from 'pinia';
import { ref } from 'vue';

const SEARCH_HISTORY_KEY = 'ttt_search_history';
const MAX_HISTORY_LIMIT = 20;

export const useSearchHistoryStore = defineStore('searchHistory', () => {
  const history = ref<string[]>([]);

  function loadFromStorage() {
    try {
      const saved = localStorage.getItem(SEARCH_HISTORY_KEY);
      if (saved) {
        history.value = JSON.parse(saved);
      }
    } catch (error) {
      console.error('加载搜索历史失败:', error);
    }
  }

  function saveToStorage() {
    try {
      localStorage.setItem(SEARCH_HISTORY_KEY, JSON.stringify(history.value));
    } catch (error) {
      console.error('保存搜索历史失败:', error);
    }
  }

  function addSearch(query: string) {
    const trimmed = query.trim();
    if (!trimmed) return;

    history.value = history.value.filter(item => item !== trimmed);
    history.value.unshift(trimmed);
    if (history.value.length > MAX_HISTORY_LIMIT) {
      history.value = history.value.slice(0, MAX_HISTORY_LIMIT);
    }
    saveToStorage();
  }

  function removeSearch(query: string) {
    history.value = history.value.filter(item => item !== query);
    saveToStorage();
  }

  function clearHistory() {
    history.value = [];
    saveToStorage();
  }

  return {
    history,
    loadFromStorage,
    addSearch,
    removeSearch,
    clearHistory
  };
});