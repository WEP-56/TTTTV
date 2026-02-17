import { defineStore } from 'pinia';
import { ref, watch } from 'vue';

export type ThemeMode = 'light' | 'dark' | 'system';

export const useThemeStore = defineStore('theme', () => {
  const mode = ref<ThemeMode>('light');
  const currentTheme = ref<'light' | 'dark'>('light');

  function initTheme() {
    const saved = localStorage.getItem('moovie-theme-mode');
    if (saved && ['light', 'dark', 'system'].includes(saved)) {
      mode.value = saved as ThemeMode;
    }
    applyTheme();
  }

  function setMode(newMode: ThemeMode) {
    mode.value = newMode;
    localStorage.setItem('moovie-theme-mode', newMode);
    applyTheme();
  }

  function toggleTheme() {
    if (mode.value === 'system') {
      mode.value = currentTheme.value === 'dark' ? 'light' : 'dark';
    } else {
      mode.value = mode.value === 'light' ? 'dark' : 'light';
    }
    localStorage.setItem('moovie-theme-mode', mode.value);
    applyTheme();
  }

  function applyTheme() {
    let theme: 'light' | 'dark';
    if (mode.value === 'system') {
      theme = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
    } else {
      theme = mode.value;
    }
    currentTheme.value = theme;
    
    if (theme === 'dark') {
      document.documentElement.classList.add('dark-theme');
      document.documentElement.classList.remove('light-theme');
    } else {
      document.documentElement.classList.add('light-theme');
      document.documentElement.classList.remove('dark-theme');
    }
  }

  watch(mode, applyTheme);

  return {
    mode,
    currentTheme,
    initTheme,
    setMode,
    toggleTheme
  };
});
