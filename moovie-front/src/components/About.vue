<template>
  <div class="about-page">
    <div class="page-header">
      <div class="page-header-content">
        <h1 class="page-title">关于</h1>
        <p class="page-subtitle">TTTTV - 精彩影视，尽在掌握</p>
      </div>
    </div>

    <div class="about-content">
      <div class="logo-section">
        <div class="app-icon">🎬</div>
        <h2 class="app-name">TTTTV</h2>
        <p class="app-version">v1.0.0</p>
      </div>

      <div class="card-section">
        <h3 class="section-title">DMCA 声明</h3>
        <div class="section-content">
          <h4 class="subsection-title">版权声明</h4>
          <p class="section-text">
            TTTTV 尊重知识产权，遵守《数字千年版权法案》(DMCA) 及相关法律法规。
          </p>
          
          <h4 class="subsection-title">免责声明</h4>
          <p class="section-text">
            本软件不存储任何视频文件，所有内容均来自互联网公开资源。本软件仅提供搜索和索引服务，不对任何第三方提供的内容负责。
          </p>
          
          <h4 class="subsection-title">版权投诉</h4>
          <p class="section-text">
            如果您是版权所有者，认为本软件索引的内容侵犯了您的权益，请通过 issues 页面提交投诉，制作者将在 24-48 小时内处理。
          </p>
          <p class="section-text">
            <strong>投诉时请提供：</strong>
          </p>
          <ul class="section-list">
            <li>您的联系方式</li>
            <li>侵权内容的具体链接</li>
            <li>版权所有权证明</li>
          </ul>
        </div>
      </div>

      <div class="card-section">
        <h3 class="section-title">GitHub</h3>
        <div class="github-section">
          <div class="github-info">
            <el-icon :size="40"><Document /></el-icon>
            <div class="github-text">
              <p class="github-repo">TTTTV</p>
              <p class="github-desc">Windows 端，Win11 原生软件风格的聚合全网资源的影视搜索、观看器</p>
            </div>
          </div>
          <el-button 
            type="primary" 
            @click="openGitHub"
            class="github-button"
          >
            <el-icon><Document /></el-icon>
            访问 GitHub
          </el-button>
        </div>
      </div>

      <div class="card-section">
        <h3 class="section-title">许可证</h3>
        <div class="license-section">
          <el-tag type="info" size="large">MIT License</el-tag>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ElMessage } from 'element-plus';
import { Document } from '@element-plus/icons-vue';

const GITHUB_URL = 'https://github.com/WEP-56/TTTTV';

function openGitHub() {
  try {
    const a = document.createElement('a');
    a.href = GITHUB_URL;
    a.target = '_blank';
    a.rel = 'noopener noreferrer';
    a.click();
  } catch (error) {
    console.error('打开 GitHub 失败:', error);
    ElMessage.error('无法打开链接');
  }
}
</script>

<style scoped>
.about-page {
  width: 100%;
}

.about-content {
  max-width: 600px;
  margin: 0 auto;
}

.logo-section {
  text-align: center;
  margin-bottom: 32px;
  padding: 32px 0;
}

.app-icon {
  font-size: 64px;
  margin-bottom: 16px;
}

.app-name {
  font-size: 28px;
  font-weight: 700;
  margin: 0 0 8px 0;
  color: var(--el-text-color-primary);
}

.app-version {
  font-size: 14px;
  color: var(--el-text-color-secondary);
  margin: 0;
}

.card-section {
  background: var(--el-fill-color-blank);
  border-radius: 12px;
  padding: 24px;
  margin-bottom: 16px;
  border: 1px solid var(--el-border-color-lighter);
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  margin: 0 0 16px 0;
  color: var(--el-text-color-primary);
}

.section-content {
  line-height: 1.7;
}

.subsection-title {
  font-size: 14px;
  font-weight: 600;
  margin: 20px 0 8px 0;
  color: var(--el-text-color-primary);
}

.section-text {
  font-size: 13px;
  color: var(--el-text-color-regular);
  margin: 0 0 12px 0;
}

.section-list {
  font-size: 13px;
  color: var(--el-text-color-regular);
  margin: 0 0 12px 0;
  padding-left: 20px;
}

.section-list li {
  margin-bottom: 6px;
}

.github-section {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
}

.github-info {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 1;
}

.github-text {
  flex: 1;
  min-width: 0;
}

.github-repo {
  font-size: 14px;
  font-weight: 600;
  margin: 0 0 4px 0;
  color: var(--el-text-color-primary);
}

.github-desc {
  font-size: 12px;
  color: var(--el-text-color-secondary);
  margin: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.github-button {
  flex-shrink: 0;
}

.license-section {
  display: flex;
  justify-content: center;
}
</style>