<template>
  <div class="favorites">
    <div v-if="loading" class="loading-container">
      <el-icon class="loading-icon"><Loading /></el-icon>
      <span class="loading-text">加载中...</span>
    </div>
    
    <div v-else-if="!hasFavorites" class="empty-container">
      <div class="empty-icon">⭐</div>
      <div class="empty-text">暂无收藏</div>
    </div>
    
    <div v-else class="favorites-grid">
      <div 
        v-for="item in favorites" 
        :key="`${item.vod_id}-${item.source_key}`"
        class="favorite-item"
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
          <div v-if="item.vod_remarks" class="item-badge">{{ item.vod_remarks }}</div>
        </div>
        
        <div class="info">
          <div class="title">{{ item.vod_name }}</div>
          <div class="meta">
            <span class="source">来源: {{ item.source_key }}</span>
          </div>
        </div>
        
        <div class="delete-btn" @click.stop="onDeleteItem(item)">
          <el-icon><Delete /></el-icon>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { storeToRefs } from 'pinia';
import { ElMessage, ElMessageBox } from 'element-plus';
import { Star, Delete } from '@element-plus/icons-vue';
import { useFavoritesStore, type FavoriteItem } from '../stores/favorites';

interface Emits {
  (e: 'select', item: FavoriteItem): void;
}

const emit = defineEmits<Emits>();

const favoritesStore = useFavoritesStore();
const { favorites, loading, hasFavorites } = storeToRefs(favoritesStore);
const { fetchFavorites, deleteFavorite } = favoritesStore;

function onItemClick(item: FavoriteItem) {
  emit('select', item);
}

async function onDeleteItem(item: FavoriteItem) {
  try {
    await ElMessageBox.confirm(
      `确定要取消收藏"${item.vod_name}"吗？`,
      '删除确认',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    );
    const success = await deleteFavorite(item.vod_id, item.source_key);
    if (success) {
      ElMessage.success('已取消收藏');
    } else {
      ElMessage.error('操作失败');
    }
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
  fetchFavorites();
});
</script>

<style scoped>
.favorites {
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

.favorites-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
  gap: 16px;
}

.favorite-item {
  position: relative;
  background: var(--el-bg-color);
  border-radius: var(--el-border-radius-base);
  overflow: hidden;
  cursor: pointer;
  transition: all 0.2s ease;
  border: 1px solid transparent;
}

.favorite-item:hover {
  border-color: var(--el-color-primary);
  transform: translateY(-4px);
  box-shadow: 0 8px 20px rgba(0, 0, 0, 0.15);
}

.poster-wrapper {
  position: relative;
  width: 100%;
  aspect-ratio: 2/3;
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

.item-badge {
  position: absolute;
  top: 8px;
  right: 8px;
  background: var(--el-color-primary);
  color: #fff;
  padding: 4px 8px;
  font-size: 12px;
  border-radius: 4px;
}

.info {
  padding: 12px;
}

.title {
  font-size: 14px;
  font-weight: 500;
  color: var(--el-text-color-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  margin-bottom: 6px;
}

.meta {
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.source {
  opacity: 0.7;
}

.delete-btn {
  position: absolute;
  top: 8px;
  left: 8px;
  width: 32px;
  height: 32px;
  background: rgba(255, 0, 0, 0.8);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  cursor: pointer;
  opacity: 0;
  transition: all 0.2s ease;
}

.favorite-item:hover .delete-btn {
  opacity: 1;
}

.delete-btn:hover {
  background: #ff0000;
  transform: scale(1.1);
}
</style>