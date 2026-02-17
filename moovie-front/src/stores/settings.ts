import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { apiClient } from '../api/client';
import type { SiteWithStatus } from '../types';

export const useSettingsStore = defineStore('settings', () => {
  const sites = ref<SiteWithStatus[]>([]);
  const loading = ref(false);
  const addingSource = ref(false);
  const deletingSource = ref(false);

  const groups = computed(() => {
    const groupMap = new Map<string, SiteWithStatus[]>();
    
    sites.value.forEach(site => {
      const groupName = site.group || (site.r18 ? 'R18' : '影视');
      if (!groupMap.has(groupName)) {
        groupMap.set(groupName, []);
      }
      groupMap.get(groupName)!.push(site);
    });
    
    return Array.from(groupMap.entries()).map(([name, sites]) => ({ name, sites }));
  });

  async function loadSites() {
    loading.value = true;
    try {
      const res = await apiClient.getSites();
      if (res.success && res.data) {
        sites.value = res.data;
      }
    } catch (err) {
      console.error('加载资源站失败', err);
    } finally {
      loading.value = false;
    }
  }

  async function toggleSite(key: string, enabled: boolean) {
    try {
      await apiClient.toggleSite(key, enabled);
      const site = sites.value.find(s => s.key === key);
      if (site) {
        site.enabled = enabled;
      }
    } catch (err) {
      console.error('切换资源站状态失败', err);
    }
  }

  async function addCustomSource(source: {
    key: string;
    name: string;
    api: string;
    detail: string;
    group?: string;
    r18?: boolean;
  }) {
    addingSource.value = true;
    try {
      await apiClient.addSource(source);
      await loadSites();
      return true;
    } catch (err) {
      console.error('添加自定义源失败', err);
      return false;
    } finally {
      addingSource.value = false;
    }
  }

  async function deleteCustomSource(key: string) {
    deletingSource.value = true;
    try {
      await apiClient.deleteSource(key);
      await loadSites();
      return true;
    } catch (err) {
      console.error('删除自定义源失败', err);
      return false;
    } finally {
      deletingSource.value = false;
    }
  }

  return {
    sites,
    loading,
    addingSource,
    deletingSource,
    groups,
    loadSites,
    toggleSite,
    addCustomSource,
    deleteCustomSource,
  };
});
