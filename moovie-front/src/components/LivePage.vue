<template>
  <div class="live-page">
    <div class="page-header">
      <div class="page-header-content">
        <h1 class="page-title">直播</h1>
        <p class="page-subtitle">四平台聚合：Bilibili / 斗鱼 / 虎牙 / 抖音</p>
      </div>
      <div class="live-header-actions">
        <el-button @click="openFavorites">
          收藏
        </el-button>
        <el-button @click="openHistory">
          历史
        </el-button>
      </div>
    </div>

    <el-tabs v-model="activePlatform" class="live-platform-tabs" @tab-change="handlePlatformChange">
      <el-tab-pane
        v-for="p in platformList"
        :key="p.id"
        :label="p.name"
        :name="p.id"
      />
    </el-tabs>

    <div class="live-toolbar">
      <el-input
        v-model="keyword"
        placeholder="搜索直播间标题 / 主播"
        clearable
        @keyup.enter="handleSearch"
      />
      <el-button type="primary" :loading="liveStore.loading" @click="handleSearch">搜索</el-button>
      <el-button :loading="liveStore.loading" @click="loadRecommend">推荐</el-button>
    </div>

    <div v-if="liveStore.error" class="error-area" style="margin-top: 12px;">
      <div class="error-icon">
        <el-icon :size="40"><WarningFilled /></el-icon>
      </div>
      <span class="error-text">{{ liveStore.error }}</span>
    </div>

    <div v-if="rooms.length === 0 && !liveStore.loading && !liveStore.error" class="empty-area">
      <div class="empty-icon">
        <el-icon :size="64"><VideoCamera /></el-icon>
      </div>
      <span class="empty-text">暂无内容，试试搜索或切换平台</span>
    </div>

    <div v-else class="live-grid" style="margin-top: 16px;">
      <div
        v-for="room in rooms"
        :key="`${room.platform}-${room.room_id}`"
        class="live-card"
        @click="openRoom(room)"
      >
        <div class="cover">
          <img :src="getCover(room)" :alt="room.title" loading="lazy" referrerpolicy="no-referrer" />
          <div class="cover-overlay">
            <span class="live-badge">LIVE</span>
            <span class="online-badge">{{ formatOnline(room.online) }}</span>
          </div>
        </div>
        <div class="info">
          <h3 class="title" :title="room.title">{{ room.title }}</h3>
          <div class="sub">
            <span class="anchor" :title="room.user_name">{{ room.user_name }}</span>
            <span class="dot">·</span>
            <span class="platform">{{ toPlatformName(room.platform) }}</span>
          </div>
        </div>
      </div>
    </div>

    <LiveRoomDialog
      v-model="showRoom"
      :platform="selectedPlatform"
      :room-id="selectedRoomId"
    />

    <el-dialog v-model="showFavorites" title="直播收藏" width="720px" :close-on-click-modal="false">
      <div v-if="liveFavoritesStore.loading" class="loading-area">
        <div class="loading-spinner"></div>
        <span class="loading-text">正在加载...</span>
      </div>
      <div v-else-if="!liveFavoritesStore.hasFavorites" class="empty-area">
        <span class="empty-text">暂无收藏</span>
      </div>
      <el-table
        v-else
        :data="liveFavoritesStore.favorites"
        style="width: 100%"
        max-height="420"
        @row-click="openFavoriteRow"
      >
        <el-table-column prop="platform" label="平台" width="90" />
        <el-table-column prop="title" label="标题" show-overflow-tooltip />
        <el-table-column prop="user_name" label="主播" width="160" />
        <el-table-column label="操作" width="90" align="center">
          <template #default="{ row }">
            <el-button type="danger" text @click.stop="removeFavorite(row)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-dialog>

    <el-dialog v-model="showHistory" title="直播历史" width="720px" :close-on-click-modal="false">
      <div v-if="liveHistoryStore.loading" class="loading-area">
        <div class="loading-spinner"></div>
        <span class="loading-text">正在加载...</span>
      </div>
      <div v-else-if="!liveHistoryStore.hasHistory" class="empty-area">
        <span class="empty-text">暂无历史</span>
      </div>
      <el-table
        v-else
        :data="liveHistoryStore.history"
        style="width: 100%"
        max-height="420"
        @row-click="openHistoryRow"
      >
        <el-table-column prop="platform" label="平台" width="90" />
        <el-table-column prop="title" label="标题" show-overflow-tooltip />
        <el-table-column prop="user_name" label="主播" width="160" />
        <el-table-column label="操作" width="90" align="center">
          <template #default="{ row }">
            <el-button type="danger" text @click.stop="removeHistory(row)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { ElMessage } from 'element-plus';
import { WarningFilled, VideoCamera } from '@element-plus/icons-vue';
import { useLiveStore } from '../stores/live';
import { useLiveFavoritesStore } from '../stores/liveFavorites';
import { useLiveHistoryStore } from '../stores/liveHistory';
import LiveRoomDialog from './live/LiveRoomDialog.vue';
import type { LiveFavoriteItem, LiveHistoryItem, LiveRoomItem } from '../types';

const liveStore = useLiveStore();
const liveFavoritesStore = useLiveFavoritesStore();
const liveHistoryStore = useLiveHistoryStore();

const placeholderImage =
  'data:image/svg+xml,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 width=%22400%22 height=%22600%22%3E%3Crect fill=%22%23111111%22 width=%22400%22 height=%22600%22/%3E%3Ctext x=%22200%22 y=%22300%22 fill=%22%234cc2ff%22 font-size=%2240%22 text-anchor=%22middle%22 dominant-baseline=%22middle%22%3ELIVE%3C/text%3E%3C/svg%3E';

const keyword = ref('');
const activePlatform = ref('bilibili');
const rooms = ref<LiveRoomItem[]>([]);

const showRoom = ref(false);
const selectedPlatform = ref('bilibili');
const selectedRoomId = ref('');

const showFavorites = ref(false);
const showHistory = ref(false);

const platformList = computed(() => {
  if (liveStore.platforms.length > 0) return liveStore.platforms;
  return [
    { id: 'bilibili', name: 'Bilibili' },
    { id: 'douyu', name: '斗鱼' },
    { id: 'huya', name: '虎牙' },
    { id: 'douyin', name: '抖音' },
  ];
});

onMounted(async () => {
  liveStore.loadPlatforms();
  liveFavoritesStore.fetchFavorites();
  liveHistoryStore.fetchHistory();
  await loadRecommend();
});

async function handlePlatformChange() {
  keyword.value = '';
  await loadRecommend();
}

async function loadRecommend() {
  rooms.value = await liveStore.recommend(activePlatform.value, 1);
}

async function handleSearch() {
  const kw = keyword.value.trim();
  if (!kw) {
    await loadRecommend();
    return;
  }
  rooms.value = await liveStore.search(activePlatform.value, kw, 1);
}

function openRoom(room: LiveRoomItem) {
  selectedPlatform.value = room.platform;
  selectedRoomId.value = room.room_id;
  showRoom.value = true;
}

function openFavorites() {
  showFavorites.value = true;
  liveFavoritesStore.fetchFavorites();
}

function openHistory() {
  showHistory.value = true;
  liveHistoryStore.fetchHistory();
}

function openFavoriteRow(row: LiveFavoriteItem) {
  showFavorites.value = false;
  selectedPlatform.value = row.platform;
  selectedRoomId.value = row.room_id;
  showRoom.value = true;
}

function openHistoryRow(row: LiveHistoryItem) {
  showHistory.value = false;
  selectedPlatform.value = row.platform;
  selectedRoomId.value = row.room_id;
  showRoom.value = true;
}

async function removeFavorite(row: LiveFavoriteItem) {
  const ok = await liveFavoritesStore.deleteFavorite(row.platform, row.room_id);
  if (ok) ElMessage.success('已删除收藏');
  else ElMessage.error('删除失败');
}

async function removeHistory(row: LiveHistoryItem) {
  const ok = await liveHistoryStore.deleteHistory(row.platform, row.room_id);
  if (ok) ElMessage.success('已删除历史');
  else ElMessage.error('删除失败');
}

function formatOnline(num: number) {
  if (!num) return '0';
  if (num >= 10000) return `${(num / 10000).toFixed(1)}万`;
  return String(num);
}

function getCover(room: LiveRoomItem) {
  if (!room.cover) return placeholderImage;
  return liveStore.toProxyUrl(room.platform, room.cover);
}

function toPlatformName(platform: string) {
  if (platform === 'bilibili') return 'Bilibili';
  if (platform === 'douyu') return '斗鱼';
  if (platform === 'huya') return '虎牙';
  if (platform === 'douyin') return '抖音';
  return platform;
}
</script>

<style scoped>
.page-header {
  margin-bottom: 18px;
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
}

.page-header-content {
  flex: 1;
}

.page-title {
  font-size: 28px;
  font-weight: 600;
  margin: 0 0 4px;
  color: var(--el-text-color-primary);
}

.page-subtitle {
  font-size: 13px;
  color: var(--el-text-color-secondary);
}

.empty-area,
.loading-area,
.error-area {
  margin-top: 24px;
  padding: 24px;
  border-radius: 12px;
  background: var(--el-fill-color-blank);
  border: 1px solid var(--el-border-color);
}

.empty-area {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  gap: 10px;
  color: var(--el-text-color-secondary);
}

.loading-area {
  display: flex;
  align-items: center;
  gap: 12px;
  color: var(--el-text-color-secondary);
}

.loading-spinner {
  width: 20px;
  height: 20px;
  border-radius: 999px;
  border: 2px solid rgba(76, 194, 255, 0.25);
  border-top-color: rgba(76, 194, 255, 0.95);
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.error-area {
  display: flex;
  align-items: center;
  gap: 12px;
  color: var(--el-text-color-secondary);
}

.error-text {
  color: var(--el-text-color-primary);
}

.live-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
  gap: 16px;
}

.live-card {
  cursor: pointer;
  border-radius: 12px;
  overflow: hidden;
  background: var(--el-fill-color-blank);
  border: 1px solid var(--el-border-color);
  transition: transform 0.15s ease, box-shadow 0.15s ease, border-color 0.15s ease;
}

.live-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--el-box-shadow);
  border-color: rgba(76, 194, 255, 0.35);
}

.cover {
  position: relative;
  width: 100%;
  padding-top: 56.25%;
  background: rgba(0,0,0,0.85);
}

.cover img {
  position: absolute;
  inset: 0;
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.cover-overlay {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  padding: 10px;
  background: linear-gradient(180deg, rgba(0,0,0,0.35) 0%, rgba(0,0,0,0) 45%, rgba(0,0,0,0.35) 100%);
  pointer-events: none;
}

.online-badge {
  padding: 4px 10px;
  border-radius: 999px;
  background: rgba(76, 194, 255, 0.95);
  color: #000;
  font-size: 12px;
  font-weight: 700;
}

.info {
  padding: 10px 12px 12px;
}

.title {
  margin: 0;
  font-size: 13px;
  font-weight: 600;
  color: var(--el-text-color-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.sub {
  margin-top: 6px;
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.anchor {
  max-width: 160px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.dot {
  opacity: 0.5;
}

.platform {
  opacity: 0.9;
}

.live-toolbar {
  display: flex;
  gap: 10px;
  align-items: center;
}

.live-header-actions {
  display: flex;
  gap: 10px;
}

.live-badge {
  padding: 6px 10px;
  border-radius: 12px;
  background: rgba(255, 0, 0, 0.85);
  color: #fff;
  font-weight: 700;
  letter-spacing: 0.5px;
  box-shadow: 0 6px 16px rgba(0,0,0,0.25);
}
</style>
