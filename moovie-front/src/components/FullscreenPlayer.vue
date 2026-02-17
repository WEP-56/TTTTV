<template>
  <div class="fullscreen-player">
    <div class="player-main">
      <div class="player-top-bar" data-tauri-drag-region>
        <button class="top-button back-button" @click="handleBack">
          <el-icon><ArrowLeft /></el-icon>
          返回
        </button>
        <div class="player-title" data-tauri-drag-region>{{ videoTitle }}</div>
        <button class="top-button sidebar-button" @click="toggleSidebar">
          <el-icon><List /></el-icon>
          详情
        </button>
      </div>

      <div class="video-container">
        <video
          ref="videoElement"
          class="video-element"
          :src="currentSrc"
          @timeupdate="onTimeUpdate"
          @loadedmetadata="onLoadedMetadata"
          @play="onPlay"
          @pause="onPause"
          @ended="onEnded"
          @click="togglePlay"
        ></video>

        <div 
          class="controls-overlay"
          :class="{ 'visible': showControls || !isPlaying }"
          @mousemove="onControlsMouseMove"
          @mouseleave="onControlsMouseLeave"
        >
          <div class="progress-bar" @mousedown="startScrubbing">
            <div class="progress-buffered" :style="{ width: bufferedPercentage + '%' }"></div>
            <div class="progress-played" :style="{ width: playedPercentage + '%' }"></div>
            <div 
              class="progress-handle" 
              :style="{ left: playedPercentage + '%' }"
            ></div>
          </div>

          <div class="controls-bottom">
            <div class="controls-left">
              <button class="control-button" @click="togglePlay">
                <span v-if="isPlaying">⏸</span>
                <span v-else>▶</span>
              </button>

              <div class="volume-control">
                <button class="control-button" @click="toggleMute">
                  <span v-if="isMuted || volume === 0">🔇</span>
                  <span v-else-if="volume < 0.5">🔉</span>
                  <span v-else>🔊</span>
                </button>
                <input 
                  type="range" 
                  class="volume-slider"
                  min="0" 
                  max="1" 
                  step="0.01"
                  v-model="volume"
                  @input="onVolumeChange"
                />
              </div>

              <div class="time-display">
                <span>{{ formatTime(currentTime) }}</span>
                <span class="time-separator">/</span>
                <span>{{ formatTime(duration) }}</span>
              </div>

              <button 
                class="control-button" 
                @click="playPreviousEpisode"
                :disabled="currentEpisodeIndex <= 0"
              >
                ⏮
              </button>

              <button 
                class="control-button" 
                @click="playNextEpisode"
                :disabled="currentEpisodeIndex >= episodes.length - 1"
              >
                ⏭
              </button>
            </div>

            <div class="controls-right">
              <button class="control-button" @click="toggleLock">
                <span v-if="isLocked">🔒</span>
                <span v-else>🔓</span>
              </button>
              <button class="control-button" @click="toggleFullscreen">
                <span v-if="isFullscreen">⛶</span>
                <span v-else>⛶</span>
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="sidebar" :class="{ open: showSidebar }">
      <div class="sidebar-content">
        <div v-if="detail" class="detail-section">
          <img :src="detail.vod_pic || placeholderImage" class="sidebar-poster" />
          <h2 class="sidebar-title">{{ detail.vod_name }}</h2>
          <div class="sidebar-meta">
            <span v-if="detail.vod_remarks" class="meta-tag">{{ detail.vod_remarks }}</span>
            <span v-if="detail.vod_year" class="meta-tag">{{ detail.vod_year }}</span>
            <span v-if="detail.vod_area" class="meta-tag">{{ detail.vod_area }}</span>
          </div>
          <div v-if="detail.vod_content" class="sidebar-desc">
            {{ detail.vod_content }}
          </div>
        </div>

        <div v-if="episodes.length > 0" class="episodes-section">
          <h3 class="section-title">选集 ({{ episodes.length }})</h3>
          <div class="episode-list">
            <div 
              v-for="(episode, index) in episodes" 
              :key="index"
              class="episode-item"
              :class="{ active: currentEpisodeIndex === index }"
              @click="playEpisode(index)"
            >
              <span class="episode-num">{{ index + 1 }}</span>
              <span class="episode-title">{{ episode.title }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div v-if="showSidebar" class="sidebar-overlay" @click="toggleSidebar"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue';
import { ArrowLeft, List } from '@element-plus/icons-vue';
import Hls from 'hls.js';

interface Props {
  src: string;
  title?: string;
  episodes?: Array<{ url: string; title: string }>;
  detail?: any;
  initialEpisodeIndex?: number;
  initialTime?: number;
  autoPlay?: boolean;
}

interface Emits {
  (e: 'close'): void;
  (e: 'timeupdate', time: number, episodeIndex: number): void;
  (e: 'episodeChange', index: number): void;
}

const props = withDefaults(defineProps<Props>(), {
  episodes: () => [],
  initialEpisodeIndex: 0,
  initialTime: 0,
  autoPlay: false
});

const emit = defineEmits<Emits>();

const videoElement = ref<HTMLVideoElement | null>(null);
const hls = ref<Hls | null>(null);

const isPlaying = ref(false);
const isMuted = ref(false);
const isFullscreen = ref(false);
const isLocked = ref(false);
const volume = ref(1);
const currentTime = ref(0);
const duration = ref(0);
const buffered = ref(0);
const showControls = ref(true);
const showSidebar = ref(false);
const isScrubbing = ref(false);
const controlsHideTimer = ref<number | null>(null);
const currentEpisodeIndex = ref(props.initialEpisodeIndex);
const currentSrc = ref(props.src);
const videoTitle = ref(props.title || '视频播放');

const placeholderImage = 'data:image/svg+xml,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 width=%22400%22 height=%22600%22%3E%3Crect fill=%22%231a1a1a%22 width=%22400%22 height=%22600%22/%3E%3Ctext x=%22200%22 y=%22300%22 fill=%22%234cc2ff%22 font-size=%2248%22 text-anchor=%22middle%22 dominant-baseline=%22middle%22%3E🎬%3C/text%3E%3C/svg%3E';

const playedPercentage = computed(() => {
  if (duration.value === 0) return 0;
  return (currentTime.value / duration.value) * 100;
});

const bufferedPercentage = computed(() => {
  if (!videoElement.value || duration.value === 0) return 0;
  if (videoElement.value.buffered.length === 0) return 0;
  return (videoElement.value.buffered.end(videoElement.value.buffered.length - 1) / duration.value) * 100;
});

function formatTime(seconds: number): string {
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${mins}:${secs.toString().padStart(2, '0')}`;
}

function initPlayer() {
  if (!videoElement.value) return;
  
  const video = videoElement.value;
  const src = currentSrc.value;
  
  if (hls.value) {
    hls.value.destroy();
    hls.value = null;
  }
  
  if (src.includes('.m3u8')) {
    if (Hls.isSupported()) {
      hls.value = new Hls({
        maxBufferLength: 30,
        maxMaxBufferLength: 120,
        maxBufferSize: 120 * 1000 * 1000,
        maxBufferHole: 0.5,
        backBufferLength: 30,
        startFragPrefetch: true,
        startLevel: -1,
        autoStartLoad: true,
        enableWorker: true,
        fragLoadingMaxRetry: 10,
        manifestLoadingMaxRetry: 10,
        levelLoadingMaxRetry: 10,
        fragLoadingRetryDelay: 1000,
        manifestLoadingRetryDelay: 1000,
        levelLoadingRetryDelay: 1000,
        lowBufferWatchdogPeriod: 2,
        highBufferWatchdogPeriod: 1,
        nudgeOffset: 0.1,
        nudgeMaxRetry: 10,
        maxFragLookUpTolerance: 0.25,
        enableSoftwareAES: true
      });
      
      hls.value.loadSource(src);
      hls.value.attachMedia(video);
      
      hls.value.on(Hls.Events.MANIFEST_PARSED, () => {
        if (props.autoPlay) {
          video.play().catch(err => console.log('Auto play failed:', err));
        }
      });
      
      hls.value.on(Hls.Events.ERROR, (event, data) => {
        console.error('HLS error:', data);
        if (data.fatal) {
          switch(data.type) {
            case Hls.ErrorTypes.NETWORK_ERROR:
              console.log('Fatal network error, trying to recover...');
              if (hls.value) {
                setTimeout(() => {
                  if (hls.value) hls.value.startLoad();
                }, 1000);
              }
              break;
            case Hls.ErrorTypes.MEDIA_ERROR:
              console.log('Fatal media error, trying to recover...');
              if (hls.value) {
                setTimeout(() => {
                  if (hls.value) hls.value.recoverMediaError();
                }, 500);
              }
              break;
          }
        } else {
          if (data.details === 'bufferSeekOverHole' || data.details === 'bufferStalledError') {
            console.log('Buffer issue detected, recovering...');
            if (hls.value) {
              setTimeout(() => {
                if (hls.value) {
                  hls.value.recoverMediaError();
                }
              }, 300);
            }
          } else if (data.details === 'manifestParsingError' || data.details === 'bufferFullError') {
            console.log('Non-fatal error, continuing...');
          }
        }
      });
    } else if (video.canPlayType('application/vnd.apple.mpegurl')) {
      video.src = src;
      video.addEventListener('loadedmetadata', () => {
        if (props.autoPlay) {
          video.play().catch(err => console.log('Auto play failed:', err));
        }
      });
    }
  } else {
    video.src = src;
    video.addEventListener('loadedmetadata', () => {
      if (props.autoPlay) {
        video.play().catch(err => console.log('Auto play failed:', err));
      }
    });
  }
  
  if (props.initialTime > 0) {
    video.currentTime = props.initialTime;
  }
}

function togglePlay() {
  if (!videoElement.value || isLocked.value) return;
  if (isPlaying.value) {
    videoElement.value.pause();
  } else {
    videoElement.value.play().catch(err => console.log('Play failed:', err));
  }
}

function toggleMute() {
  if (!videoElement.value || isLocked.value) return;
  isMuted.value = !isMuted.value;
  videoElement.value.muted = isMuted.value;
}

function onVolumeChange() {
  if (!videoElement.value || isLocked.value) return;
  videoElement.value.volume = volume.value;
  if (volume.value > 0) {
    isMuted.value = false;
    videoElement.value.muted = false;
  }
}

function toggleFullscreen() {
  if (isLocked.value) return;
  if (!document.fullscreenElement) {
    document.documentElement.requestFullscreen();
  } else {
    document.exitFullscreen();
  }
}

function handleFullscreenChange() {
  isFullscreen.value = !!document.fullscreenElement;
}

const handleKeydown = (e: KeyboardEvent) => {
  if (isLocked.value) return;
  
  if (e.target.tagName === 'INPUT' || e.target.tagName === 'TEXTAREA') return;
  
  if (e.key === 'ArrowLeft' && e.altKey && currentEpisodeIndex.value > 0) {
    playPreviousEpisode();
    e.preventDefault();
  }
  if (e.key === 'ArrowRight' && e.altKey && currentEpisodeIndex.value < props.episodes.length - 1) {
    playNextEpisode();
    e.preventDefault();
  }
  if (e.key === ' ' || e.key === 'k') {
    togglePlay();
    e.preventDefault();
  }
  if (e.key === 'Escape') {
    if (isFullscreen.value) {
      toggleFullscreen();
    } else {
      handleBack();
    }
    e.preventDefault();
  }
  if (e.key === 'f' || e.key === 'F') {
    toggleFullscreen();
    e.preventDefault();
  }
  if (e.key === 'ArrowLeft' && !e.altKey) {
    if (videoElement.value) {
      videoElement.value.currentTime = Math.max(0, videoElement.value.currentTime - 10);
    }
    e.preventDefault();
  }
  if (e.key === 'ArrowRight' && !e.altKey) {
    if (videoElement.value) {
      videoElement.value.currentTime = Math.min(duration.value, videoElement.value.currentTime + 10);
    }
    e.preventDefault();
  }
  if (e.key === 'ArrowUp') {
    if (videoElement.value) {
      volume.value = Math.min(1, volume.value + 0.1);
      videoElement.value.volume = volume.value;
      if (volume.value > 0) {
        isMuted.value = false;
        videoElement.value.muted = false;
      }
    }
    e.preventDefault();
  }
  if (e.key === 'ArrowDown') {
    if (videoElement.value) {
      volume.value = Math.max(0, volume.value - 0.1);
      videoElement.value.volume = volume.value;
    }
    e.preventDefault();
  }
  if (e.key === 'm' || e.key === 'M') {
    toggleMute();
    e.preventDefault();
  }
  if (e.key === 'l' || e.key === 'L') {
    toggleLock();
    e.preventDefault();
  }
  if (e.key === 's' || e.key === 'S') {
    toggleSidebar();
    e.preventDefault();
  }
};

watch(() => props.src, (newSrc) => {
  if (newSrc) {
    currentSrc.value = newSrc;
    initPlayer();
  }
});

watch(() => props.episodes, (newEpisodes) => {
  if (newEpisodes.length > 0 && currentEpisodeIndex.value >= newEpisodes.length) {
    currentEpisodeIndex.value = 0;
  }
});

onMounted(() => {
  if (currentSrc.value) {
    initPlayer();
  }
  document.addEventListener('fullscreenchange', handleFullscreenChange);
  document.addEventListener('keydown', handleKeydown);
});

onUnmounted(() => {
  if (hls.value) {
    hls.value.destroy();
  }
  document.removeEventListener('fullscreenchange', handleFullscreenChange);
  document.removeEventListener('keydown', handleKeydown);
  if (controlsHideTimer.value) {
    clearTimeout(controlsHideTimer.value);
  }
});

function onTimeUpdate() {
  if (!videoElement.value) return;
  currentTime.value = videoElement.value.currentTime;
  emit('timeupdate', currentTime.value, currentEpisodeIndex.value);
}

function onLoadedMetadata() {
  if (!videoElement.value) return;
  duration.value = videoElement.value.duration;
  if (props.initialTime > 0) {
    videoElement.value.currentTime = props.initialTime;
  }
}

function onPlay() {
  isPlaying.value = true;
}

function onPause() {
  isPlaying.value = false;
}

function onEnded() {
  if (currentEpisodeIndex.value < props.episodes.length - 1) {
    playNextEpisode();
  }
}

function startScrubbing(e: MouseEvent) {
  if (isLocked.value || !videoElement.value) return;
  isScrubbing.value = true;
  updateProgress(e);
  
  const handleMouseMove = (moveEvent: MouseEvent) => {
    updateProgress(moveEvent);
  };
  
  const handleMouseUp = () => {
    isScrubbing.value = false;
    document.removeEventListener('mousemove', handleMouseMove);
    document.removeEventListener('mouseup', handleMouseUp);
  };
  
  document.addEventListener('mousemove', handleMouseMove);
  document.addEventListener('mouseup', handleMouseUp);
}

function updateProgress(e: MouseEvent) {
  if (!videoElement.value) return;
  const rect = (e.target as HTMLElement).getBoundingClientRect();
  const pos = (e.clientX - rect.left) / rect.width;
  videoElement.value.currentTime = pos * duration.value;
}

function onControlsMouseMove() {
  if (isLocked.value) return;
  showControls.value = true;
  if (controlsHideTimer.value) {
    clearTimeout(controlsHideTimer.value);
  }
  controlsHideTimer.value = window.setTimeout(() => {
    if (isPlaying.value && !isLocked.value) {
      showControls.value = false;
    }
  }, 3000);
}

function onControlsMouseLeave() {
  if (isPlaying.value && !isLocked.value) {
    showControls.value = false;
  }
}

function toggleLock() {
  isLocked.value = !isLocked.value;
}

function toggleSidebar() {
  showSidebar.value = !showSidebar.value;
}

function playEpisode(index: number) {
  if (isLocked.value) return;
  if (index >= 0 && index < props.episodes.length) {
    currentEpisodeIndex.value = index;
    currentSrc.value = props.episodes[index].url;
    emit('episodeChange', index);
  }
}

function playPreviousEpisode() {
  if (currentEpisodeIndex.value > 0) {
    playEpisode(currentEpisodeIndex.value - 1);
  }
}

function playNextEpisode() {
  if (currentEpisodeIndex.value < props.episodes.length - 1) {
    playEpisode(currentEpisodeIndex.value + 1);
  }
}

function handleBack() {
  emit('close');
}
</script>

<style scoped>
.fullscreen-player {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: #000;
  z-index: 9999;
  display: flex;
  overflow: hidden;
}

.player-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  position: relative;
}

.player-top-bar {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 60px;
  background: linear-gradient(180deg, rgba(0,0,0,0.8) 0%, rgba(0,0,0,0) 100%);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 20px;
  z-index: 10;
}

.top-button {
  display: flex;
  align-items: center;
  gap: 8px;
  background: rgba(255,255,255,0.1);
  border: 1px solid rgba(255,255,255,0.2);
  color: #fff;
  font-size: 14px;
  padding: 8px 16px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
  backdrop-filter: blur(10px);
}

.top-button:hover {
  background: rgba(255,255,255,0.2);
  border-color: rgba(255,255,255,0.3);
}

.player-title {
  color: #fff;
  font-size: 16px;
  font-weight: 500;
  max-width: 40%;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.video-container {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #000;
}

.video-element {
  width: 100%;
  height: 100%;
  object-fit: contain;
  cursor: pointer;
}

.controls-overlay {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  background: linear-gradient(transparent, rgba(0, 0, 0, 0.9));
  opacity: 0;
  transition: opacity 0.3s ease;
  padding: 20px 24px 24px;
}

.controls-overlay.visible {
  opacity: 1;
}

.progress-bar {
  position: relative;
  width: 100%;
  height: 5px;
  background: rgba(255, 255, 255, 0.2);
  border-radius: 3px;
  cursor: pointer;
  margin-bottom: 16px;
  transition: height 0.2s ease;
}

.progress-bar:hover {
  height: 8px;
}

.progress-buffered {
  position: absolute;
  top: 0;
  left: 0;
  height: 100%;
  background: rgba(255, 255, 255, 0.3);
  border-radius: 3px;
}

.progress-played {
  position: absolute;
  top: 0;
  left: 0;
  height: 100%;
  background: #4cc2ff;
  border-radius: 3px;
}

.progress-handle {
  position: absolute;
  top: 50%;
  transform: translate(-50%, -50%);
  width: 14px;
  height: 14px;
  background: #4cc2ff;
  border-radius: 50%;
  opacity: 0;
  transition: opacity 0.2s ease;
}

.progress-bar:hover .progress-handle {
  opacity: 1;
}

.controls-bottom {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.controls-left,
.controls-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.control-button {
  background: rgba(255,255,255,0.1);
  border: none;
  color: #fff;
  width: 40px;
  height: 40px;
  border-radius: 50%;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 18px;
  transition: all 0.2s ease;
}

.control-button:hover:not(:disabled) {
  background: rgba(255,255,255,0.2);
  transform: scale(1.1);
}

.control-button:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.volume-control {
  display: flex;
  align-items: center;
  gap: 8px;
}

.volume-slider {
  width: 100px;
  cursor: pointer;
  -webkit-appearance: none;
  appearance: none;
  background: transparent;
  height: 4px;
}

.volume-slider::-webkit-slider-runnable-track {
  height: 4px;
  background: rgba(255,255,255,0.3);
  border-radius: 2px;
}

.volume-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 14px;
  height: 14px;
  border-radius: 50%;
  background: #fff;
  cursor: pointer;
  margin-top: -5px;
}

.volume-slider::-moz-range-track {
  height: 4px;
  background: rgba(255,255,255,0.3);
  border-radius: 2px;
}

.volume-slider::-moz-range-thumb {
  width: 14px;
  height: 14px;
  border-radius: 50%;
  background: #fff;
  cursor: pointer;
  border: none;
}

.time-display {
  color: #fff;
  font-size: 14px;
  font-family: 'Segoe UI', system-ui;
  margin-left: 8px;
}

.time-separator {
  margin: 0 6px;
  opacity: 0.6;
}

.sidebar {
  position: fixed;
  top: 0;
  right: 0;
  width: 360px;
  height: 100vh;
  background: rgba(20, 20, 20, 0.98);
  backdrop-filter: blur(20px);
  border-left: 1px solid rgba(255,255,255,0.1);
  transition: transform 0.3s ease;
  overflow-y: auto;
  z-index: 10000;
  transform: translateX(100%);
}

.sidebar.open {
  transform: translateX(0);
}

.sidebar-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.5);
  z-index: 9999;
}

.sidebar-content {
  padding: 24px;
}

.sidebar-poster {
  width: 100%;
  border-radius: 12px;
  margin-bottom: 16px;
}

.sidebar-title {
  font-size: 22px;
  font-weight: 600;
  color: #fff;
  margin: 0 0 12px;
  line-height: 1.3;
}

.sidebar-meta {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
  margin-bottom: 16px;
}

.meta-tag {
  font-size: 12px;
  color: rgba(255,255,255,0.7);
  padding: 4px 10px;
  background: rgba(255,255,255,0.1);
  border-radius: 12px;
}

.sidebar-desc {
  font-size: 13px;
  color: rgba(255,255,255,0.7);
  line-height: 1.6;
  margin-bottom: 24px;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  color: #fff;
  margin: 0 0 16px;
}

.episode-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.episode-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: rgba(255,255,255,0.05);
  border: 1px solid rgba(255,255,255,0.08);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.episode-item:hover {
  background: rgba(255,255,255,0.1);
  border-color: rgba(255,255,255,0.2);
}

.episode-item.active {
  background: rgba(76, 194, 255, 0.2);
  border-color: #4cc2ff;
}

.episode-num {
  flex-shrink: 0;
  width: 32px;
  height: 32px;
  background: rgba(255,255,255,0.1);
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 13px;
  font-weight: 600;
  color: #fff;
}

.episode-item.active .episode-num {
  background: #4cc2ff;
  color: #000;
}

.episode-title {
  flex: 1;
  font-size: 14px;
  color: #fff;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
