<template>
  <div class="watch-history">
    <div v-if="loading" class="loading-container">
      <el-icon class="loading-icon"><Loading /></el-icon>
      <span class="loading-text">加载中...</span>
    </div>
    
    <div v-else-if="!hasHistory" class="empty-container">
      <div class="empty-icon">📺</div>
      <div class="empty-text">暂无观影记录</div>
    </div>
    
    <div v-else class="history-list">
      <div 
        v-for="(item, index) in history" 
        :key="`${item.vod_id}-${item.source_key}`"
        class="history-item"
      >
        <div 
          class="item-content"
          @click="onItemClick(item)"
        >
          <div class="poster-wrapper">
            <img 
              v-if="item.vod_pic" 
              :src="item.vod_pic" 
              :alt="item.vod_name"
              class="poster"
              @error="onPosterError"
            />
            <div v-else class="poster-placeholder">
              <span class="placeholder-icon">🎬</span>
            </div>
            <div class="progress-bar">
              <div class="progress-fill" :style="{ width: `${Math.min(100, item.progress * 100)}%` }"></div>
            </div>
            <div class="progress-text">{{ Math.round(item.progress * 100) }}%</div>
          </div>
          
          <div class="info">
            <div class="title">{{ item.vod_name }}</div>
            <div class="meta">
              <span v-if="item.episode" class="episode">{{ item.episode }}</span>
              <span class="source">来源: {{ item.source_key }}</span>
              <span class="time">{{ formatTime(item.last_play_time) }}</span>
            </div>
          </div>
        </div>
        
        <div class="item-actions">
          <el-button 
            type="danger" 
            circle 
            size="small"
            @click.stop="onDeleteItem(item)"
          >
            <el-icon><Delete /></el-icon>
          </el-button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { storeToRefs } from 'pinia';
import { ElMessage, ElMessageBox } from 'element-plus';
import { Loading, Delete } from '@element-plus/icons-vue';
import { useHistoryStore, type WatchHistoryItem } from '../stores/history';

interface Emits {
  (e: 'select', item: WatchHistoryItem): void;
}

const emit = defineEmits<Emits>();

const historyStore = useHistoryStore();
const { history, loading, hasHistory } = storeToRefs(historyStore);
const { fetchHistory, deleteHistory } = historyStore;

function formatTime(timestamp: number): string {
  const date = new Date(timestamp * 1000);
  const now = new Date();
  const diff = now.getTime() - date.getTime();
  const minutes = Math.floor(diff / 60000);
  const hours = Math.floor(diff / 3600000);
  const days = Math.floor(diff / 86400000);
  
  if (minutes < 1) return '刚刚';
  if (minutes < 60) return `${minutes}分钟前`;
  if (hours < 24) return `${hours}小时前`;
  if (days < 7) return `${days}天前`;
  
  return date.toLocaleDateString('zh-CN');
}

function onItemClick(item: WatchHistoryItem) {
  emit('select', item);
}

async function onDeleteItem(item: WatchHistoryItem) {
  try {
    await ElMessageBox.confirm(
      `确定要删除观影记录"${item.vod_name}"吗？`,
      '删除确认',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    );
    await deleteHistory(item.vod_id, item.source_key);
    ElMessage.success('已删除');
  } catch (error) {
    if (error !== 'cancel') {
      console.error('删除失败:', error);
      ElMessage.error('删除失败');
    }
  }
}

function onPosterError(event: Event) {
  const img = event.target as HTMLImageElement;
  img.style.display = 'none';
  const placeholder = img.parentElement?.querySelector('.poster-placeholder');
  if (placeholder) {
    (placeholder as HTMLElement).style.display = 'flex';
  }
}

onMounted(() => {
  fetchHistory();
});
</script>

<style scoped>
.watch-history {
  width: 100%;
}

.loading-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  gap: 12px;
}

.loading-icon {
  font-size: 32px;
  color: var(--el-color-primary);
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.loading-text {
  color: var(--el-text-color-secondary);
  font-size: 14px;
}

.empty-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  gap: 12px;
}

.empty-icon {
  font-size: 48px;
  opacity: 0.5;
}

.empty-text {
  color: var(--el-text-color-secondary);
  font-size: 14px;
}

.history-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.history-item {
  display: flex;
  gap: 12px;
  padding: 12px;
  background: var(--el-bg-color);
  border-radius: var(--el-border-radius-base);
  align-items: center;
  transition: all 0.2s ease;
  border: 1px solid transparent;
}

.history-item:hover {
  border-color: var(--el-color-primary);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.item-content {
  flex: 1;
  display: flex;
  gap: 12px;
  cursor: pointer;
}

.poster-wrapper {
  position: relative;
  flex-shrink: 0;
  width: 100px;
  height: 140px;
  border-radius: 8px;
  overflow: hidden;
}

.poster {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.poster-placeholder {
  display: none;
  width: 100%;
  height: 100%;
  background: var(--el-fill-color-light);
  align-items: center;
  justify-content: center;
}

.poster-placeholder .placeholder-icon {
  font-size: 36px;
  opacity: 0.5;
}

.progress-bar {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 4px;
  background: rgba(0, 0, 0, 0.5);
}

.progress-fill {
  height: 100%;
  background: var(--el-color-primary);
  transition: width 0.3s ease;
}

.progress-text {
  position: absolute;
  bottom: 8px;
  right: 8px;
  background: rgba(0, 0, 0, 0.7);
  color: #fff;
  font-size: 11px;
  padding: 2px 6px;
  border-radius: 4px;
}

.info {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 8px;
  min-width: 0;
}

.title {
  font-size: 15px;
  font-weight: 500;
  color: var(--el-text-color-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.meta {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.episode {
  background: var(--el-color-primary-light-9);
  color: var(--el-color-primary);
  padding: 2px 8px;
  border-radius: 4px;
}

.source,
.time {
  opacity: 0.7;
}

.item-actions {
  flex-shrink: 0;
}
</style>
