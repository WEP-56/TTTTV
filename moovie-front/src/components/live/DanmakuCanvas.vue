<template>
  <canvas ref="canvasEl" class="danmaku-canvas"></canvas>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref, watch } from 'vue';
import type { LiveMessage } from '../../types';

const props = defineProps<{
  wsUrl: string;
  opacity: number;
  fontSize: number;
  speed: number;
}>();

type Bullet = {
  text: string;
  color: string;
  x: number;
  y: number;
  width: number;
  speed: number;
  createdAt: number;
};

const canvasEl = ref<HTMLCanvasElement | null>(null);
let ctx: CanvasRenderingContext2D | null = null;
let rafId: number | null = null;
let ws: WebSocket | null = null;
let resizeObs: ResizeObserver | null = null;
let bullets: Bullet[] = [];
let reconnectTimer: number | null = null;
let reconnectAttempts = 0;
let allowReconnect = true;

let lastTs = 0;
let trackIndex = 0;

function setupCanvasSize() {
  const canvas = canvasEl.value;
  if (!canvas) return;
  const parent = canvas.parentElement;
  if (!parent) return;

  const rect = parent.getBoundingClientRect();
  const dpr = window.devicePixelRatio || 1;
  canvas.width = Math.max(1, Math.floor(rect.width * dpr));
  canvas.height = Math.max(1, Math.floor(rect.height * dpr));
  canvas.style.width = `${rect.width}px`;
  canvas.style.height = `${rect.height}px`;

  ctx = canvas.getContext('2d');
  if (ctx) {
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    ctx.textBaseline = 'top';
  }
}

function connect() {
  if (!props.wsUrl) return;
  disconnect();
  clearReconnectTimer();

  try {
    ws = new WebSocket(props.wsUrl);
    ws.onopen = () => {
      reconnectAttempts = 0;
    };
    ws.onmessage = (event) => {
      try {
        const msg: LiveMessage = JSON.parse(event.data);
        if (msg.type !== 'chat') return;
        enqueue(msg);
      } catch {
        // ignore
      }
    };
    ws.onerror = () => {
      scheduleReconnect();
    };
    ws.onclose = () => {
      ws = null;
      scheduleReconnect();
    };
  } catch {
    ws = null;
    scheduleReconnect();
  }
}

function disconnect() {
  clearReconnectTimer();
  if (ws) {
    try {
      ws.onopen = null;
      ws.onmessage = null;
      ws.onerror = null;
      ws.onclose = null;
      ws.close();
    } catch {
      // ignore
    }
    ws = null;
  }
}

function clearReconnectTimer() {
  if (reconnectTimer != null) {
    clearTimeout(reconnectTimer);
    reconnectTimer = null;
  }
}

function scheduleReconnect() {
  if (!allowReconnect) return;
  if (!props.wsUrl) return;
  if (reconnectTimer != null) return;

  const delay = Math.min(30000, Math.floor(1000 * Math.pow(1.8, reconnectAttempts)));
  reconnectAttempts = Math.min(reconnectAttempts + 1, 10);
  reconnectTimer = window.setTimeout(() => {
    reconnectTimer = null;
    connect();
  }, delay);
}

function enqueue(msg: LiveMessage) {
  const canvas = canvasEl.value;
  if (!canvas || !ctx) return;

  const text = `${msg.user_name}: ${msg.message}`;
  const color = msg.color ? `rgb(${msg.color.r},${msg.color.g},${msg.color.b})` : 'rgb(255,255,255)';

  ctx.font = `${props.fontSize}px "Segoe UI", system-ui, sans-serif`;
  const width = ctx.measureText(text).width;

  const height = canvas.clientHeight;
  const trackHeight = props.fontSize + 6;
  const tracks = Math.max(1, Math.floor(height / trackHeight));
  const y = (trackIndex % tracks) * trackHeight + 4;
  trackIndex++;

  bullets.push({
    text,
    color,
    x: canvas.clientWidth + 10,
    y,
    width,
    speed: props.speed,
    createdAt: Date.now(),
  });

  // basic cap
  if (bullets.length > 200) {
    bullets = bullets.slice(bullets.length - 200);
  }
}

function tick(ts: number) {
  const canvas = canvasEl.value;
  if (!canvas || !ctx) return;

  if (!lastTs) lastTs = ts;
  const dt = Math.min(0.05, (ts - lastTs) / 1000);
  lastTs = ts;

  ctx.clearRect(0, 0, canvas.clientWidth, canvas.clientHeight);
  ctx.globalAlpha = Math.max(0.05, Math.min(1, props.opacity));
  ctx.font = `${props.fontSize}px "Segoe UI", system-ui, sans-serif`;

  const toKeep: Bullet[] = [];
  for (const b of bullets) {
    const x = b.x - b.speed * dt;
    b.x = x;
    if (x + b.width < -20) continue;
    ctx.fillStyle = b.color;
    ctx.fillText(b.text, x, b.y);
    toKeep.push(b);
  }
  bullets = toKeep;

  rafId = requestAnimationFrame(tick);
}

function start() {
  if (rafId) return;
  lastTs = 0;
  rafId = requestAnimationFrame(tick);
}

function stop() {
  if (rafId) {
    cancelAnimationFrame(rafId);
    rafId = null;
  }
  bullets = [];
  if (ctx && canvasEl.value) {
    ctx.clearRect(0, 0, canvasEl.value.clientWidth, canvasEl.value.clientHeight);
  }
}

watch(
  () => props.wsUrl,
  () => {
    connect();
  }
);

watch(
  () => [props.opacity, props.fontSize, props.speed],
  () => {
    // redraw with new style
  }
);

onMounted(() => {
  setupCanvasSize();
  start();
  connect();

  const canvas = canvasEl.value;
  if (canvas && canvas.parentElement) {
    resizeObs = new ResizeObserver(() => setupCanvasSize());
    resizeObs.observe(canvas.parentElement);
  }
});

onUnmounted(() => {
  allowReconnect = false;
  disconnect();
  stop();
  resizeObs?.disconnect();
  resizeObs = null;
});
</script>

<style scoped>
.danmaku-canvas {
  width: 100%;
  height: 100%;
}
</style>
