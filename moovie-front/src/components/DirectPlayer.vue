<template>
  <div class="direct-player-page">
    <div class="page-header">
      <div class="page-header-content">
        <h1 class="page-title">M3U8 直连播放</h1>
        <p class="page-subtitle">输入 m3u8 链接即可播放</p>
      </div>
    </div>

    <div class="input-section">
      <el-input
        v-model="m3u8Url"
        placeholder="请输入 m3u8 链接..."
        size="large"
        class="url-input"
        clearable
      />
      <el-button 
        type="primary" 
        size="large" 
        @click="startPlay"
        :disabled="!m3u8Url.trim()"
      >
        播放
      </el-button>
    </div>

    <div v-if="recentLinks.length > 0" class="recent-section">
      <div class="recent-header">
        <span class="recent-title">最近播放</span>
        <el-button type="text" size="small" @click="clearRecentLinks">
          清空
        </el-button>
      </div>
      <div class="recent-links">
        <div 
          v-for="(link, index) in recentLinks" 
          :key="index"
          class="recent-link-item"
          @click="selectRecentLink(link)"
        >
          <div class="link-info">
            <span class="link-text">{{ link }}</span>
          </div>
          <el-icon class="play-icon"><VideoPlay /></el-icon>
        </div>
      </div>
    </div>

    <div v-if="showPlayer" class="player-section">
      <FullscreenPlayer
        :src="currentUrl"
        :title="currentTitle"
        :auto-play="true"
        @close="closePlayer"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { ElMessage } from 'element-plus';
import { VideoPlay } from '@element-plus/icons-vue';
import FullscreenPlayer from './FullscreenPlayer.vue';

const RECENT_KEY = 'ttt_recent_m3u8';
const MAX_RECENT = 10;

const m3u8Url = ref('');
const recentLinks = ref<string[]>([]);
const showPlayer = ref(false);
const currentUrl = ref('');
const currentTitle = ref('M3U8 播放');

function loadRecentLinks() {
  try {
    const saved = localStorage.getItem(RECENT_KEY);
    if (saved) {
      recentLinks.value = JSON.parse(saved);
    }
  } catch (error) {
    console.error('加载最近播放失败:', error);
  }
}

function saveRecentLink(url: string) {
  recentLinks.value = recentLinks.value.filter(item => item !== url);
  recentLinks.value.unshift(url);
  if (recentLinks.value.length > MAX_RECENT) {
    recentLinks.value = recentLinks.value.slice(0, MAX_RECENT);
  }
  try {
    localStorage.setItem(RECENT_KEY, JSON.stringify(recentLinks.value));
  } catch (error) {
    console.error('保存最近播放失败:', error);
  }
}

function clearRecentLinks() {
  recentLinks.value = [];
  try {
    localStorage.removeItem(RECENT_KEY);
    ElMessage.success('已清空最近播放');
  } catch (error) {
    console.error('清空最近播放失败:', error);
  }
}

function startPlay() {
  const url = m3u8Url.value.trim();
  if (!url) {
    ElMessage.warning('请输入 m3u8 链接');
    return;
  }
  
  saveRecentLink(url);
  currentUrl.value = url;
  currentTitle.value = 'M3U8 播放';
  showPlayer.value = true;
}

function selectRecentLink(url: string) {
  m3u8Url.value = url;
  startPlay();
}

function closePlayer() {
  showPlayer.value = false;
}

onMounted(() => {
  loadRecentLinks();
});
</script>

<style scoped>
.direct-player-page {
  width: 100%;
}

.input-section {
  display: flex;
  gap: 12px;
  margin-bottom: 24px;
}

.url-input {
  flex: 1;
}

.recent-section {
  margin-bottom: 24px;
}

.recent-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.recent-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--el-text-color-primary);
}

.recent-links {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.recent-link-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  background: var(--el-fill-color-blank);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
  border: 1px solid transparent;
}

.recent-link-item:hover {
  background: var(--el-fill-color-light);
  border-color: rgba(76, 194, 255, 0.3);
}

.link-info {
  flex: 1;
  min-width: 0;
}

.link-text {
  font-size: 13px;
  color: var(--el-text-color-regular);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  display: block;
}

.play-icon {
  color: #4cc2ff;
  flex-shrink: 0;
}

.player-section {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 1000;
}
</style>