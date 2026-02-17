<template>
  <div class="smart-recommendations">
    <div class="section-header">
      <h3 class="title">
        <span class="icon">✨</span>
        智能推荐
      </h3>
      <button class="see-more" @click="onSeeMore">
        查看更多
      </button>
    </div>
    
    <div v-if="loading" class="loading-container">
      <el-icon class="loading-icon"><Loading /></el-icon>
    </div>
    
    <div v-else-if="items.length === 0" class="empty-container">
      <div class="empty-text">暂无推荐内容</div>
    </div>
    
    <div v-else class="recommendations-grid">
      <div 
        v-for="item in items" 
        :key="item.id"
        class="recommendation-card"
        @click="onCardClick(item)"
      >
        <div class="poster-wrapper">
          <img 
            v-if="item.vod_pic" 
            :src="item.vod_pic" 
            :alt="item.vod_name"
            class="poster"
          />
          <div v-else class="poster-placeholder">
            <span class="placeholder-icon">🎬</span>
          </div>
          <div class="card-overlay">
            <button class="play-overlay">
              <span class="play-icon">▶</span>
            </button>
          </div>
          <div v-if="item.rating" class="rating-badge">
            ★ {{ item.rating }}
          </div>
        </div>
        
        <div class="card-info">
          <div class="card-title">{{ item.vod_name }}</div>
          <div class="card-meta">
            <span v-if="item.vod_remarks" class="card-remarks">{{ item.vod_remarks }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Loading } from '@element-plus/icons-vue';
import type { RecommendationItem } from '../stores/recommendation';

interface Props {
  items: RecommendationItem[];
  loading?: boolean;
}

interface Emits {
  (e: 'select', item: RecommendationItem): void;
  (e: 'see-more'): void;
}

const props = withDefaults(defineProps<Props>(), {
  loading: false
});

const emit = defineEmits<Emits>();

function onCardClick(item: RecommendationItem) {
  emit('select', item);
}

function onSeeMore() {
  emit('see-more');
}
</script>

<style scoped>
.smart-recommendations {
  width: 100%;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 18px;
  font-weight: 600;
  color: var(--el-text-color-primary);
  margin: 0;
}

.title .icon {
  font-size: 20px;
}

.see-more {
  background: transparent;
  border: none;
  color: var(--el-color-primary);
  font-size: 14px;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: 4px;
  transition: all 0.2s ease;
}

.see-more:hover {
  background: var(--el-color-primary-light-9);
}

.loading-container {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
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

.empty-container {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
}

.empty-text {
  color: var(--el-text-color-secondary);
  font-size: 14px;
}

.recommendations-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
  gap: 16px;
}

.recommendation-card {
  cursor: pointer;
  transition: all 0.3s ease;
}

.recommendation-card:hover {
  transform: translateY(-4px);
}

.recommendation-card:hover .card-overlay {
  opacity: 1;
}

.poster-wrapper {
  position: relative;
  width: 100%;
  aspect-ratio: 2 / 3;
  border-radius: 8px;
  overflow: hidden;
  margin-bottom: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.poster {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.poster-placeholder {
  width: 100%;
  height: 100%;
  background: var(--el-fill-color-light);
  display: flex;
  align-items: center;
  justify-content: center;
}

.poster-placeholder .placeholder-icon {
  font-size: 36px;
  opacity: 0.5;
}

.card-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.4);
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  transition: opacity 0.3s ease;
}

.play-overlay {
  width: 56px;
  height: 56px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.9);
  border: none;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.3s ease;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

.play-overlay:hover {
  transform: scale(1.1);
  background: #fff;
}

.play-icon {
  font-size: 18px;
  margin-left: 4px;
}

.rating-badge {
  position: absolute;
  top: 8px;
  right: 8px;
  background: linear-gradient(135deg, #ffd700, #ffaa00);
  color: #000;
  font-size: 12px;
  font-weight: 600;
  padding: 4px 8px;
  border-radius: 4px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
}

.card-info {
  padding: 0 4px;
}

.card-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--el-text-color-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  margin-bottom: 4px;
}

.card-meta {
  display: flex;
  gap: 8px;
}

.card-remarks {
  font-size: 12px;
  color: var(--el-text-color-secondary);
}
</style>
