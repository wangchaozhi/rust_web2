# 🏗️ 项目架构说明

## 后端架构

本项目采用模块化设计，代码清晰、易于维护和扩展。

### 模块划分

```
src/
├── main.rs       # 应用入口，负责启动服务器
├── models.rs     # 数据模型定义
├── db.rs         # 数据库操作层
└── handlers.rs   # API 处理器层
```

### 1. models.rs - 数据模型层

定义了应用中使用的所有数据结构：

- `User` - 用户实体
- `CreateUser` - 创建用户的请求体
- `ApiResponse<T>` - 统一的 API 响应包装器

**职责**：
- 定义数据结构
- 序列化/反序列化配置
- 类型安全保证

### 2. db.rs - 数据访问层

负责所有数据库相关操作：

- `init_pool()` - 初始化数据库连接池
- `init_database()` - 初始化数据库表结构
- `get_all_users()` - 查询所有用户
- `get_user_by_id()` - 根据 ID 查询用户
- `create_user()` - 创建新用户
- `update_user()` - 更新用户信息
- `delete_user()` - 删除用户

**职责**：
- 数据库连接管理
- SQL 查询执行
- 数据映射（DB ↔ Model）
- 错误处理

**设计模式**：
- 使用 r2d2 连接池，提高并发性能
- 所有函数返回 `Result<T>` 类型，便于错误传播
- 使用 `?` 操作符简化错误处理

### 3. handlers.rs - API 处理层

定义所有 HTTP 请求处理函数：

- `index()` - 首页/API 信息
- `get_users()` - GET /api/users
- `get_user()` - GET /api/users/{id}
- `create_new_user()` - POST /api/users
- `update_existing_user()` - PUT /api/users/{id}
- `delete_existing_user()` - DELETE /api/users/{id}

**职责**：
- HTTP 请求解析
- 参数验证
- 调用数据访问层
- HTTP 响应构建
- 错误转换（DB Error → HTTP Response）

**特点**：
- 使用 Actix-Web 宏简化路由定义
- 统一的响应格式（ApiResponse）
- 友好的错误提示（如邮箱重复）

### 4. main.rs - 应用入口

负责应用的启动和配置：

**职责**：
- 初始化日志系统
- 创建数据库连接池
- 配置 CORS
- 注册路由
- 启动 HTTP 服务器

## 数据流

```
┌─────────────┐
│   Client    │
│  (Browser)  │
└──────┬──────┘
       │ HTTP Request
       ↓
┌─────────────────────┐
│   handlers.rs       │
│  - 解析请求         │
│  - 参数验证         │
└──────┬──────────────┘
       │
       ↓
┌─────────────────────┐
│     db.rs           │
│  - 执行 SQL         │
│  - 数据转换         │
└──────┬──────────────┘
       │
       ↓
┌─────────────────────┐
│    SQLite DB        │
│  - 持久化存储       │
└─────────────────────┘
```

## 前端架构

```
frontend/src/
├── main.js           # 应用入口
├── App.vue           # 根组件
├── style.css         # 全局样式
├── router/           # 路由配置
│   └── index.js
├── api/              # API 封装
│   └── index.js
└── views/            # 页面组件
    ├── Home.vue
    └── Users.vue
```

### API 层设计

在 `api/index.js` 中统一管理 API 请求：

- 使用 Axios 拦截器
- 统一错误处理
- 请求/响应转换
- 超时配置

### 组件设计

- **Home.vue** - 首页展示
  - API 状态检查
  - 功能特性介绍
  
- **Users.vue** - 用户管理
  - 用户列表展示
  - 创建/编辑用户表单
  - 删除确认
  - 错误提示

## 技术亮点

### 1. 类型安全
- Rust 的强类型系统保证编译时安全
- Serde 自动序列化/反序列化

### 2. 错误处理
- Result 类型强制处理错误
- 自定义错误信息
- 友好的用户提示

### 3. 性能优化
- 数据库连接池复用连接
- 异步处理（Tokio + Actix）
- 编译时优化（LTO）

### 4. 代码组织
- 单一职责原则
- 分层架构
- 模块化设计

### 5. 开发体验
- 热更新（Vite）
- 日志系统（env_logger）
- 清晰的错误信息

## 扩展建议

### 添加新的实体

1. 在 `models.rs` 中定义新的结构体
2. 在 `db.rs` 中添加数据库操作函数
3. 在 `handlers.rs` 中添加 API 处理器
4. 在 `main.rs` 中注册新的路由

### 添加认证

可以添加新的模块：
- `auth.rs` - 认证逻辑
- `middleware.rs` - 认证中间件
- 使用 JWT 或 Session

### 添加更多功能

- 分页查询
- 搜索和过滤
- 数据验证
- 文件上传
- WebSocket 实时通信

## 最佳实践

1. **保持模块独立**：每个模块应有明确的职责
2. **使用类型系统**：利用 Rust 类型系统避免运行时错误
3. **错误处理**：不要 `unwrap()`，使用 `?` 或 `match`
4. **日志记录**：记录关键操作和错误
5. **代码复用**：提取共同逻辑到独立函数
6. **测试**：为关键函数编写单元测试

## 性能考虑

- SQLite 适合中小型应用
- 如需更高性能，可替换为 PostgreSQL/MySQL
- 使用 `cargo build --release` 构建生产版本
- 考虑使用 CDN 托管前端静态资源

