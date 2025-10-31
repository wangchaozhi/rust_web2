# 🚀 快速开始指南

5 分钟内启动你的 Rust + Vue3 全栈应用！

## ⚡ 一键启动

### Windows PowerShell

#### 1. 安装前端依赖并启动后端
```powershell
# 在项目根目录
cd frontend
yarn install
cd ..
cargo run
```

#### 2. 启动前端（打开新的 PowerShell 窗口）
```powershell
cd frontend
yarn dev
```

### Linux / macOS

#### 1. 安装前端依赖并启动后端
```bash
cd frontend && yarn install && cd ..
cargo run
```

#### 2. 启动前端（新终端）
```bash
cd frontend && yarn dev
```

## 🌐 访问应用

- **前端界面**: http://localhost:5173
- **后端 API**: http://localhost:8080

## 📱 功能测试

启动成功后，你可以：

1. **首页** - 查看 API 状态和功能介绍
2. **用户列表** - 点击顶部导航 "用户列表"
3. **添加用户** - 点击 "添加用户" 按钮
4. **编辑用户** - 点击用户卡片上的 ✏️ 按钮
5. **删除用户** - 点击用户卡片上的 🗑️ 按钮

## 🔍 验证 API

### 使用 curl

```bash
# 获取所有用户
curl http://localhost:8080/api/users

# 获取单个用户
curl http://localhost:8080/api/users/1

# 创建用户
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"name":"王五","email":"wangwu@example.com"}'

# 更新用户
curl -X PUT http://localhost:8080/api/users/1 \
  -H "Content-Type: application/json" \
  -d '{"name":"张三","email":"new@example.com"}'

# 删除用户
curl -X DELETE http://localhost:8080/api/users/3
```

### 使用浏览器

直接访问：http://localhost:8080/api/users

## 📊 数据库文件

首次运行后，会在项目根目录生成 `rust_web2.db` 文件。

如需重置数据库，删除该文件并重启后端即可。

## 🛠️ 开发技巧

### 后端热重载

使用 cargo-watch 实现自动重载：

```bash
# 安装 cargo-watch
cargo install cargo-watch

# 启动监听模式
cargo watch -x run
```

### 前端已支持热更新

Vite 默认支持热模块替换（HMR），修改代码后浏览器会自动刷新。

### 查看日志

后端日志会显示在终端，包括：
- 请求路径和方法
- 响应状态码
- 数据库操作
- 错误信息

### 调试模式

```bash
# 启用详细日志
RUST_LOG=debug cargo run

# 仅显示应用日志
RUST_LOG=rust_web2=debug cargo run
```

## ❌ 常见问题

### 端口已被占用

**问题**：`Address already in use`

**解决**：
```bash
# Windows
netstat -ano | findstr :8080
taskkill /PID <进程ID> /F

# Linux/macOS
lsof -i :8080
kill -9 <PID>
```

### 前端无法连接后端

**检查**：
1. 后端是否正在运行？
2. 访问 http://localhost:8080 是否有响应？
3. 查看浏览器控制台是否有 CORS 错误

**解决**：
- 确保后端先启动
- 检查 `src/main.rs` 中的 CORS 配置

### Rust 编译错误

**首次编译**时间较长（5-10 分钟），这是正常的。

**如遇编译错误**：
```bash
# 清理并重新编译
cargo clean
cargo build
```

### Node 模块错误

```bash
# 删除并重新安装
cd frontend
rm -rf node_modules
yarn install
```

## 📚 下一步

- 查看 [README.md](README.md) 了解详细功能
- 阅读 [ARCHITECTURE.md](ARCHITECTURE.md) 理解架构设计
- 修改代码，体验热更新
- 添加新功能

## 💡 提示

- 开发时保持两个终端窗口（前端 + 后端）
- 使用 VS Code / RustRover / WebStorm 等 IDE
- 安装相应的 Rust 和 Vue 插件
- 使用浏览器开发者工具查看网络请求

---

**开始你的全栈开发之旅！** 🎉

