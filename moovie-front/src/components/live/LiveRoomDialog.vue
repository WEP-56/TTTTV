<template>
  <el-dialog
    v-model="visible"
    :title="detail?.title || '直播间'"
    width="980px"
    class="live-room-dialog"
    :close-on-click-modal="false"
    @closed="handleClosed"
  >
    <div v-if="loading" class="loading-area">
      <div class="loading-spinner"></div>
      <span class="loading-text">正在加载直播间...</span>
    </div>

    <div v-else-if="!detail" class="empty-area">
      <span class="empty-text">直播间加载失败</span>
    </div>

    <div v-else class="live-room-content">
      <div class="live-player-area">
        <div class="live-player-container">
          <LiveVideoPlayer :src="currentSrc" />
          <DanmakuCanvas
            v-if="danmakuEnabled && danmakuWsUrl"
            class="danmaku-layer"
            :ws-url="danmakuWsUrl"
            :opacity="danmakuOpacity"
            :font-size="danmakuFontSize"
            :speed="danmakuSpeed"
          />
        </div>

        <div class="live-controls">
          <div class="control-row">
            <span class="control-label">清晰度</span>
            <el-select
              v-model="selectedQualityId"
              placeholder="选择清晰度"
              size="small"
              style="width: 220px"
              @change="refreshPlay"
              :disabled="qualities.length === 0"
            >
              <el-option
                v-for="q in qualities"
                :key="q.id"
                :label="q.name"
                :value="q.id"
              />
            </el-select>
          </div>

          <div class="control-row" v-if="playUrls.length > 1">
            <span class="control-label">线路</span>
            <el-select v-model="selectedLineIndex" size="small" style="width: 220px" @change="updateSrcFromLine">
              <el-option v-for="(u, idx) in playUrls" :key="idx" :label="`线路 ${idx + 1}`" :value="idx" />
            </el-select>
          </div>

          <div class="control-row">
            <el-switch v-model="danmakuEnabled" />
            <span class="control-label" style="margin-left: 8px;">弹幕</span>
            <el-button size="small" style="margin-left: 12px;" @click="showDanmakuSettings = true">设置</el-button>
          </div>

          <div class="control-row">
            <el-button size="small" type="primary" :loading="refreshing" @click="refreshPlay">刷新播放</el-button>
            <el-button
              size="small"
              :type="isFavorited ? 'danger' : 'default'"
              @click="toggleFavorite"
            >
              {{ isFavorited ? '已收藏' : '收藏' }}
            </el-button>
          </div>
        </div>
      </div>

      <div class="live-info">
        <div class="info-row">
          <img :src="detail.user_avatar || placeholderAvatar" class="avatar" />
          <div class="info-main">
            <div class="anchor">{{ detail.user_name }}</div>
            <div class="meta">
              <span>在线：{{ formatOnline(detail.online) }}</span>
              <span style="margin-left: 12px;">平台：{{ detail.platform }}</span>
              <span v-if="!detail.status" style="margin-left: 12px;">(未开播)</span>
            </div>
          </div>
        </div>

        <div v-if="detail.introduction" class="info-block">
          <div class="block-title">简介</div>
          <div class="block-text">{{ detail.introduction }}</div>
        </div>
      </div>
    </div>

    <el-drawer
      v-model="showDanmakuSettings"
      title="弹幕设置"
      direction="rtl"
      size="360px"
    >
      <div class="danmaku-settings">
        <div class="setting-line">
          <span>透明度</span>
          <el-slider v-model="danmakuOpacity" :min="0.1" :max="1" :step="0.05" />
        </div>
        <div class="setting-line">
          <span>字号</span>
          <el-slider v-model="danmakuFontSize" :min="14" :max="40" :step="1" />
        </div>
        <div class="setting-line">
          <span>速度</span>
          <el-slider v-model="danmakuSpeed" :min="60" :max="240" :step="10" />
        </div>
        <div class="setting-line">
          <el-button type="primary" @click="saveDanmakuSettings">保存</el-button>
          <el-button @click="resetDanmakuSettings" style="margin-left: 8px;">重置</el-button>
        </div>
      </div>
    </el-drawer>
  </el-dialog>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { ElMessage } from 'element-plus';
import { useLiveStore } from '../../stores/live';
import { useLiveFavoritesStore } from '../../stores/liveFavorites';
import { useLiveHistoryStore } from '../../stores/liveHistory';
import LiveVideoPlayer from './LiveVideoPlayer.vue';
import DanmakuCanvas from './DanmakuCanvas.vue';
import type { LivePlayQuality, LiveRoomDetail } from '../../types';

const props = defineProps<{
  modelValue: boolean;
  platform: string;
  roomId: string;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', v: boolean): void;
}>();

const liveStore = useLiveStore();
const liveFavoritesStore = useLiveFavoritesStore();
const liveHistoryStore = useLiveHistoryStore();

const visible = computed({
  get: () => props.modelValue,
  set: (v: boolean) => emit('update:modelValue', v),
});

const loading = ref(false);
const refreshing = ref(false);
const detail = ref<LiveRoomDetail | null>(null);
const qualities = ref<LivePlayQuality[]>([]);
const selectedQualityId = ref('');
const playUrls = ref<string[]>([]);
const selectedLineIndex = ref(0);
const currentSrc = ref('');

const placeholderAvatar =
  'data:image/svg+xml,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 width=%22120%22 height=%22120%22%3E%3Crect fill=%22%23222222%22 width=%22120%22 height=%22120%22/%3E%3Ctext x=%2260%22 y=%2260%22 fill=%22%234cc2ff%22 font-size=%2224%22 text-anchor=%22middle%22 dominant-baseline=%22middle%22%3EUser%3C/text%3E%3C/svg%3E';

const danmakuEnabled = ref(true);
const danmakuOpacity = ref(0.85);
const danmakuFontSize = ref(22);
const danmakuSpeed = ref(120);
const showDanmakuSettings = ref(false);

const danmakuWsUrl = computed(() => {
  const rid = detail.value?.room_id || props.roomId;
  if (!props.platform || !rid) return '';
  return liveStore.danmakuWsUrl(props.platform, rid);
});

const isFavorited = computed(() => {
  const platform = detail.value?.platform || props.platform;
  const rid = detail.value?.room_id || props.roomId;
  return liveFavoritesStore.isFavoriteLocally(platform, rid);
});

watch(
  () => visible.value,
  async (open) => {
    if (!open) return;
    await initRoom();
  }
);

async function initRoom() {
  loading.value = true;
  try {
    await Promise.all([
      liveFavoritesStore.fetchFavorites(),
      liveHistoryStore.fetchHistory(),
    ]);

    detail.value = await liveStore.getDetail(props.platform, props.roomId);
    if (!detail.value) return;

    qualities.value = await liveStore.getQualities(props.platform, detail.value.room_id);
    selectedQualityId.value = qualities.value[0]?.id || '';
    selectedLineIndex.value = 0;

    await refreshPlay();

    await liveHistoryStore.addHistory({
      platform: detail.value.platform,
      room_id: detail.value.room_id,
      title: detail.value.title,
      cover: detail.value.cover,
      user_name: detail.value.user_name,
      user_avatar: detail.value.user_avatar,
    });
  } finally {
    loading.value = false;
  }
}

async function refreshPlay() {
  if (!detail.value) return;
  refreshing.value = true;
  try {
    const qid = selectedQualityId.value || qualities.value[0]?.id || '';
    if (!qid) {
      ElMessage.warning('未找到可用清晰度');
      return;
    }
    selectedQualityId.value = qid;

    const play = await liveStore.getPlay(props.platform, detail.value.room_id, qid);
    playUrls.value = play?.urls || [];
    if (playUrls.value.length === 0) {
      ElMessage.error('未获取到播放地址');
      return;
    }
    selectedLineIndex.value = 0;
    updateSrcFromLine();
  } finally {
    refreshing.value = false;
  }
}

function updateSrcFromLine() {
  if (!detail.value) return;
  const url =
    playUrls.value.find((u, idx) => idx === selectedLineIndex.value && u.includes('.m3u8')) ||
    playUrls.value[selectedLineIndex.value] ||
    '';
  currentSrc.value = url ? liveStore.toProxyUrl(props.platform, url) : '';
}

async function toggleFavorite() {
  if (!detail.value) return;

  if (isFavorited.value) {
    const ok = await liveFavoritesStore.deleteFavorite(detail.value.platform, detail.value.room_id);
    if (ok) ElMessage.success('已取消收藏');
    else ElMessage.error('取消收藏失败');
    return;
  }

  const ok = await liveFavoritesStore.addFavorite({
    platform: detail.value.platform,
    room_id: detail.value.room_id,
    title: detail.value.title,
    cover: detail.value.cover,
    user_name: detail.value.user_name,
    user_avatar: detail.value.user_avatar,
  });
  if (ok) ElMessage.success('已收藏');
  else ElMessage.error('收藏失败');
}

function saveDanmakuSettings() {
  try {
    const payload = {
      enabled: danmakuEnabled.value,
      opacity: danmakuOpacity.value,
      fontSize: danmakuFontSize.value,
      speed: danmakuSpeed.value,
    };
    localStorage.setItem('ttttv-live-danmaku-settings', JSON.stringify(payload));
    ElMessage.success('已保存');
  } catch {
    ElMessage.error('保存失败');
  }
}

function resetDanmakuSettings() {
  danmakuEnabled.value = true;
  danmakuOpacity.value = 0.85;
  danmakuFontSize.value = 22;
  danmakuSpeed.value = 120;
}

function loadDanmakuSettings() {
  try {
    const raw = localStorage.getItem('ttttv-live-danmaku-settings');
    if (!raw) return;
    const obj = JSON.parse(raw);
    danmakuEnabled.value = obj.enabled ?? true;
    danmakuOpacity.value = obj.opacity ?? 0.85;
    danmakuFontSize.value = obj.fontSize ?? 22;
    danmakuSpeed.value = obj.speed ?? 120;
  } catch {
    // ignore
  }
}

function handleClosed() {
  detail.value = null;
  qualities.value = [];
  selectedQualityId.value = '';
  playUrls.value = [];
  selectedLineIndex.value = 0;
  currentSrc.value = '';
}

loadDanmakuSettings();

function formatOnline(num: number) {
  if (!num) return '0';
  if (num >= 10000) return `${(num / 10000).toFixed(1)}万`;
  return String(num);
}
</script>

<style scoped>
.live-room-content {
  display: flex;
  gap: 16px;
}

.live-player-area {
  flex: 1;
  min-width: 0;
}

.live-player-container {
  position: relative;
  width: 100%;
  aspect-ratio: 16 / 9;
  background: #000;
  border-radius: 12px;
  overflow: hidden;
}

.danmaku-layer {
  position: absolute;
  inset: 0;
  pointer-events: none;
}

.live-controls {
  margin-top: 12px;
  display: flex;
  flex-wrap: wrap;
  gap: 12px 16px;
  align-items: center;
}

.control-row {
  display: flex;
  align-items: center;
}

.control-label {
  font-size: 13px;
  opacity: 0.85;
  margin-right: 8px;
}

.live-info {
  width: 300px;
  flex-shrink: 0;
}

.info-row {
  display: flex;
  gap: 12px;
  align-items: center;
}

.avatar {
  width: 44px;
  height: 44px;
  border-radius: 50%;
  object-fit: cover;
  background: rgba(255, 255, 255, 0.08);
}

.anchor {
  font-weight: 600;
}

.meta {
  font-size: 12px;
  opacity: 0.8;
  margin-top: 2px;
}

.info-block {
  margin-top: 14px;
}

.block-title {
  font-size: 13px;
  font-weight: 600;
  margin-bottom: 6px;
}

.block-text {
  font-size: 12px;
  opacity: 0.85;
  line-height: 1.5;
  white-space: pre-wrap;
}

.danmaku-settings {
  padding: 12px;
}

.setting-line {
  margin-bottom: 18px;
}
</style>
