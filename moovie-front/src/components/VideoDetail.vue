<template>
  <div class="video-detail" v-if="!showFullscreenPlayer">
    <button class="back-button" @click="goBack">
      <el-icon><ArrowLeft /></el-icon>
      返回
    </button>

    <div v-if="detailLoading" class="loading-area">
      <div class="loading-spinner"></div>
      <span class="loading-text">加载中...</span>
    </div>

    <div v-else-if="error" class="error-area">
      <div class="error-icon">
        <el-icon :size="56"><WarningFilled /></el-icon>
      </div>
      <span class="error-text">{{ error }}</span>
      <el-button type="primary" @click="fetchDetail" class="retry-button">重试</el-button>
    </div>

    <div v-else-if="detail" class="detail-content">
      <div class="detail-header">
        <div class="poster-wrapper">
          <img :src="detail.vod_pic || placeholderImage" :alt="detail.vod_name" class="detail-poster" />
        </div>
        <div class="detail-info">
          <h1 class="detail-title">{{ detail.vod_name }}</h1>
          <div class="detail-meta">
            <span v-if="detail.vod_remarks" class="meta-item">{{ detail.vod_remarks }}</span>
            <span v-if="detail.vod_year" class="meta-item">{{ detail.vod_year }}</span>
            <span v-if="detail.vod_area" class="meta-item">{{ detail.vod_area }}</span>
          </div>
          <div v-if="detail.vod_actor" class="detail-section">
            <span class="section-label">主演：</span>
            <span class="section-content">{{ detail.vod_actor }}</span>
          </div>
          <div v-if="detail.vod_director" class="detail-section">
            <span class="section-label">导演：</span>
            <span class="section-content">{{ detail.vod_director }}</span>
          </div>
          <div v-if="detail.vod_content" class="detail-section">
            <span class="section-label">简介：</span>
            <span class="section-content">{{ detail.vod_content }}</span>
          </div>
          <div class="detail-actions">
            <el-button type="primary" size="large" @click="startPlay" class="play-button" :loading="playerLoading">
              <el-icon><VideoPlay /></el-icon>
              立即播放
            </el-button>
            <el-button 
              :type="isFavorited ? 'danger' : 'default'" 
              size="large" 
              @click="toggleFavorite"
              :loading="favoriteLoading"
            >
              <el-icon><Star :fill="isFavorited" /></el-icon>
              {{ isFavorited ? '已收藏' : '收藏' }}
            </el-button>
          </div>
        </div>
      </div>

      <div v-if="hasEpisodes" class="episodes-section">
        <h3 class="section-title">选集</h3>
        <div class="episode-grid">
          <div 
            v-for="(episode, index) in episodes" 
            :key="index"
            class="episode-card"
            :class="{ active: currentEpisodeIndex === index }"
            @click="startPlayFromEpisode(index)"
          >
            {{ episode.title }}
          </div>
        </div>
      </div>
    </div>
  </div>

  <FullscreenPlayer 
    v-if="showFullscreenPlayer"
    :src="currentPlayerUrl"
    :title="detail?.vod_name"
    :episodes="episodes"
    :detail="detail"
    :initial-episode-index="getInitialEpisodeIndex()"
    :initial-time="getInitialTime()"
    :auto-play="true"
    @close="closePlayer"
    @timeupdate="onPlayTimeUpdate"
    @episode-change="onEpisodeChange"
  />
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue';
import { ElMessage } from 'element-plus';
import { ArrowLeft, VideoPlay, WarningFilled, Star } from '@element-plus/icons-vue';
import FullscreenPlayer from './FullscreenPlayer.vue';
import type { VodItem } from '../types';
import { useHistoryStore } from '../stores/history';
import { useFavoritesStore } from '../stores/favorites';
import { apiClient } from '../api/client';

interface Props {
  vodItem: VodItem;
  onBack: () => void;
}

const props = defineProps<Props>();

const historyStore = useHistoryStore();
const favoritesStore = useFavoritesStore();

const detailLoading = ref(false);
const playerLoading = ref(false);
const favoriteLoading = ref(false);
const error = ref('');
const detail = ref<VodItem | null>(null);
const showFullscreenPlayer = ref(false);
const currentPlayerUrl = ref('');
const episodes = ref<Array<{ url: string; title: string }>>([]);
const currentEpisodeIndex = ref(0);
const currentPlayTime = ref(0);
const saveTimeout = ref<number | null>(null);
const isFavorited = ref(false);

const placeholderImage = 'data:image/svg+xml,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 width=%22400%22 height=%22600%22%3E%3Crect fill=%22%23f3f3f3%22 width=%22400%22 height=%22600%22/%3E%3Ctext x=%22200%22 y=%22300%22 fill=%22%234cc2ff%22 font-size=%2248%22 text-anchor=%22middle%22 dominant-baseline=%22middle%22%3E🎬%3C/text%3E%3C/svg%3E';

const hasEpisodes = computed(() => episodes.value.length > 0);

async function checkFavoriteStatus() {
  if (detail.value) {
    isFavorited.value = await favoritesStore.checkFavorite(detail.value.vod_id, detail.value.source_key);
  }
}

async function toggleFavorite() {
  if (!detail.value) return;
  
  favoriteLoading.value = true;
  
  try {
    if (isFavorited.value) {
      const success = await favoritesStore.deleteFavorite(detail.value.vod_id, detail.value.source_key);
      if (success) {
        isFavorited.value = false;
        ElMessage.success('已取消收藏');
      } else {
        ElMessage.error('操作失败');
      }
    } else {
      const success = await favoritesStore.addFavorite({
        vod_id: detail.value.vod_id,
        source_key: detail.value.source_key,
        vod_name: detail.value.vod_name,
        vod_pic: detail.value.vod_pic,
        vod_remarks: detail.value.vod_remarks,
        vod_actor: detail.value.vod_actor,
        vod_director: detail.value.vod_director,
        vod_content: detail.value.vod_content
      });
      if (success) {
        isFavorited.value = true;
        ElMessage.success('已添加到收藏');
      } else {
        ElMessage.error('操作失败');
      }
    }
  } catch (error) {
    console.error('操作失败:', error);
    ElMessage.error('操作失败');
  } finally {
    favoriteLoading.value = false;
  }
}

async function fetchDetail() {
  detailLoading.value = true;
  error.value = '';
  
  try {
    const result = await apiClient.getDetail(props.vodItem.source_key, props.vodItem.vod_id);
    
    if (result.success && result.data) {
      detail.value = result.data;
      parseEpisodes(result.data);
    } else {
      detail.value = props.vodItem;
      parseEpisodes(props.vodItem);
    }
    await checkFavoriteStatus();
  } catch (e) {
    console.error('获取详情失败:', e);
    detail.value = props.vodItem;
    parseEpisodes(props.vodItem);
  } finally {
    detailLoading.value = false;
  }
}

function cleanUrl(url: string): string {
  let cleaned = url.trim();
  
  console.log('清理 URL 前:', cleaned);
  
  cleaned = cleaned.replace(/^[`'"]+|[`'"]+$/g, '');
  
  cleaned = cleaned.replace(/\s+/g, '');
  
  console.log('清理 URL 后:', cleaned);
  
  return cleaned;
}

function parseEpisodes(vod: VodItem) {
  const newEpisodes: Array<{ url: string; title: string }> = [];
  
  console.log('=== parseEpisodes 开始 ===');
  console.log('vod.vod_play_url:', vod.vod_play_url);
  
  if (vod.vod_play_url) {
    const playUrlStr = vod.vod_play_url;
    const sourceSections = playUrlStr.split('$$$');
    
    console.log('sourceSections:', sourceSections);
    
    for (let section of sourceSections) {
      const parts = section.split('#');
      console.log('section:', section, 'parts:', parts);
      
      for (let part of parts) {
        if (part.trim()) {
          const firstDollarIndex = part.indexOf('$');
          console.log('part:', part, 'firstDollarIndex:', firstDollarIndex);
          
          if (firstDollarIndex !== -1) {
            const title = part.substring(0, firstDollarIndex).trim();
            let url = part.substring(firstDollarIndex + 1).trim();
            
            url = cleanUrl(url);
            
            newEpisodes.push({
              url: url,
              title: title
            });
            console.log('添加剧集:', title, '->', url);
          }
        }
      }
    }
  }
  
  console.log('解析到的剧集总数:', newEpisodes.length);
  
  if (newEpisodes.length === 0) {
    newEpisodes.push({
      url: 'https://test-streams.mux.dev/x36xhzz/x36xhzz.m3u8',
      title: '正片'
    });
  }
  
  episodes.value = newEpisodes;
  
  console.log('=== parseEpisodes 结束 ===');
}

function goBack() {
  props.onBack();
}

function startPlay() {
  if (episodes.value.length > 0) {
    startPlayFromEpisode(0);
  }
}

function startPlayFromEpisode(index: number) {
  const episode = episodes.value[index];
  if (!episode) return;
  
  currentEpisodeIndex.value = index;
  currentPlayerUrl.value = episode.url;
  showFullscreenPlayer.value = true;
}

function getInitialTime(): number {
  if (detail.value) {
    const historyItem = historyStore.getHistoryByVodId(detail.value.vod_id, detail.value.source_key);
    if (historyItem && historyItem.progress > 0) {
      return historyItem.progress;
    }
  }
  return 0;
}

function getInitialEpisodeIndex(): number {
  if (detail.value) {
    const historyItem = historyStore.getHistoryByVodId(detail.value.vod_id, detail.value.source_key);
    if (historyItem && historyItem.episode) {
      const index = episodes.value.findIndex(e => e.title === historyItem.episode);
      if (index !== -1) {
        return index;
      }
    }
  }
  return 0;
}

function closePlayer() {
  saveProgressNow();
  showFullscreenPlayer.value = false;
}

function onEpisodeChange(index: number) {
  saveProgressNow();
  currentEpisodeIndex.value = index;
}

function onPlayTimeUpdate(time: number, episodeIndex: number) {
  if (detail.value) {
    currentPlayTime.value = time;
    currentEpisodeIndex.value = episodeIndex;
    
    if (saveTimeout.value) {
      window.clearTimeout(saveTimeout.value);
    }
    saveTimeout.value = window.setTimeout(() => {
      historyStore.addHistory({
        vod_id: detail.value!.vod_id,
        source_key: detail.value!.source_key,
        vod_name: detail.value!.vod_name,
        vod_pic: detail.value!.vod_pic,
        progress: time,
        episode: episodes.value[episodeIndex]?.title
      });
    }, 2000);
  }
}

function saveProgressNow() {
  if (saveTimeout.value) {
    window.clearTimeout(saveTimeout.value);
    saveTimeout.value = null;
  }
  if (detail.value) {
    historyStore.addHistory({
      vod_id: detail.value.vod_id,
      source_key: detail.value.source_key,
      vod_name: detail.value.vod_name,
      vod_pic: detail.value.vod_pic,
      progress: currentPlayTime.value,
      episode: episodes.value[currentEpisodeIndex.value]?.title
    });
  }
}

onMounted(() => {
  fetchDetail();
  historyStore.fetchHistory();
});

onUnmounted(() => {
  if (saveTimeout.value) {
    window.clearTimeout(saveTimeout.value);
  }
  saveProgressNow();
});
</script>

<style scoped>
.video-detail {
  width: 100%;
  padding: 20px 0;
}

.back-button {
  display: flex;
  align-items: center;
  gap: 8px;
  background: transparent;
  border: none;
  color: var(--el-text-color-secondary);
  font-size: 14px;
  cursor: pointer;
  padding: 8px 12px;
  border-radius: 8px;
  margin-bottom: 20px;
  transition: all 0.2s ease;
}

.back-button:hover {
  background: var(--el-fill-color-lighter);
  color: var(--el-text-color-primary);
}

.loading-area,
.error-area {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px 20px;
  gap: 16px;
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 3px solid rgba(76, 194, 255, 0.2);
  border-top-color: #4cc2ff;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.loading-text,
.error-text {
  font-size: 13px;
  color: var(--el-text-color-secondary);
}

.error-icon {
  color: #4cc2ff;
  opacity: 0.4;
}

.retry-button {
  margin-top: 4px;
  border-radius: 8px;
  background: #4cc2ff;
  border: none;
  color: #000;
}

.retry-button:hover {
  background: #5fb3ff;
  color: #000;
}

.detail-content {
  width: 100%;
}

.detail-header {
  display: flex;
  gap: 32px;
  margin-bottom: 32px;
}

.poster-wrapper {
  flex-shrink: 0;
}

.detail-poster {
  width: 240px;
  border-radius: 12px;
  box-shadow: var(--el-box-shadow);
}

.detail-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.detail-title {
  font-size: 28px;
  font-weight: 600;
  margin: 0;
  color: var(--el-text-color-primary);
}

.detail-meta {
  display: flex;
  gap: 16px;
  flex-wrap: wrap;
}

.meta-item {
  font-size: 13px;
  color: var(--el-text-color-secondary);
  padding: 4px 12px;
  background: var(--el-fill-color-blank);
  border-radius: 12px;
}

.detail-section {
  display: flex;
  gap: 8px;
  font-size: 14px;
  line-height: 1.6;
}

.section-label {
  color: var(--el-text-color-secondary);
  flex-shrink: 0;
}

.section-content {
  color: var(--el-text-color-regular);
}

.detail-actions {
  margin-top: 8px;
}

.play-button {
  padding: 12px 32px;
  border-radius: 10px;
  background: #4cc2ff;
  border: none;
  color: #000;
  font-weight: 600;
  font-size: 15px;
}

.play-button:hover {
  background: #5fb3ff;
  color: #000;
}

.player-section {
  margin-bottom: 32px;
}

.episodes-section {
  margin-bottom: 20px;
}

.section-title {
  font-size: 18px;
  font-weight: 600;
  margin: 0 0 16px;
  color: var(--el-text-color-primary);
}

.episode-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
  gap: 12px;
}

.episode-card {
  padding: 12px 16px;
  background: var(--el-bg-color-overlay);
  border: 1px solid var(--el-border-color);
  border-radius: 8px;
  text-align: center;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s ease;
  color: var(--el-text-color-primary);
}

.episode-card:hover {
  border-color: var(--el-color-primary);
  background: rgba(76, 194, 255, 0.1);
}

.episode-card.active {
  border-color: var(--el-color-primary);
  background: var(--el-color-primary);
  color: #000;
}

.debug-info {
  background: #333;
  color: #fff;
  padding: 10px;
  margin-bottom: 10px;
  border-radius: 8px;
  font-family: monospace;
  font-size: 12px;
}

.debug-info p {
  margin: 5px 0;
}

@media (max-width: 768px) {
  .detail-header {
    flex-direction: column;
  }

  .detail-poster {
    width: 100%;
    max-width: 240px;
  }
}
</style>
