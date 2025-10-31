<template>
  <div id="app">
    <header class="header">
      <div class="container">
        <h1>🦀 Rust + Vue3 全栈应用</h1>
        <nav>
          <router-link to="/">首页</router-link>
          <router-link to="/users">用户列表</router-link>
        </nav>
      </div>
    </header>
    
    <main class="main">
      <router-view />
    </main>
    
    <footer class="footer">
      <p>Powered by Rust (Actix-Web) + Vue3 + Vite</p>
    </footer>
  </div>
</template>

<script setup>
</script>

<style scoped>
.header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 1.5rem 0;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 2rem;
}

.header h1 {
  margin: 0 0 1rem 0;
  font-size: 2rem;
}

nav {
  display: flex;
  gap: 1.5rem;
}

nav a {
  color: white;
  text-decoration: none;
  padding: 0.5rem 1rem;
  border-radius: 4px;
  transition: background 0.3s;
}

nav a:hover,
nav a.router-link-active {
  background: rgba(255, 255, 255, 0.2);
}

.main {
  min-height: calc(100vh - 200px);
  padding: 2rem 0;
}

.footer {
  background: #2c3e50;
  color: white;
  text-align: center;
  padding: 2rem 0;
  margin-top: 4rem;
}
</style>

