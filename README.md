# 🦀 Rust Web2 - 全栈应用

一个使用 Rust (Actix-Web) 作为后端、Vue3 + Vite 作为前端的现代化全栈 Web 应用。

> 💡 **新手？** 查看 [快速开始指南](QUICKSTART.md) 在 5 分钟内启动项目！

## 📋 技术栈

### 后端
- **Rust** - 系统编程语言，高性能、内存安全
- **Actix-Web** - 强大的异步 Web 框架
- **rusqlite** - SQLite 数据库驱动
- **r2d2** - 数据库连接池
- **Serde** - 序列化/反序列化库
- **Tokio** - 异步运行时

### 前端
- **Vue 3** - 渐进式 JavaScript 框架
- **Vite** - 下一代前端构建工具
- **Vue Router** - 官方路由管理器
- **Pinia** - 轻量级状态管理
- **Axios** - HTTP 客户端

## 🚀 快速开始

### 前置要求

确保已安装以下工具：
- Rust (1.70+) - [安装指南](https://www.rust-lang.org/tools/install)
- Node.js (18+) - [下载地址](https://nodejs.org/)
- Yarn - 运行 `npm install -g yarn`

### 安装依赖

#### 1. 后端依赖（自动安装）
```bash
# Cargo 会在首次运行时自动下载依赖
```

#### 2. 前端依赖
```bash
cd frontend
yarn install
```

### 运行项目

#### 启动后端服务
在项目根目录运行：
```bash
cargo run
```

后端服务将在 `http://localhost:8080` 启动

#### 启动前端服务
在新的终端窗口中：
```bash
cd frontend
yarn dev
```

前端服务将在 `http://localhost:5173` 启动

### 访问应用

打开浏览器访问：`http://localhost:5173`

## 📁 项目结构

```
rust-web2/
├── src/                    # Rust 后端源码
│   ├── main.rs            # 主程序入口
│   ├── models.rs          # 数据模型
│   ├── db.rs              # 数据库操作
│   └── handlers.rs        # API 处理器
├── frontend/              # Vue3 前端项目
│   ├── src/
│   │   ├── api/          # API 请求封装
│   │   ├── router/       # 路由配置
│   │   ├── views/        # 页面组件
│   │   ├── App.vue       # 根组件
│   │   ├── main.js       # 前端入口
│   │   └── style.css     # 全局样式
│   ├── index.html        # HTML 模板
│   ├── package.json      # 前端依赖
│   └── vite.config.js    # Vite 配置
├── Cargo.toml            # Rust 依赖配置
├── rust_web2.db          # SQLite 数据库文件（运行后生成）
├── .gitignore
└── README.md
```

## 🔌 API 端点

### 获取所有用户
```http
GET http://localhost:8080/api/users
```

### 获取单个用户
```http
GET http://localhost:8080/api/users/{id}
```

### 创建用户
```http
POST http://localhost:8080/api/users
Content-Type: application/json

{
  "name": "张三",
  "email": "zhangsan@example.com"
}
```

### 更新用户
```http
PUT http://localhost:8080/api/users/{id}
Content-Type: application/json

{
  "name": "张三",
  "email": "newemail@example.com"
}
```

### 删除用户
```http
DELETE http://localhost:8080/api/users/{id}
```

### 获取 API 信息
```http
GET http://localhost:8080/
```

## 🛠️ 开发

### 后端开发

#### 运行开发服务器（带日志）
```bash
RUST_LOG=info cargo run
```

#### 构建发布版本
```bash
cargo build --release
```

#### 运行测试
```bash
cargo test
```

### 前端开发

#### 开发模式（热更新）
```bash
cd frontend
yarn dev
```

#### 构建生产版本
```bash
cd frontend
yarn build
```

#### 预览生产构建
```bash
cd frontend
yarn preview
```

## ✨ 功能特性

- ✅ RESTful API 设计
- ✅ SQLite 数据库持久化
- ✅ 数据库连接池管理
- ✅ 模块化代码架构
- ✅ CORS 跨域支持
- ✅ 响应式前端界面
- ✅ 完整的用户 CRUD 操作
- ✅ 现代化 UI/UX 设计
- ✅ API 代理配置
- ✅ 错误处理和加载状态
- ✅ 表单验证（邮箱唯一性）

## 📝 配置说明

### 数据库

项目使用 SQLite 数据库，数据库文件会在首次运行时自动创建：
- 文件名：`rust_web2.db`
- 位置：项目根目录
- 自动初始化表结构和测试数据

无需手动配置，开箱即用！

### 环境变量

#### 后端
可通过环境变量配置日志级别：
```bash
RUST_LOG=debug cargo run
```

#### 前端
前端通过 Vite 的代理配置连接后端，配置在 `frontend/vite.config.js`

## 🎨 自定义配置

### 修改后端端口
编辑 `src/main.rs` 中的端口配置：
```rust
.bind(("127.0.0.1", 8080))?
```

### 修改前端端口
编辑 `frontend/vite.config.js`：
```javascript
server: {
  port: 5173
}
```

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

MIT License

## 👨‍💻 作者

Rust Web2 Team

## 📚 更多文档

- [架构设计文档](ARCHITECTURE.md) - 详细的模块化架构说明

## 🎯 项目特点

### 后端亮点
- 🦀 **Rust 原生性能** - 接近 C/C++ 的执行速度
- 🔒 **内存安全** - 编译时保证，零成本抽象
- 📦 **模块化设计** - 清晰的分层架构（models/db/handlers）
- 💾 **SQLite 集成** - 轻量级、零配置数据库
- 🔄 **连接池管理** - r2d2 提供高效的连接复用
- ✅ **统一 API 响应** - 标准化的响应格式

### 前端亮点
- ⚡ **Vue 3 Composition API** - 更好的代码组织和复用
- 🎨 **现代化 UI** - 渐变色、卡片式设计、流畅动画
- 🔧 **Vite 开发服务器** - 极速热更新
- 📱 **响应式布局** - 适配各种屏幕尺寸
- 🎭 **交互式反馈** - 加载状态、错误提示、确认对话框

---

**享受编码！** 🚀

