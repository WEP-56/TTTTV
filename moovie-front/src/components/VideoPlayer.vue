<template>
  <div class="video-player" ref="playerContainer">
    <div class="video-wrapper">
      <video
        ref="videoElement"
        class="video-element"
        :class="{ 'fullscreen': isFullscreen }"
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
          </div>
          
          <div class="controls-right">
            <button class="control-button" @click="toggleFullscreen">
              <span v-if="isFullscreen">⛶</span>
              <span v-else>⛶</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue';
import Hls from 'hls.js';

interface Props {
  src: string;
  sources?: Array<{ url: string; name: string }>;
  episodes?: Array<{ url: string; title: string }>;
  autoPlay?: boolean;
  initialTime?: number;
}

interface Emits {
  (e: 'play'): void;
  (e: 'pause'): void;
  (e: 'timeupdate', time: number): void;
  (e: 'ended'): void;
}

const props = withDefaults(defineProps<Props>(), {
  sources: () => [],
  episodes: () => [],
  autoPlay: false,
  initialTime: 0
});

const emit = defineEmits<Emits>();

const videoElement = ref<HTMLVideoElement | null>(null);
const playerContainer = ref<HTMLDivElement | null>(null);
const hls = ref<Hls | null>(null);

const isPlaying = ref(false);
const isMuted = ref(false);
const isFullscreen = ref(false);
const volume = ref(1);
const currentTime = ref(0);
const duration = ref(0);
const buffered = ref(0);
const showControls = ref(true);
const isScrubbing = ref(false);
const controlsHideTimer = ref<number | null>(null);

const playedPercentage = computed(() => {
  if (duration.value === 0) return 0;
  return (currentTime.value / duration.value) * 100;
});

const bufferedPercentage = computed(() => {
  if (!videoElement.value || duration.value === 0) return 0;
  if (videoElement.value.buffered.length === 0) return 0;
  const bufferedEnd = videoElement.value.buffered.end(videoElement.value.buffered.length - 1);
  return (bufferedEnd / duration.value) * 100;
});

function formatTime(seconds: number): string {
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  const s = Math.floor(seconds % 60);
  if (h > 0) {
    return `${h}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
  }
  return `${m}:${s.toString().padStart(2, '0')}`;
}

function initPlayer() {
  console.log('VideoPlayer: initPlayer called with src:', props.src);
  
  if (!videoElement.value) {
    console.log('VideoPlayer: videoElement is null');
    return;
  }
  
  const video = videoElement.value;
  const src = props.src;
  
  if (hls.value) {
    hls.value.destroy();
    hls.value = null;
  }
  
  if (src.includes('.m3u8')) {
    if (Hls.isSupported()) {
      hls.value = new Hls({
        maxBufferLength: 15,
        maxMaxBufferLength: 60,
        maxBufferSize: 60 * 1000 * 1000,
        maxBufferHole: 1.5,
        backBufferLength: 10,
        startFragPrefetch: true,
        startLevel: -1,
        autoStartLoad: true,
        enableWorker: true,
        fragLoadingMaxRetry: 5,
        manifestLoadingMaxRetry: 5
      });
      
      hls.value.loadSource(src);
      hls.value.attachMedia(video);
      
      hls.value.on(Hls.Events.MANIFEST_PARSED, () => {
        console.log('VideoPlayer: HLS manifest parsed');
        if (props.autoPlay) {
          video.play().catch(err => console.log('VideoPlayer: Auto play failed:', err));
        }
        
        setTimeout(() => {
          if (hls.value) {
            hls.value.config.maxBufferLength = 40;
            hls.value.config.maxMaxBufferLength = 90;
          }
        }, 8000);
      });
      
      hls.value.on(Hls.Events.ERROR, (event, data) => {
        console.error('VideoPlayer: HLS error:', data);
        if (data.fatal) {
          switch(data.type) {
            case Hls.ErrorTypes.NETWORK_ERROR:
              console.error('网络错误,尝试恢复...');
              if (hls.value) {
                hls.value.startLoad();
              }
              break;
            case Hls.ErrorTypes.MEDIA_ERROR:
              console.error('媒体错误,尝试恢复...');
              if (hls.value) {
                hls.value.recoverMediaError();
              }
              break;
            default:
              console.error('无法恢复的错误');
              break;
          }
        }
      });
      
      hls.value.on(Hls.Events.LEVEL_LOADED, (event, data) => {
        console.log('VideoPlayer: HLS level loaded, 可以开始播放');
      });
    } else if (video.canPlayType('application/vnd.apple.mpegurl')) {
      video.src = src;
      video.addEventListener('loadedmetadata', () => {
        if (props.autoPlay) {
          video.play().catch(err => console.log('VideoPlayer: Auto play failed:', err));
        }
      });
    }
  } else {
    video.src = src;
    video.addEventListener('loadedmetadata', () => {
      console.log('VideoPlayer: Video metadata loaded');
      if (props.autoPlay) {
        video.play().catch(err => console.log('VideoPlayer: Auto play failed:', err));
      }
    });
  }
  
  if (props.initialTime > 0) {
    video.currentTime = props.initialTime;
  }
  
  console.log('VideoPlayer: initPlayer completed');
}

function togglePlay() {
  if (!videoElement.value) return;
  if (isPlaying.value) {
    videoElement.value.pause();
  } else {
    videoElement.value.play().catch(err => console.log('VideoPlayer: Play failed:', err));
  }
}

function onPlay() {
  isPlaying.value = true;
  emit('play');
}

function onPause() {
  isPlaying.value = false;
  emit('pause');
}

function onEnded() {
  isPlaying.value = false;
  emit('ended');
}

function onTimeUpdate() {
  if (!videoElement.value || isScrubbing.value) return;
  currentTime.value = videoElement.value.currentTime;
  emit('timeupdate', currentTime.value);
}

function onLoadedMetadata() {
  if (!videoElement.value) return;
  duration.value = videoElement.value.duration;
  console.log('VideoPlayer: Duration set to', duration.value);
}

function onVolumeChange() {
  if (!videoElement.value) return;
  videoElement.value.volume = volume.value;
  if (volume.value > 0) {
    isMuted.value = false;
    videoElement.value.muted = false;
  }
}

function toggleMute() {
  if (!videoElement.value) return;
  isMuted.value = !isMuted.value;
  videoElement.value.muted = isMuted.value;
  if (!isMuted.value && volume.value === 0) {
    volume.value = 0.5;
    videoElement.value.volume = 0.5;
  }
}

function startScrubbing(event: MouseEvent) {
  if (!videoElement.value) return;
  isScrubbing.value = true;
  
  const progressBar = event.currentTarget as HTMLElement;
  const rect = progressBar.getBoundingClientRect();
  const percent = (event.clientX - rect.left) / rect.width;
  currentTime.value = percent * duration.value;
  videoElement.value.currentTime = currentTime.value;
  
  const handleMouseMove = (e: MouseEvent) => {
    const percent = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
    currentTime.value = percent * duration.value;
    videoElement.value.currentTime = currentTime.value;
  };
  
  const handleMouseUp = () => {
    isScrubbing.value = false;
    document.removeEventListener('mousemove', handleMouseMove);
    document.removeEventListener('mouseup', handleMouseUp);
  };
  
  document.addEventListener('mousemove', handleMouseMove);
  document.addEventListener('mouseup', handleMouseUp);
}

function toggleFullscreen() {
  if (!playerContainer.value) return;
  
  if (!document.fullscreenElement) {
    playerContainer.value.requestFullscreen().then(() => {
      isFullscreen.value = true;
    }).catch(err => console.log('VideoPlayer: Fullscreen failed:', err));
  } else {
    document.exitFullscreen().then(() => {
      isFullscreen.value = false;
    }).catch(err => console.log('VideoPlayer: Exit fullscreen failed:', err));
  }
}

function onControlsMouseMove() {
  showControls.value = true;
  if (controlsHideTimer.value) {
    clearTimeout(controlsHideTimer.value);
  }
  controlsHideTimer.value = window.setTimeout(() => {
    if (isPlaying.value) {
      showControls.value = false;
    }
  }, 3000);
}

function onControlsMouseLeave() {
  if (isPlaying.value) {
    showControls.value = false;
  }
}

const handleFullscreenChange = () => {
  isFullscreen.value = !!document.fullscreenElement;
};

watch(() => props.src, (newSrc) => {
  console.log('VideoPlayer: src changed to', newSrc);
  if (newSrc) {
    initPlayer();
  }
});

onMounted(() => {
  console.log('VideoPlayer: Mounted');
  if (props.src) {
    initPlayer();
  }
  document.addEventListener('fullscreenchange', handleFullscreenChange);
});

onUnmounted(() => {
  if (hls.value) {
    hls.value.destroy();
  }
  document.removeEventListener('fullscreenchange', handleFullscreenChange);
  if (controlsHideTimer.value) {
    clearTimeout(controlsHideTimer.value);
  }
});
</script>

<style scoped>
.video-player {
  width: 100%;
  background: #000;
  border-radius: var(--el-border-radius-base);
  overflow: hidden;
  min-height: 200px;
}

.video-wrapper {
  position: relative;
  width: 100%;
  aspect-ratio: 16 / 9;
  background: #000;
}

.video-wrapper.fullscreen {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  z-index: 9999;
}

.video-element {
  width: 100%;
  height: 100%;
  object-fit: contain;
  cursor: pointer;
  background: #000;
}

.controls-overlay {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  background: linear-gradient(transparent, rgba(0, 0, 0, 0.8));
  opacity: 0;
  transition: opacity 0.3s ease;
  padding: 20px 16px 16px;
}

.controls-overlay.visible {
  opacity: 1;
}

.progress-bar {
  position: relative;
  width: 100%;
  height: 4px;
  background: rgba(255, 255, 255, 0.2);
  border-radius: 2px;
  cursor: pointer;
  margin-bottom: 12px;
}

.progress-bar:hover {
  height: 6px;
}

.progress-buffered {
  position: absolute;
  top: 0;
  left: 0;
  height: 100%;
  background: rgba(255, 255, 255, 0.3);
  border-radius: 2px;
}

.progress-played {
  position: absolute;
  top: 0;
  left: 0;
  height: 100%;
  background: var(--el-color-primary);
  border-radius: 2px;
}

.progress-handle {
  position: absolute;
  top: 50%;
  transform: translate(-50%, -50%);
  width: 12px;
  height: 12px;
  background: var(--el-color-primary);
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
  background: transparent;
  border: none;
  color: #fff;
  font-size: 20px;
  width: 36px;
  height: 36px;
  border-radius: 50%;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background 0.2s ease;
}

.control-button:hover {
  background: rgba(255, 255, 255, 0.15);
}

.volume-control {
  display: flex;
  align-items: center;
  gap: 4px;
}

.volume-slider {
  width: 80px;
  height: 4px;
  border-radius: 2px;
  background: rgba(255, 255, 255, 0.2);
  outline: none;
  cursor: pointer;
}

.volume-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: #fff;
  cursor: pointer;
}

.time-display {
  color: #fff;
  font-size: 13px;
  font-family: 'Segoe UI', system-ui;
  margin-left: 8px;
}

.time-separator {
  margin: 0 4px;
  opacity: 0.6;
}
</style>
