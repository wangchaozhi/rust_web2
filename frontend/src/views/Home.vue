<template>
  <div class="container">
    <div class="hero">
      <h1>🎉 欢迎使用 Rust + Vue3 全栈应用</h1>
      <p class="subtitle">现代化的 Web 开发技术栈</p>
      
      <div class="stats" v-if="apiStatus">
        <div class="stat-card">
          <div class="stat-icon">🦀</div>
          <div class="stat-content">
            <h3>后端</h3>
            <p>Rust + Actix-Web</p>
            <span class="badge success">运行中</span>
          </div>
        </div>
        
        <div class="stat-card">
          <div class="stat-icon">⚡</div>
          <div class="stat-content">
            <h3>前端</h3>
            <p>Vue3 + Vite</p>
            <span class="badge success">运行中</span>
          </div>
        </div>
        
        <div class="stat-card">
          <div class="stat-icon">🚀</div>
          <div class="stat-content">
            <h3>API 版本</h3>
            <p>{{ apiStatus.version }}</p>
            <span class="badge info">{{ apiStatus.message }}</span>
          </div>
        </div>
      </div>
      
      <div class="loading" v-else>
        <p>正在连接后端服务...</p>
      </div>
      
      <div class="features">
        <h2>✨ 特性</h2>
        <div class="feature-grid">
          <div class="feature-item">
            <h3>⚡ 高性能</h3>
            <p>Rust 提供接近 C/C++ 的性能，无 GC 停顿</p>
          </div>
          <div class="feature-item">
            <h3>🔒 内存安全</h3>
            <p>编译时保证内存安全，无空指针和数据竞争</p>
          </div>
          <div class="feature-item">
            <h3>🎨 现代化 UI</h3>
            <p>Vue3 Composition API，响应式开发体验</p>
          </div>
          <div class="feature-item">
            <h3>🛠️ 开发友好</h3>
            <p>Vite 提供极速的热更新和构建体验</p>
          </div>
        </div>
      </div>
      
      <div class="cta">
        <router-link to="/users">
          <button class="btn-large">查看用户列表 →</button>
        </router-link>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import axios from 'axios'

const apiStatus = ref(null)

const checkApi = async () => {
  try {
    const response = await axios.get('http://localhost:8080/')
    apiStatus.value = response.data
  } catch (error) {
    console.error('无法连接到后端服务:', error)
  }
}

onMounted(() => {
  checkApi()
})
</script>

<style scoped>
.hero {
  text-align: center;
  padding: 3rem 0;
}

.hero h1 {
  font-size: 3rem;
  margin-bottom: 1rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.subtitle {
  font-size: 1.5rem;
  color: #666;
  margin-bottom: 3rem;
}

.stats {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 2rem;
  margin: 3rem 0;
}

.stat-card {
  background: white;
  border-radius: 12px;
  padding: 2rem;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  display: flex;
  align-items: center;
  gap: 1.5rem;
  transition: all 0.3s;
}

.stat-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
}

.stat-icon {
  font-size: 3rem;
}

.stat-content {
  text-align: left;
}

.stat-content h3 {
  margin-bottom: 0.5rem;
  color: #2c3e50;
}

.stat-content p {
  color: #666;
  margin-bottom: 0.5rem;
}

.badge {
  display: inline-block;
  padding: 0.25rem 0.75rem;
  border-radius: 12px;
  font-size: 0.85rem;
  font-weight: 600;
}

.badge.success {
  background: #d4edda;
  color: #155724;
}

.badge.info {
  background: #d1ecf1;
  color: #0c5460;
}

.loading {
  padding: 2rem;
  color: #666;
  font-size: 1.2rem;
}

.features {
  margin: 4rem 0;
}

.features h2 {
  font-size: 2.5rem;
  margin-bottom: 2rem;
}

.feature-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 2rem;
}

.feature-item {
  background: white;
  border-radius: 12px;
  padding: 2rem;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  transition: all 0.3s;
}

.feature-item:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
}

.feature-item h3 {
  font-size: 1.5rem;
  margin-bottom: 1rem;
  color: #2c3e50;
}

.feature-item p {
  color: #666;
  line-height: 1.6;
}

.cta {
  margin-top: 3rem;
}

.btn-large {
  padding: 1rem 2.5rem;
  font-size: 1.2rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border: none;
  border-radius: 8px;
  color: white;
  cursor: pointer;
  transition: all 0.3s;
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
}

.btn-large:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(102, 126, 234, 0.6);
}
</style>

