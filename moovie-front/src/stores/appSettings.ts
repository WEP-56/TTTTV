import { defineStore } from 'pinia';
import { ref, watch } from 'vue';

interface AppSettings {
  darkMode: boolean;
  autoPlay: boolean;
  savePlayProgress: boolean;
  showR18: boolean;
}

const SETTINGS_KEY = 'moovie-settings';

function loadSettings(): AppSettings {
  try {
    const saved = localStorage.getItem(SETTINGS_KEY);
    if (saved) {
      return {
        ...getDefaultSettings(),
        ...JSON.parse(saved)
      };
    }
  } catch (e) {
    console.error('加载设置失败:', e);
  }
  return getDefaultSettings();
}

function getDefaultSettings(): AppSettings {
  return {
    darkMode: false,
    autoPlay: true,
    savePlayProgress: true,
    showR18: false
  };
}

export const useAppSettingsStore = defineStore('appSettings', () => {
  const settings = ref<AppSettings>(loadSettings());

  watch(settings, (newSettings) => {
    try {
      localStorage.setItem(SETTINGS_KEY, JSON.stringify(newSettings));
    } catch (e) {
      console.error('保存设置失败:', e);
    }
  }, { deep: true });

  function toggleDarkMode() {
    settings.value.darkMode = !settings.value.darkMode;
  }

  function setDarkMode(value: boolean) {
    settings.value.darkMode = value;
  }

  function toggleAutoPlay() {
    settings.value.autoPlay = !settings.value.autoPlay;
  }

  function setAutoPlay(value: boolean) {
    settings.value.autoPlay = value;
  }

  function toggleSavePlayProgress() {
    settings.value.savePlayProgress = !settings.value.savePlayProgress;
  }

  function setSavePlayProgress(value: boolean) {
    settings.value.savePlayProgress = value;
  }

  function toggleShowR18() {
    settings.value.showR18 = !settings.value.showR18;
  }

  function setShowR18(value: boolean) {
    settings.value.showR18 = value;
  }

  function resetToDefault() {
    settings.value = getDefaultSettings();
  }

  return {
    settings,
    toggleDarkMode,
    setDarkMode,
    toggleAutoPlay,
    setAutoPlay,
    toggleSavePlayProgress,
    setSavePlayProgress,
    toggleShowR18,
    setShowR18,
    resetToDefault
  };
});
