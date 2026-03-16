<template>
  <video
    ref="videoEl"
    class="live-video"
    controls
    playsinline
    preload="auto"
  ></video>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref, watch } from 'vue';
import Hls from 'hls.js';

const props = withDefaults(
  defineProps<{
    src: string;
    autoPlay?: boolean;
  }>(),
  {
    autoPlay: true,
  }
);

const videoEl = ref<HTMLVideoElement | null>(null);
let hls: Hls | null = null;

function destroyHls() {
  if (hls) {
    hls.destroy();
    hls = null;
  }
}

function load() {
  const video = videoEl.value;
  if (!video) return;

  destroyHls();

  if (!props.src) {
    video.removeAttribute('src');
    video.load();
    return;
  }

  if (props.src.includes('.m3u8') && Hls.isSupported()) {
    hls = new Hls({
      lowLatencyMode: true,
      backBufferLength: 5,
      maxBufferLength: 15,
      maxMaxBufferLength: 30,
      enableWorker: true,
    });
    hls.loadSource(props.src);
    hls.attachMedia(video);
  } else {
    video.src = props.src;
  }

  if (props.autoPlay) {
    // Autoplay might be blocked, ignore errors.
    video.play().catch(() => {});
  }
}

watch(
  () => props.src,
  () => {
    load();
  }
);

onMounted(() => load());
onUnmounted(() => destroyHls());
</script>

<style scoped>
.live-video {
  width: 100%;
  height: 100%;
  background: #000;
  object-fit: contain;
}
</style>

