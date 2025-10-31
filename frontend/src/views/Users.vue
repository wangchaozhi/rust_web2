<template>
  <div class="container">
    <div class="users-page">
      <div class="page-header">
        <h1>👥 用户管理</h1>
        <button @click="showCreateForm = !showCreateForm">
          {{ showCreateForm ? '取消' : '添加用户' }}
        </button>
      </div>
      
      <!-- 创建/编辑用户表单 -->
      <div class="create-form card" v-if="showCreateForm">
        <h2>{{ editingUser ? '编辑用户' : '创建新用户' }}</h2>
        <form @submit.prevent="handleSubmitUser">
          <div class="form-group">
            <label for="name">姓名</label>
            <input
              id="name"
              v-model="formUser.name"
              type="text"
              placeholder="请输入姓名"
              required
            />
          </div>
          <div class="form-group">
            <label for="email">邮箱</label>
            <input
              id="email"
              v-model="formUser.email"
              type="email"
              placeholder="请输入邮箱"
              required
            />
          </div>
          <div class="form-actions">
            <button type="submit" :disabled="loading">
              {{ loading ? '处理中...' : (editingUser ? '更新用户' : '创建用户') }}
            </button>
            <button type="button" @click="cancelForm" class="btn-cancel">
              取消
            </button>
          </div>
        </form>
      </div>
      
      <!-- 用户列表 -->
      <div class="users-list" v-if="!loading || users.length > 0">
        <div class="user-card card" v-for="user in users" :key="user.id">
          <div class="user-avatar">
            {{ user.name.charAt(0) }}
          </div>
          <div class="user-info">
            <h3>{{ user.name }}</h3>
            <p>{{ user.email }}</p>
            <span class="user-id">ID: {{ user.id }}</span>
          </div>
          <div class="user-actions">
            <button @click="handleEditUser(user)" class="btn-edit" title="编辑">
              ✏️
            </button>
            <button @click="handleDeleteUser(user.id)" class="btn-delete" title="删除">
              🗑️
            </button>
          </div>
        </div>
        
        <div v-if="users.length === 0" class="empty-state">
          <p>暂无用户数据</p>
        </div>
      </div>
      
      <!-- 加载状态 -->
      <div class="loading-state" v-if="loading && users.length === 0">
        <div class="spinner"></div>
        <p>加载中...</p>
      </div>
      
      <!-- 错误提示 -->
      <div class="error-message" v-if="error">
        <p>❌ {{ error }}</p>
        <button @click="fetchUsers">重试</button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { userApi } from '../api'

const users = ref([])
const loading = ref(false)
const error = ref(null)
const showCreateForm = ref(false)
const editingUser = ref(null)
const formUser = ref({
  name: '',
  email: ''
})

const fetchUsers = async () => {
  loading.value = true
  error.value = null
  try {
    const response = await userApi.getUsers()
    // 处理新的响应格式
    users.value = response.data || response
  } catch (err) {
    error.value = '无法加载用户列表，请确保后端服务正在运行'
    console.error('Error fetching users:', err)
  } finally {
    loading.value = false
  }
}

const handleSubmitUser = async () => {
  if (!formUser.value.name || !formUser.value.email) {
    return
  }
  
  loading.value = true
  try {
    if (editingUser.value) {
      // 更新用户
      const response = await userApi.updateUser(editingUser.value.id, formUser.value)
      const updatedUser = response.data || response
      const index = users.value.findIndex(u => u.id === editingUser.value.id)
      if (index !== -1) {
        users.value[index] = updatedUser
      }
    } else {
      // 创建用户
      const response = await userApi.createUser(formUser.value)
      const createdUser = response.data || response
      users.value.push(createdUser)
    }
    cancelForm()
  } catch (err) {
    const action = editingUser.value ? '更新' : '创建'
    error.value = `${action}用户失败: ${err.response?.data?.message || err.message}`
    console.error(`Error ${action} user:`, err)
  } finally {
    loading.value = false
  }
}

const handleEditUser = (user) => {
  editingUser.value = user
  formUser.value = { name: user.name, email: user.email }
  showCreateForm.value = true
}

const handleDeleteUser = async (userId) => {
  if (!confirm('确定要删除这个用户吗？')) {
    return
  }
  
  loading.value = true
  try {
    await userApi.deleteUser(userId)
    users.value = users.value.filter(u => u.id !== userId)
  } catch (err) {
    error.value = '删除用户失败'
    console.error('Error deleting user:', err)
  } finally {
    loading.value = false
  }
}

const cancelForm = () => {
  showCreateForm.value = false
  editingUser.value = null
  formUser.value = { name: '', email: '' }
}

onMounted(() => {
  fetchUsers()
})
</script>

<style scoped>
.users-page {
  max-width: 900px;
  margin: 0 auto;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
}

.page-header h1 {
  font-size: 2.5rem;
  color: #2c3e50;
}

.create-form {
  margin-bottom: 2rem;
  animation: slideDown 0.3s ease;
}

@keyframes slideDown {
  from {
    opacity: 0;
    transform: translateY(-20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.create-form h2 {
  margin-bottom: 1.5rem;
  color: #2c3e50;
}

.form-group {
  margin-bottom: 1.5rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  color: #2c3e50;
  font-weight: 600;
}

.form-group input {
  width: 100%;
}

.users-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1.5rem;
}

.user-card {
  display: flex;
  gap: 1.5rem;
  align-items: center;
  position: relative;
}

.user-avatar {
  width: 60px;
  height: 60px;
  border-radius: 50%;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.5rem;
  font-weight: bold;
  flex-shrink: 0;
}

.user-info {
  flex: 1;
}

.user-info h3 {
  margin-bottom: 0.5rem;
  color: #2c3e50;
  font-size: 1.25rem;
}

.user-info p {
  color: #666;
  margin-bottom: 0.5rem;
}

.user-id {
  display: inline-block;
  padding: 0.25rem 0.5rem;
  background: #f0f0f0;
  border-radius: 4px;
  font-size: 0.85rem;
  color: #666;
}

.user-actions {
  display: flex;
  gap: 0.5rem;
  margin-left: auto;
}

.user-actions button {
  padding: 0.5rem 0.75rem;
  font-size: 1rem;
  min-width: auto;
  background: transparent;
  border: none;
  cursor: pointer;
  transition: all 0.3s;
}

.btn-edit:hover {
  background: #e3f2fd;
  transform: scale(1.1);
}

.btn-delete:hover {
  background: #ffebee;
  transform: scale(1.1);
}

.form-actions {
  display: flex;
  gap: 1rem;
  margin-top: 1rem;
}

.btn-cancel {
  background: #95a5a6;
}

.btn-cancel:hover {
  background: #7f8c8d;
}

.empty-state {
  grid-column: 1 / -1;
  text-align: center;
  padding: 3rem;
  color: #666;
  font-size: 1.2rem;
}

.loading-state {
  grid-column: 1 / -1;
  text-align: center;
  padding: 3rem;
}

.spinner {
  width: 50px;
  height: 50px;
  border: 4px solid #f3f3f3;
  border-top: 4px solid #667eea;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 1rem;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.error-message {
  grid-column: 1 / -1;
  text-align: center;
  padding: 2rem;
  background: #fee;
  border-radius: 12px;
  color: #c00;
}

.error-message button {
  margin-top: 1rem;
}
</style>

