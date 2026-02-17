import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { apiClient, type DoubanSubject } from '../api/client';

export interface RecommendationItem extends DoubanSubject {
  id: string;
  rating?: number;
  isHot?: boolean;
}

export const useRecommendationStore = defineStore('recommendation', () => {
  const hotItems = ref<RecommendationItem[]>([]);
  const movieItems = ref<RecommendationItem[]>([]);
  const tvItems = ref<RecommendationItem[]>([]);
  const loading = ref(false);
  const currentHotIndex = ref(0);
  const error = ref<string | null>(null);

  const hasHotItems = computed(() => hotItems.value.length > 0);
  const hasMovieItems = computed(() => movieItems.value.length > 0);
  const hasTvItems = computed(() => tvItems.value.length > 0);

  function convertToRecommendationItem(subject: DoubanSubject, isHot = false): RecommendationItem {
    return {
      id: subject.id || String(Math.random()),
      title: subject.title,
      cover: subject.cover || subject.cover_url,
      cover_url: subject.cover_url || subject.cover,
      rate: subject.rate,
      year: subject.year,
      url: subject.url,
      rating: subject.rate ? parseFloat(subject.rate) : undefined,
      isHot,
    };
  }

  async function fetchRecommendations() {
    try {
      loading.value = true;
      error.value = null;

      const [hotResult, movieResult, tvResult] = await Promise.allSettled([
        apiClient.doubanChart({ type: '11', limit: '10' }),
        apiClient.doubanSearch({ type: 'movie', tag: '热门', page_limit: 20 }),
        apiClient.doubanSearch({ type: 'tv', tag: '热门', page_limit: 20 }),
      ]);

      if (hotResult.status === 'fulfilled' && hotResult.value.success) {
        hotItems.value = hotResult.value.data?.subjects.slice(0, 6).map(s => convertToRecommendationItem(s, true)) || [];
      }

      if (movieResult.status === 'fulfilled' && movieResult.value.success) {
        movieItems.value = movieResult.value.data?.subjects.map(s => convertToRecommendationItem(s)) || [];
      }

      if (tvResult.status === 'fulfilled' && tvResult.value.success) {
        tvItems.value = tvResult.value.data?.subjects.map(s => convertToRecommendationItem(s)) || [];
      }

    } catch (err) {
      console.error('获取推荐失败:', err);
      error.value = '加载失败，请稍后重试';
    } finally {
      loading.value = false;
    }
  }

  function nextHotItem() {
    if (hotItems.value.length > 0) {
      currentHotIndex.value = (currentHotIndex.value + 1) % hotItems.value.length;
    }
  }

  function prevHotItem() {
    if (hotItems.value.length > 0) {
      currentHotIndex.value = (currentHotIndex.value - 1 + hotItems.value.length) % hotItems.value.length;
    }
  }

  function setHotIndex(index: number) {
    if (index >= 0 && index < hotItems.value.length) {
      currentHotIndex.value = index;
    }
  }

  return {
    hotItems,
    movieItems,
    tvItems,
    loading,
    currentHotIndex,
    error,
    hasHotItems,
    hasMovieItems,
    hasTvItems,
    fetchRecommendations,
    nextHotItem,
    prevHotItem,
    setHotIndex
  };
});
