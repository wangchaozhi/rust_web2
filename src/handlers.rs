use actix_web::{get, post, delete, put, web, HttpResponse, Responder};

use crate::db::{DbPool, get_all_users, get_user_by_id, create_user, delete_user as db_delete_user, update_user};
use crate::models::{CreateUser, ApiResponse};

/// 首页 - 获取 API 信息
#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "欢迎使用 Rust Web API",
        "version": "1.0.0",
        "database": "SQLite"
    }))
}

/// 获取所有用户
#[get("/api/users")]
pub async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    match get_all_users(pool.get_ref()) {
        Ok(users) => HttpResponse::Ok().json(ApiResponse::success(users)),
        Err(e) => {
            log::error!("获取用户列表失败: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                format!("获取用户列表失败: {}", e)
            ))
        }
    }
}

/// 根据 ID 获取用户
#[get("/api/users/{id}")]
pub async fn get_user(
    path: web::Path<i64>,
    pool: web::Data<DbPool>
) -> impl Responder {
    let user_id = path.into_inner();
    
    match get_user_by_id(pool.get_ref(), user_id) {
        Ok(Some(user)) => HttpResponse::Ok().json(ApiResponse::success(user)),
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()>::error(
            "用户不存在".to_string()
        )),
        Err(e) => {
            log::error!("获取用户失败: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                format!("获取用户失败: {}", e)
            ))
        }
    }
}

/// 创建新用户
#[post("/api/users")]
pub async fn create_new_user(
    user: web::Json<CreateUser>,
    pool: web::Data<DbPool>
) -> impl Responder {
    match create_user(pool.get_ref(), &user) {
        Ok(new_user) => {
            log::info!("创建用户成功: {} (ID: {})", new_user.name, new_user.id);
            HttpResponse::Created().json(ApiResponse::success(new_user))
        },
        Err(e) => {
            log::error!("创建用户失败: {}", e);
            let error_message = if e.to_string().contains("UNIQUE constraint failed") {
                "邮箱已存在".to_string()
            } else {
                format!("创建用户失败: {}", e)
            };
            HttpResponse::BadRequest().json(ApiResponse::<()>::error(error_message))
        }
    }
}

/// 更新用户
#[put("/api/users/{id}")]
pub async fn update_existing_user(
    path: web::Path<i64>,
    user: web::Json<CreateUser>,
    pool: web::Data<DbPool>
) -> impl Responder {
    let user_id = path.into_inner();
    
    match update_user(pool.get_ref(), user_id, &user) {
        Ok(Some(updated_user)) => {
            log::info!("更新用户成功: {} (ID: {})", updated_user.name, updated_user.id);
            HttpResponse::Ok().json(ApiResponse::success(updated_user))
        },
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()>::error(
            "用户不存在".to_string()
        )),
        Err(e) => {
            log::error!("更新用户失败: {}", e);
            let error_message = if e.to_string().contains("UNIQUE constraint failed") {
                "邮箱已存在".to_string()
            } else {
                format!("更新用户失败: {}", e)
            };
            HttpResponse::BadRequest().json(ApiResponse::<()>::error(error_message))
        }
    }
}

/// 删除用户
#[delete("/api/users/{id}")]
pub async fn delete_existing_user(
    path: web::Path<i64>,
    pool: web::Data<DbPool>
) -> impl Responder {
    let user_id = path.into_inner();
    
    match db_delete_user(pool.get_ref(), user_id) {
        Ok(true) => {
            log::info!("删除用户成功 (ID: {})", user_id);
            HttpResponse::Ok().json(ApiResponse::success(
                serde_json::json!({"message": "用户已删除"})
            ))
        },
        Ok(false) => HttpResponse::NotFound().json(ApiResponse::<()>::error(
            "用户不存在".to_string()
        )),
        Err(e) => {
            log::error!("删除用户失败: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                format!("删除用户失败: {}", e)
            ))
        }
    }
}

