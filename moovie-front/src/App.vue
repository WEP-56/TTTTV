<template>
  <div id="app" class="app-shell">
    <div class="titlebar" data-tauri-drag-region>
      <div class="titlebar-left" data-tauri-drag-region>
        <div class="app-icon">🎬</div>
        <span class="app-title">TTTTV</span>
      </div>
      <div class="titlebar-center" data-tauri-drag-region></div>
      <div class="titlebar-right">
        <ThemeToggle />
        <button class="titlebar-button" @click="minimizeWindow" :title="'最小化'">
          <svg viewBox="0 0 10 10" class="titlebar-icon">
            <rect x="0" y="4" width="10" height="2" fill="currentColor" />
          </svg>
        </button>
        <button class="titlebar-button" @click="toggleMaximize" :title="isMaximized ? '还原' : '最大化'">
          <svg v-if="!isMaximized" viewBox="0 0 10 10" class="titlebar-icon">
            <rect x="0.5" y="0.5" width="9" height="9" fill="none" stroke="currentColor" stroke-width="1.2" />
          </svg>
          <svg v-else viewBox="0 0 10 10" class="titlebar-icon">
            <rect x="2.5" y="0.5" width="7" height="7" fill="none" stroke="currentColor" stroke-width="1.2" />
            <rect x="0.5" y="2.5" width="7" height="7" fill="none" stroke="currentColor" stroke-width="1.2" />
          </svg>
        </button>
        <button class="titlebar-button close-button" @click="closeWindow" :title="'关闭'">
          <svg viewBox="0 0 10 10" class="titlebar-icon">
            <line x1="1" y1="1" x2="9" y2="9" stroke="currentColor" stroke-width="1.2" />
            <line x1="9" y1="1" x2="1" y2="9" stroke="currentColor" stroke-width="1.2" />
          </svg>
        </button>
      </div>
    </div>

    <div class="app-content">
      <nav class="sidebar">
        <div class="nav-section">
          <button 
            v-for="item in navItems" 
            :key="item.id"
            class="nav-item"
            :class="{ active: activeNav === item.id }"
            @click="activeNav = item.id"
          >
            <span class="nav-icon">{{ item.icon }}</span>
            <span class="nav-tooltip">{{ item.label }}</span>
          </button>
        </div>

        <div class="nav-footer">
          <button class="nav-item" @click="showSettings = true">
            <span class="nav-icon">⚙️</span>
            <span class="nav-tooltip">设置</span>
          </button>
        </div>
      </nav>

      <main class="main-area">
        <div v-if="showVideoDetail && currentVideo" class="page">
          <VideoDetail 
            :vod-item="currentVideo"
            :on-back="closeVideoDetail"
          />
        </div>

        <div v-else-if="activeNav === 'home'" class="page">
          <div class="page-header">
            <div class="page-header-content">
              <h1 class="page-title">发现</h1>
              <p class="page-subtitle">探索精彩内容</p>
            </div>
            <el-button 
              type="primary" 
              size="small" 
              @click="recStore.fetchRecommendations"
              :loading="recStore.loading"
            >
              <el-icon><Refresh /></el-icon>
              刷新
            </el-button>
          </div>

          <div v-if="recStore.loading" class="loading-area">
            <div class="loading-spinner"></div>
            <span class="loading-text">正在加载...</span>
          </div>

          <div v-else-if="recStore.error" class="error-area">
            <div class="error-icon">
              <el-icon :size="56"><WarningFilled /></el-icon>
            </div>
            <span class="error-text">{{ recStore.error }}</span>
            <el-button type="primary" @click="recStore.fetchRecommendations" class="retry-button">重试</el-button>
          </div>

          <div v-else>
            <div v-if="recStore.hasMovieItems" class="recommendations-section">
              <div class="section-header">
                <h3 class="section-title">热门电影</h3>
              </div>
              <div class="video-grid">
                <div 
                  v-for="item in recStore.movieItems" 
                  :key="item.id"
                  class="video-item"
                  @click="handleDoubanItemClick(item)"
                >
                  <div class="item-poster">
                    <img 
                      :src="item.cover || placeholderImage" 
                      :alt="item.title"
                      loading="lazy"
                    />
                    <div class="item-overlay">
                      <div class="play-indicator">
                        <el-icon :size="32"><Search /></el-icon>
                      </div>
                    </div>
                    <div v-if="item.rate" class="rating-badge">{{ item.rate }}</div>
                    <div v-if="item.year" class="item-badge">{{ item.year }}</div>
                  </div>
                  <div class="item-info">
                    <h3 class="item-title">{{ item.title }}</h3>
                  </div>
                </div>
              </div>
            </div>

            <div v-if="recStore.hasTvItems" class="recommendations-section" style="margin-top: 32px;">
              <div class="section-header">
                <h3 class="section-title">热门剧集</h3>
              </div>
              <div class="video-grid">
                <div 
                  v-for="item in recStore.tvItems" 
                  :key="item.id"
                  class="video-item"
                  @click="handleDoubanItemClick(item)"
                >
                  <div class="item-poster">
                    <img 
                      :src="item.cover || placeholderImage" 
                      :alt="item.title"
                      loading="lazy"
                    />
                    <div class="item-overlay">
                      <div class="play-indicator">
                        <el-icon :size="32"><Search /></el-icon>
                      </div>
                    </div>
                    <div v-if="item.rate" class="rating-badge">{{ item.rate }}</div>
                    <div v-if="item.year" class="item-badge">{{ item.year }}</div>
                  </div>
                  <div class="item-info">
                    <h3 class="item-title">{{ item.title }}</h3>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div v-else-if="activeNav === 'search'" class="page">
          <div class="page-header">
            <div class="page-header-content">
              <h1 class="page-title">搜索</h1>
              <p class="page-subtitle">发现精彩内容</p>
            </div>
          </div>

          <div class="search-section">
            <div class="search-box">
              <el-input
                v-model="searchQuery"
                placeholder="搜索电影、剧集、动漫..."
                size="large"
                :prefix-icon="Search"
                @keyup.enter="handleSearch"
                clearable
                class="search-input"
              />
              <el-button type="primary" size="large" @click="handleSearch" :loading="searchStore.loading" class="search-button">
                <el-icon><Search /></el-icon>
              </el-button>
            </div>
          </div>

          <div v-if="!searchStore.loading && searchStore.results.length === 0 && searchHistoryStore.history.length > 0" class="search-history-area">
            <div class="search-history-header">
              <span class="search-history-title">搜索历史</span>
              <el-button type="text" size="small" @click="searchHistoryStore.clearHistory()">
                清空
              </el-button>
            </div>
            <div class="search-history-tags">
              <el-tag
                v-for="item in searchHistoryStore.history"
                :key="item"
                class="search-history-tag"
                @click="useHistorySearch(item)"
                closable
                @close="searchHistoryStore.removeSearch(item)"
              >
                {{ item }}
              </el-tag>
            </div>
          </div>

          <div v-if="searchStore.loading" class="loading-area">
            <div class="loading-spinner"></div>
            <span class="loading-text">正在搜索中...</span>
          </div>

          <div v-else-if="searchStore.error" class="error-area">
            <div class="error-icon">
              <el-icon :size="56"><WarningFilled /></el-icon>
            </div>
            <span class="error-text">{{ searchStore.error }}</span>
            <el-button type="primary" @click="handleRetry" class="retry-button">重试</el-button>
          </div>

          <div v-else-if="searchStore.results.length > 0" class="results-area">
            <div class="results-header">
              <span class="results-count">{{ searchStore.results.length }} 个结果</span>
              <span v-if="searchStore.filteredCount > 0" class="filtered-badge">
                已过滤 {{ searchStore.filteredCount }} 条版权内容
              </span>
            </div>

            <div class="video-grid">
              <div 
                v-for="item in searchStore.results" 
                :key="`${item.source_key}-${item.vod_id}`"
                class="video-item"
                @click="handleSelectItem(item)"
              >
                <div class="item-poster">
                  <img 
                    :src="item.vod_pic || placeholderImage" 
                    :alt="item.vod_name"
                    loading="lazy"
                  />
                  <div class="item-overlay">
                    <div class="play-indicator">
                      <el-icon :size="32"><VideoPlay /></el-icon>
                    </div>
                  </div>
                  <div v-if="item.vod_remarks" class="item-badge">{{ item.vod_remarks }}</div>
                </div>
                <div class="item-info">
                  <h3 class="item-title">{{ item.vod_name }}</h3>
                  <span class="item-source">{{ item.source_key }}</span>
                </div>
              </div>
            </div>
          </div>

          <div v-else class="empty-area">
            <div class="empty-icon">
              <el-icon :size="64"><VideoCamera /></el-icon>
            </div>
            <span class="empty-text">输入关键词开始搜索</span>
          </div>
        </div>

        <div v-else-if="activeNav === 'history'" class="page">
          <div class="page-header">
            <h1 class="page-title">历史记录</h1>
            <p class="page-subtitle">继续观看</p>
          </div>
          <WatchHistory @select="handleHistorySelect" />
        </div>

        <div v-else-if="activeNav === 'favorites'" class="page">
          <div class="page-header">
            <h1 class="page-title">收藏夹</h1>
            <p class="page-subtitle">我的收藏</p>
          </div>
          <Favorites @select="handleFavoriteSelect" />
        </div>

        <div v-else-if="activeNav === 'player'" class="page">
          <DirectPlayer />
        </div>

        <div v-else-if="activeNav === 'about'" class="page">
          <About />
        </div>
      </main>
    </div>

    <el-dialog
      v-model="showSettings"
      title="设置"
      width="720px"
      class="settings-dialog"
      :close-on-click-modal="false"
    >
      <div class="settings-content">
        <div class="settings-section">
          <h3 class="section-title">外观</h3>
        </div>
        <div class="setting-item">
          <span class="setting-label">主题模式</span>
          <el-radio-group v-model="themeStore.mode" @change="themeStore.setMode(themeStore.mode)">
            <el-radio-button value="light">浅色</el-radio-button>
            <el-radio-button value="dark">深色</el-radio-button>
            <el-radio-button value="system">跟随系统</el-radio-button>
          </el-radio-group>
        </div>

        <div class="settings-section" style="margin-top: 24px;">
          <h3 class="section-title">资源站管理</h3>
          <div class="section-actions">
            <el-button @click="settingsStore.loadSites" :loading="settingsStore.loading">
              <el-icon><Refresh /></el-icon>
              刷新
            </el-button>
            <el-button type="primary" @click="showAddCustomSource = true">
              <el-icon><Plus /></el-icon>
              添加自定义源
            </el-button>
          </div>
        </div>
        
        <div v-for="group in settingsStore.groups" :key="group.name" class="source-group">
          <div class="group-header">
            <span class="group-name">{{ group.name }}</span>
            <span class="group-count">{{ group.sites.length }} 个</span>
          </div>
          <el-table 
            :data="group.sites" 
            style="width: 100%" 
            max-height="300"
            class="sites-table"
          >
            <el-table-column prop="name" label="资源站" width="220" />
            <el-table-column prop="key" label="标识" width="140" />
            <el-table-column label="类型" width="80" align="center">
              <template #default="{ row }">
                <el-tag v-if="row.r18" type="danger" size="small">R18</el-tag>
                <el-tag v-else type="success" size="small">普通</el-tag>
              </template>
            </el-table-column>
            <el-table-column label="状态" width="100" align="center">
              <template #default="{ row }">
                <el-switch 
                  v-model="row.enabled" 
                  @change="handleToggleSite(row)"
                />
              </template>
            </el-table-column>
            <el-table-column prop="comment" label="备注" show-overflow-tooltip />
          </el-table>
        </div>

        <div class="settings-section" style="margin-top: 24px;">
          <h3 class="section-title">数据管理</h3>
        </div>
        <div class="setting-item">
          <span class="setting-label">清除缓存</span>
          <el-button type="danger" @click="clearCache">清除</el-button>
        </div>
        <div class="setting-item">
          <span class="setting-label">删除使用记录</span>
          <el-button type="danger" @click="clearAllData">全部删除</el-button>
        </div>
      </div>
      <template #footer>
        <el-button @click="showSettings = false">关闭</el-button>
      </template>
    </el-dialog>

    <el-dialog
      v-model="showAddCustomSource"
      title="添加自定义源"
      width="500px"
      class="custom-source-dialog"
      :close-on-click-modal="false"
    >
      <div class="custom-source-form">
        <el-form label-width="80px">
          <el-form-item label="名称">
            <el-input v-model="newCustomSource.name" placeholder="资源站名称" />
          </el-form-item>
          <el-form-item label="API地址">
            <el-input v-model="newCustomSource.api" placeholder="https://example.com/api.php/provide/vod" />
          </el-form-item>
          <el-form-item label="详情页">
            <el-input v-model="newCustomSource.detail" placeholder="https://example.com" />
          </el-form-item>
        </el-form>
      </div>
      <template #footer>
        <el-button @click="showAddCustomSource = false">取消</el-button>
        <el-button type="primary" @click="saveCustomSource">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { ElMessage, ElMessageBox } from 'element-plus';
import { Search, WarningFilled, VideoPlay, Refresh, VideoCamera, Clock, Star, ArrowLeft, Plus, Delete } from '@element-plus/icons-vue';
import { useSearchStore } from './stores/search';
import { useSettingsStore } from './stores/settings';
import { useThemeStore } from './stores/theme';
import { useHistoryStore } from './stores/history';
import { useFavoritesStore } from './stores/favorites';
import { useRecommendationStore } from './stores/recommendation';
import ThemeToggle from './components/ThemeToggle.vue';
import WatchHistory from './components/WatchHistory.vue';
import Favorites from './components/Favorites.vue';
import HotCarousel from './components/HotCarousel.vue';
import SmartRecommendations from './components/SmartRecommendations.vue';
import VideoDetail from './components/VideoDetail.vue';
import DirectPlayer from './components/DirectPlayer.vue';
import About from './components/About.vue';
import type { VodItem } from './types';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useSearchHistoryStore } from './stores/searchHistory';

const searchStore = useSearchStore();
const settingsStore = useSettingsStore();
const themeStore = useThemeStore();
const historyStore = useHistoryStore();
const favoritesStore = useFavoritesStore();
const recStore = useRecommendationStore();
const searchHistoryStore = useSearchHistoryStore();

const searchQuery = ref('');
const showSettings = ref(false);
const showAddCustomSource = ref(false);
const activeNav = ref('home');
const isMaximized = ref(false);
const isTauri = window.__TAURI__ !== undefined;
const currentVideo = ref<VodItem | null>(null);
const showVideoDetail = ref(false);

const newCustomSource = ref({
  name: '',
  api: '',
  detail: '',
});

const placeholderImage = 'data:image/svg+xml,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 width=%22400%22 height=%22600%22%3E%3Crect fill=%22%23f3f3f3%22 width=%22400%22 height=%22600%22/%3E%3Ctext x=%22200%22 y=%22300%22 fill=%22%234cc2ff%22 font-size=%2248%22 text-anchor=%22middle%22 dominant-baseline=%22middle%22%3E🎬%3C/text%3E%3C/svg%3E';

const navItems = [
  { id: 'home', icon: '🏠', label: '发现' },
  { id: 'search', icon: '🔍', label: '搜索' },
  { id: 'player', icon: '🎬', label: 'M3U8' },
  { id: 'history', icon: '⏱️', label: '历史' },
  { id: 'favorites', icon: '⭐', label: '收藏' },
  { id: 'about', icon: 'ℹ️', label: '关于' },
];

onMounted(async () => {
  themeStore.initTheme();
  settingsStore.loadSites();
  searchHistoryStore.loadFromStorage();
  recStore.fetchRecommendations();
  if (isTauri) {
    try {
      const appWindow = getCurrentWindow();
      const maximized = await appWindow.isMaximized();
      isMaximized.value = maximized;
    } catch (e) {
      console.log('Not in Tauri environment');
    }
  }
});

async function minimizeWindow() {
  if (isTauri) {
    try {
      const appWindow = getCurrentWindow();
      await appWindow.minimize();
    } catch (e) {
      ElMessage.info('此功能仅在桌面端可用');
    }
  } else {
    ElMessage.info('此功能仅在桌面端可用');
  }
}

async function toggleMaximize() {
  if (isTauri) {
    try {
      const appWindow = getCurrentWindow();
      if (isMaximized.value) {
        await appWindow.unmaximize();
      } else {
        await appWindow.maximize();
      }
      isMaximized.value = !isMaximized.value;
    } catch (e) {
      ElMessage.info('此功能仅在桌面端可用');
    }
  } else {
    ElMessage.info('此功能仅在桌面端可用');
  }
}

async function closeWindow() {
  if (isTauri) {
    try {
      const appWindow = getCurrentWindow();
      await appWindow.close();
    } catch (e) {
      ElMessage.info('此功能仅在桌面端可用');
    }
  } else {
    ElMessage.info('此功能仅在桌面端可用');
  }
}

async function handleSearch() {
  if (!searchQuery.value.trim()) {
    ElMessage.warning('请输入搜索关键词');
    return;
  }
  searchHistoryStore.addSearch(searchQuery.value);
  await searchStore.search(searchQuery.value);
}

function useHistorySearch(query: string) {
  searchQuery.value = query;
  handleSearch();
}

function handleRetry() {
  searchStore.clearResults();
  if (searchQuery.value.trim()) {
    handleSearch();
  }
}

function handleSelectItem(item: VodItem) {
  currentVideo.value = item;
  showVideoDetail.value = true;
}

function handleHistorySelect(item: any) {
  currentVideo.value = item;
  showVideoDetail.value = true;
}

function handleFavoriteSelect(item: any) {
  currentVideo.value = item;
  showVideoDetail.value = true;
}

function handleHotPlay(item: any) {
  currentVideo.value = item;
  showVideoDetail.value = true;
}

function handleRecSelect(item: any) {
  currentVideo.value = item;
  showVideoDetail.value = true;
}

function handleDoubanItemClick(item: any) {
  activeNav.value = 'search';
  searchQuery.value = item.title;
  searchStore.search(item.title);
}

function closeVideoDetail() {
  showVideoDetail.value = false;
  currentVideo.value = null;
}

async function handleToggleSite(site: any) {
  await settingsStore.toggleSite(site.key, site.enabled);
  ElMessage.success(site.enabled ? '已启用该资源站' : '已禁用该资源站');
}

function saveCustomSource() {
  if (!newCustomSource.value.name || !newCustomSource.value.api) {
    ElMessage.warning('请填写完整信息');
    return;
  }
  
  settingsStore.addCustomSource(newCustomSource.value);
  newCustomSource.value = { name: '', api: '', detail: '' };
  showAddCustomSource.value = false;
  ElMessage.success('自定义源添加成功');
}

async function clearCache() {
  try {
    await ElMessageBox.confirm(
      '确定要清除缓存吗？这将清除推荐内容缓存。',
      '清除缓存',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    );
    recStore.fetchRecommendations();
    ElMessage.success('缓存已清除');
  } catch (error) {
    if (error !== 'cancel') {
      console.error('清除缓存失败:', error);
      ElMessage.error('清除缓存失败');
    }
  }
}

async function clearAllData() {
  try {
    await ElMessageBox.confirm(
      '确定要删除所有使用记录吗？这将包括：\n- 观影历史\n- 收藏记录\n- 搜索历史\n\n此操作不可恢复！',
      '删除使用记录',
      {
        confirmButtonText: '确定删除',
        cancelButtonText: '取消',
        type: 'warning'
      }
    );
    
    searchHistoryStore.clearHistory();
    
    try {
      await historyStore.clearHistory();
    } catch (e) {
      console.log('历史记录清除可能失败');
    }
    
    try {
      await favoritesStore.clearFavorites();
    } catch (e) {
      console.log('收藏记录清除可能失败');
    }
    
    ElMessage.success('使用记录已删除');
  } catch (error) {
    if (error !== 'cancel') {
      console.error('删除使用记录失败:', error);
      ElMessage.error('删除使用记录失败');
    }
  }
}
</script>

<style>
:root {
  --el-border-radius-base: 10px;
  --el-color-primary: #4cc2ff;
  --el-color-primary-light-3: #5fb3ff;
  --el-color-primary-light-5: #7fc4ff;
  --el-color-primary-light-7: #b3e0ff;
  --el-color-primary-light-9: #e0f2ff;
  --el-color-primary-dark-2: #3daee9;
}

.light-theme {
  --el-bg-color: rgba(255, 255, 255, 0.7);
  --el-bg-color-overlay: rgba(255, 255, 255, 0.95);
  --el-bg-color-page: rgba(255, 255, 255, 0.7);
  --el-fill-color-blank: rgba(255, 255, 255, 0.8);
  --el-border-color: rgba(0, 0, 0, 0.08);
  --el-border-color-lighter: rgba(0, 0, 0, 0.12);
  --el-border-color-light: rgba(0, 0, 0, 0.1);
  --el-border-color-extra-light: rgba(0, 0, 0, 0.06);
  --el-text-color-primary: #1a1a1a;
  --el-text-color-regular: #333333;
  --el-text-color-secondary: #666666;
  --el-text-color-placeholder: #999999;
  --el-box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
  --el-box-shadow-light: 0 2px 12px 0 rgba(0, 0, 0, 0.08);
  --el-box-shadow-lighter: 0 2px 8px 0 rgba(0, 0, 0, 0.06);
  --el-box-shadow-dark: 0 2px 16px 0 rgba(0, 0, 0, 0.12);
  --el-mask-color: rgba(0, 0, 0, 0.5);
  --el-mask-color-extra-light: rgba(0, 0, 0, 0.3);
}

.dark-theme {
  --el-bg-color: rgba(37, 37, 37, 0.85);
  --el-bg-color-overlay: rgba(37, 37, 37, 0.95);
  --el-bg-color-page: rgba(37, 37, 37, 0.85);
  --el-fill-color-blank: rgba(255, 255, 255, 0.05);
  --el-border-color: rgba(255, 255, 255, 0.08);
  --el-border-color-lighter: rgba(255, 255, 255, 0.12);
  --el-border-color-light: rgba(255, 255, 255, 0.1);
  --el-border-color-extra-light: rgba(255, 255, 255, 0.06);
  --el-text-color-primary: #e6e6e6;
  --el-text-color-regular: #cfcfcf;
  --el-text-color-secondary: #a8a8a8;
  --el-text-color-placeholder: #757575;
  --el-box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.3);
  --el-box-shadow-light: 0 2px 12px 0 rgba(0, 0, 0, 0.25);
  --el-box-shadow-lighter: 0 2px 8px 0 rgba(0, 0, 0, 0.2);
  --el-box-shadow-dark: 0 2px 16px 0 rgba(0, 0, 0, 0.35);
  --el-mask-color: rgba(0, 0, 0, 0.7);
  --el-mask-color-extra-light: rgba(0, 0, 0, 0.5);
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body, #app {
  width: 100%;
  height: 100%;
  overflow: hidden;
  background: transparent;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}
</style>

<style scoped>
.app-shell {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
  transition: background 0.3s ease;
  background: var(--el-bg-color-page);
}

.titlebar {
  display: flex;
  align-items: center;
  height: 32px;
  padding: 0 8px;
  flex-shrink: 0;
  user-select: none;
  transition: background 0.3s ease;
  background: var(--el-bg-color);
}

.titlebar-left {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 0 8px;
}

.app-icon {
  font-size: 14px;
}

.app-title {
  font-size: 12px;
  font-weight: 400;
  transition: color 0.3s ease;
  color: var(--el-text-color-secondary);
}

.titlebar-center {
  flex: 1;
}

.titlebar-right {
  display: flex;
  align-items: center;
  height: 100%;
  gap: 4px;
  margin-left: auto;
}

.titlebar-button {
  width: 46px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  cursor: pointer;
  transition: background 0.08s, color 0.3s;
  color: var(--el-text-color-primary);
}

.titlebar-button:hover {
  background: var(--el-fill-color-lighter);
}

.titlebar-button.close-button:hover {
  background: #c42b1c !important;
  color: white !important;
}

.titlebar-icon {
  width: 10px;
  height: 10px;
}

.app-content {
  flex: 1;
  display: flex;
  overflow: hidden;
  background: var(--el-bg-color-page);
}

.sidebar {
  width: 64px;
  border-right: 1px solid var(--el-border-color-lighter);
  display: flex;
  flex-direction: column;
  padding: 12px 0;
  flex-shrink: 0;
  transition: background 0.3s ease, border-color 0.3s ease;
  background: var(--el-bg-color);
}

.nav-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 0 8px;
}

.nav-footer {
  padding: 0 8px;
  border-top: 1px solid var(--el-border-color-lighter);
  padding-top: 12px;
  margin-top: auto;
  transition: border-color 0.3s ease;
}

.nav-item {
  position: relative;
  width: 100%;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.15s ease;
  color: var(--el-text-color-secondary);
}

.nav-item:hover {
  background: var(--el-fill-color-lighter);
  color: var(--el-text-color-primary);
}

.nav-item.active {
  background: rgba(76, 194, 255, 0.15);
  color: #4cc2ff;
}

.nav-icon {
  font-size: 20px;
}

.nav-tooltip {
  position: absolute;
  left: 72px;
  padding: 6px 12px;
  background: var(--el-bg-color-overlay);
  color: var(--el-text-color-primary);
  font-size: 12px;
  border-radius: 6px;
  border: 1px solid var(--el-border-color-lighter);
  box-shadow: var(--el-box-shadow);
  white-space: nowrap;
  opacity: 0;
  pointer-events: none;
  transition: opacity 0.1s ease;
  z-index: 100;
}

.nav-item:hover .nav-tooltip {
  opacity: 1;
}

.main-area {
  flex: 1;
  overflow-y: auto;
  padding: 28px 32px;
}

.main-area::-webkit-scrollbar {
  width: 8px;
}

.main-area::-webkit-scrollbar-track {
  background: transparent;
}

.main-area::-webkit-scrollbar-thumb {
  border-radius: 4px;
  background: var(--el-border-color-lighter);
  transition: background 0.3s ease;
}

.main-area::-webkit-scrollbar-thumb:hover {
  background: var(--el-border-color-light);
}

.page {
  width: 100%;
}

.page-header {
  margin-bottom: 28px;
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
  margin-bottom: 4px;
  transition: color 0.3s ease;
  color: var(--el-text-color-primary);
}

.page-subtitle {
  font-size: 13px;
  transition: color 0.3s ease;
  color: var(--el-text-color-secondary);
}

.carousel-section {
  margin-bottom: 32px;
}

.recommendations-section {
  margin-bottom: 20px;
}

.section-header {
  margin-bottom: 20px;
}

.section-title {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--el-text-color-primary);
  transition: color 0.3s ease;
}

.rating-badge {
  position: absolute;
  top: 8px;
  left: 8px;
  padding: 3px 8px;
  background: rgba(255, 193, 7, 0.95);
  color: #000;
  font-size: 11px;
  font-weight: 700;
  border-radius: 12px;
  backdrop-filter: blur(8px);
}

.search-section {
  margin-bottom: 28px;
}

.search-box {
  display: flex;
  align-items: center;
  gap: 12px;
  max-width: 520px;
}

.search-input {
  flex: 1;
}

.search-input :deep(.el-input__wrapper) {
  border-radius: 10px;
  padding: 7px 14px;
  box-shadow: none;
  transition: all 0.15s;
  background: var(--el-fill-color-blank);
  border: 1px solid var(--el-border-color);
}

.search-input :deep(.el-input__wrapper:hover) {
  border-color: rgba(76, 194, 255, 0.4);
  background: var(--el-bg-color-overlay);
}

.search-input :deep(.el-input__wrapper.is-focus) {
  border-color: #4cc2ff;
  box-shadow: 0 0 0 3px rgba(76, 194, 255, 0.15);
  background: var(--el-bg-color-overlay);
}

.search-input :deep(.el-input__inner) {
  font-size: 14px;
  transition: color 0.3s ease;
  color: var(--el-text-color-primary);
}

.search-input :deep(.el-input__inner::placeholder) {
  transition: color 0.3s ease;
  color: var(--el-text-color-placeholder);
}

.search-button {
  padding: 7px 16px;
  border-radius: 10px;
  background: #4cc2ff;
  border: none;
  color: #000;
  font-weight: 500;
}

.search-button:hover {
  background: #5fb3ff;
  color: #000;
}

.search-button:active {
  transform: scale(0.98);
}

.loading-area,
.error-area,
.empty-area {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px 20px;
  gap: 16px;
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 3px solid rgba(76, 194, 255, 0.2);
  border-top-color: #4cc2ff;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.loading-text,
.error-text,
.empty-text {
  font-size: 13px;
  transition: color 0.3s ease;
  color: var(--el-text-color-secondary);
}

.error-icon,
.empty-icon {
  color: #4cc2ff;
  opacity: 0.4;
}

.retry-button {
  margin-top: 4px;
  border-radius: 8px;
  background: #4cc2ff;
  border: none;
  color: #000;
}

.retry-button:hover {
  background: #5fb3ff;
  color: #000;
}

.search-history-area {
  margin-bottom: 24px;
}

.search-history-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.search-history-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--el-text-color-primary);
}

.search-history-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.search-history-tag {
  cursor: pointer;
  transition: all 0.2s ease;
}

.results-area {
  width: 100%;
}

.results-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 20px;
}

.results-count {
  font-size: 13px;
  font-weight: 500;
  transition: color 0.3s ease;
  color: var(--el-text-color-regular);
}

.filtered-badge {
  color: #f6ad55;
  font-size: 11px;
  padding: 3px 10px;
  background: rgba(246, 173, 85, 0.12);
  border-radius: 12px;
}

.video-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(170px, 1fr));
  gap: 18px;
}

.video-item {
  cursor: pointer;
  transition: transform 0.15s ease;
}

.video-item:hover {
  transform: translateY(-2px);
}

.item-poster {
  position: relative;
  width: 100%;
  padding-top: 145%;
  border-radius: 10px;
  overflow: hidden;
  transition: box-shadow 0.15s;
  background: var(--el-fill-color-blank);
  box-shadow: var(--el-box-shadow-lighter);
}

.video-item:hover .item-poster {
  box-shadow: var(--el-box-shadow);
}

.item-poster img {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.item-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(180deg, transparent 0%, rgba(0, 0, 0, 0.55) 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  transition: opacity 0.15s;
}

.video-item:hover .item-overlay {
  opacity: 1;
}

.play-indicator {
  width: 48px;
  height: 48px;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.25);
  backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  transition: transform 0.15s;
}

.video-item:hover .play-indicator {
  transform: scale(1.05);
}

.item-badge {
  position: absolute;
  top: 8px;
  right: 8px;
  padding: 3px 8px;
  background: rgba(76, 194, 255, 0.95);
  color: #000;
  font-size: 11px;
  font-weight: 600;
  border-radius: 12px;
  backdrop-filter: blur(8px);
}

.item-info {
  padding: 10px 2px 2px;
}

.item-title {
  margin: 0 0 4px;
  font-size: 13px;
  font-weight: 500;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  transition: color 0.3s ease;
  color: var(--el-text-color-primary);
}

.item-source {
  font-size: 11px;
  transition: color 0.3s ease;
  color: var(--el-text-color-secondary);
}

.settings-dialog :deep(.el-dialog) {
  backdrop-filter: blur(24px);
  border-radius: 12px;
  border: 1px solid var(--el-border-color);
  transition: background 0.3s ease, border-color 0.3s ease;
  background: var(--el-bg-color-overlay);
}

.settings-dialog :deep(.el-dialog__header) {
  border-bottom: 1px solid var(--el-border-color-lighter);
  padding: 14px 18px;
  transition: border-color 0.3s ease;
}

.settings-dialog :deep(.el-dialog__title) {
  font-size: 14px;
  font-weight: 600;
  transition: color 0.3s ease;
  color: var(--el-text-color-primary);
}

.settings-dialog :deep(.el-dialog__body) {
  padding: 18px;
}

.settings-content {
  width: 100%;
}

.settings-section {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 14px;
}

.section-title {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  transition: color 0.3s ease;
  color: var(--el-text-color-regular);
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 0;
}

.setting-label {
  font-size: 14px;
  transition: color 0.3s ease;
  color: var(--el-text-color-regular);
}

.section-actions {
  display: flex;
  align-items: center;
  gap: 12px;
}

.source-group {
  margin-bottom: 24px;
}

.group-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
  padding: 0 4px;
}

.group-name {
  font-size: 14px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.group-count {
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.custom-source-dialog :deep(.el-dialog) {
  backdrop-filter: blur(24px);
  border-radius: 12px;
  border: 1px solid var(--el-border-color);
  background: var(--el-bg-color-overlay);
}

.sites-table {
  background: transparent;
  border-radius: 8px;
  overflow: hidden;
}

.sites-table :deep(.el-table) {
  background: transparent;
}

.sites-table :deep(.el-table th.el-table__cell) {
  border-bottom: 1px solid var(--el-border-color-lighter);
  background: var(--el-fill-color-blank);
  color: var(--el-text-color-secondary);
}

.sites-table :deep(.el-table tr.el-table__row) {
  background: transparent;
}

.sites-table :deep(.el-table tr.el-table__row:hover > td) {
  transition: background 0.15s;
  background: var(--el-fill-color-blank);
}

.sites-table :deep(.el-table td.el-table__cell) {
  border-color: var(--el-border-color-extra-light);
  color: var(--el-text-color-regular);
}
</style>
