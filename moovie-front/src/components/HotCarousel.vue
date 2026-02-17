<template>
  <div class="hot-carousel" ref="carouselRef">
    <div class="carousel-container">
      <div 
        class="carousel-track"
        :style="{ transform: `translateX(-${currentIndex * 100}%)` }"
      >
        <div 
          v-for="(item, index) in items" 
          :key="item.id"
          class="carousel-slide"
        >
          <div class="slide-content">
            <div class="poster-section">
              <img 
                v-if="item.vod_pic" 
                :src="item.vod_pic" 
                :alt="item.vod_name"
                class="poster"
              />
            </div>
            <div class="info-section">
              <div class="hot-badge">🔥 热门推荐</div>
              <h2 class="title">{{ item.vod_name }}</h2>
              <div class="meta">
                <span v-if="item.vod_remarks" class="year">{{ item.vod_remarks }}</span>
                <span v-if="item.rating" class="rating">
                  <span class="star">★</span>
                  {{ item.rating }}
                </span>
              </div>
              <p class="description">精彩内容，不容错过</p>
              <button class="play-button" @click="onPlayClick(item)">
                <span class="icon">▶</span>
                立即播放
              </button>
            </div>
          </div>
          <div class="slide-overlay"></div>
        </div>
      </div>
    </div>
    
    <button class="nav-button prev" @click="prev">
      ‹
    </button>
    <button class="nav-button next" @click="next">
      ›
    </button>
    
    <div class="indicators">
      <button 
        v-for="(item, index) in items" 
        :key="index"
        class="indicator"
        :class="{ active: index === currentIndex }"
        @click="goTo(index)"
      ></button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import type { RecommendationItem } from '../stores/recommendation';

interface Props {
  items: RecommendationItem[];
  autoPlay?: boolean;
  interval?: number;
}

interface Emits {
  (e: 'play', item: RecommendationItem): void;
}

const props = withDefaults(defineProps<Props>(), {
  autoPlay: true,
  interval: 5000
});

const emit = defineEmits<Emits>();

const carouselRef = ref<HTMLDivElement | null>(null);
const currentIndex = ref(0);
let autoPlayTimer: number | null = null;

function next() {
  if (props.items.length === 0) return;
  currentIndex.value = (currentIndex.value + 1) % props.items.length;
  resetAutoPlay();
}

function prev() {
  if (props.items.length === 0) return;
  currentIndex.value = (currentIndex.value - 1 + props.items.length) % props.items.length;
  resetAutoPlay();
}

function goTo(index: number) {
  if (index >= 0 && index < props.items.length) {
    currentIndex.value = index;
    resetAutoPlay();
  }
}

function onPlayClick(item: RecommendationItem) {
  emit('play', item);
}

function resetAutoPlay() {
  if (autoPlayTimer) {
    clearInterval(autoPlayTimer);
  }
  if (props.autoPlay && props.items.length > 1) {
    autoPlayTimer = window.setInterval(next, props.interval);
  }
}

onMounted(() => {
  if (props.autoPlay && props.items.length > 1) {
    autoPlayTimer = window.setInterval(next, props.interval);
  }
});

onUnmounted(() => {
  if (autoPlayTimer) {
    clearInterval(autoPlayTimer);
  }
});
</script>

<style scoped>
.hot-carousel {
  position: relative;
  width: 100%;
  border-radius: var(--el-border-radius-base);
  overflow: hidden;
  background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
}

.carousel-container {
  position: relative;
  width: 100%;
  overflow: hidden;
}

.carousel-track {
  display: flex;
  transition: transform 0.5s cubic-bezier(0.4, 0, 0.2, 1);
}

.carousel-slide {
  position: relative;
  min-width: 100%;
  aspect-ratio: 21 / 9;
}

.slide-content {
  position: relative;
  z-index: 2;
  display: flex;
  height: 100%;
  padding: 40px;
  gap: 40px;
}

.poster-section {
  flex-shrink: 0;
  width: 180px;
  height: 270px;
  border-radius: 12px;
  overflow: hidden;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.4);
}

.poster {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.info-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
  color: #fff;
}

.hot-badge {
  display: inline-block;
  background: linear-gradient(135deg, #ff6b6b, #ee5a5a);
  padding: 6px 16px;
  border-radius: 20px;
  font-size: 13px;
  font-weight: 500;
  margin-bottom: 16px;
  width: fit-content;
}

.title {
  font-size: 32px;
  font-weight: 700;
  margin: 0 0 12px 0;
  text-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}

.meta {
  display: flex;
  gap: 12px;
  margin-bottom: 16px;
  font-size: 14px;
}

.year {
  opacity: 0.8;
}

.rating {
  display: flex;
  align-items: center;
  gap: 4px;
}

.star {
  color: #ffd700;
}

.description {
  font-size: 15px;
  opacity: 0.8;
  margin-bottom: 24px;
  max-width: 500px;
}

.play-button {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 28px;
  background: var(--el-color-primary);
  color: #fff;
  border: none;
  border-radius: 24px;
  font-size: 15px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  width: fit-content;
  box-shadow: 0 4px 16px rgba(76, 194, 255, 0.4);
}

.play-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(76, 194, 255, 0.5);
}

.play-button .icon {
  font-size: 12px;
}

.slide-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(90deg, rgba(0, 0, 0, 0.7) 0%, rgba(0, 0, 0, 0.3) 50%, rgba(0, 0, 0, 0.1) 100%);
  z-index: 1;
}

.nav-button {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
  z-index: 10;
  width: 48px;
  height: 48px;
  border: none;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.15);
  backdrop-filter: blur(10px);
  color: #fff;
  font-size: 24px;
  cursor: pointer;
  transition: all 0.3s ease;
}

.nav-button:hover {
  background: rgba(255, 255, 255, 0.25);
}

.nav-button.prev {
  left: 16px;
}

.nav-button.next {
  right: 16px;
}

.indicators {
  position: absolute;
  bottom: 20px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 10;
  display: flex;
  gap: 8px;
}

.indicator {
  width: 8px;
  height: 8px;
  border: none;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.4);
  cursor: pointer;
  transition: all 0.3s ease;
}

.indicator.active {
  width: 24px;
  border-radius: 4px;
  background: #fff;
}

.indicator:hover {
  background: rgba(255, 255, 255, 0.7);
}
</style>
