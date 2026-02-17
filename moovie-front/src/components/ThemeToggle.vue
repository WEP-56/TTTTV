<template>
  <button 
    class="theme-toggle-button"
    @click="handleToggle"
    ref="buttonRef"
  >
    <span class="icon">{{ currentIcon }}</span>
    <div 
      v-for="(ripple, index) in ripples" 
      :key="index"
      class="ripple"
      :style="rippleStyle"
      ref="rippleRefs"
    ></div>
  </button>
</template>

<script setup lang="ts">
import { ref, computed, nextTick } from 'vue';
import { useThemeStore } from '../stores/theme';

const themeStore = useThemeStore();
const buttonRef = ref<HTMLButtonElement>();
const rippleRefs = ref<HTMLDivElement[]>([]);
const ripples = ref<{ x: number; y: number; size: number; id: number }[]>([]);
let rippleId = 0;

const currentIcon = computed(() => {
  return themeStore.currentTheme === 'dark' ? '🌙' : '☀️';
});

const rippleStyle = computed(() => ({
  '--ripple-color': themeStore.currentTheme === 'dark' 
    ? 'rgba(255, 255, 255, 0.35)' 
    : 'rgba(0, 0, 0, 0.25)'
}));

function handleToggle(event: MouseEvent) {
  if (!buttonRef.value) return;

  const rect = buttonRef.value.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const y = event.clientY - rect.top;
  const size = Math.max(rect.width, rect.height) * 2.5;

  const id = rippleId++;
  ripples.value.push({ x, y, size, id });

  nextTick(() => {
    setTimeout(() => {
      const index = ripples.value.findIndex(r => r.id === id);
      if (index !== -1) {
        ripples.value.splice(index, 1);
      }
    }, 500);
  });

  themeStore.toggleTheme();
}
</script>

<style scoped>
.theme-toggle-button {
  position: relative;
  width: 40px;
  height: 40px;
  border: none;
  border-radius: 10px;
  background: transparent;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  transition: background 0.15s;
}

.theme-toggle-button:hover {
  background: var(--hover-bg, rgba(0, 0, 0, 0.05));
}

.theme-toggle-button .icon {
  font-size: 18px;
  position: relative;
  z-index: 1;
}

.ripple {
  position: absolute;
  border-radius: 50%;
  background: var(--ripple-color);
  transform: scale(0);
  animation: ripple 0.5s ease-out forwards;
  pointer-events: none;
  z-index: 0;
}

@keyframes ripple {
  0% {
    transform: scale(0);
    opacity: 1;
  }
  100% {
    transform: scale(1);
    opacity: 0;
  }
}
</style>
