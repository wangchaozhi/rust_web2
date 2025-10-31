mod models;
mod db;
mod handlers;

use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;

use db::{init_pool, init_database};
use handlers::{
    index,
    get_users,
    get_user,
    create_new_user,
    update_existing_user,
    delete_existing_user,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    // 数据库文件路径
    let database_url = "rust_web2.db";
    
    log::info!("🔧 正在初始化数据库连接池...");
    
    // 初始化数据库连接池
    let pool = init_pool(database_url)
        .expect("❌ 无法创建数据库连接池");
    
    // 初始化数据库表结构
    init_database(&pool)
        .expect("❌ 数据库初始化失败");
    
    let pool_data = web::Data::new(pool);
    
    let bind_address = "127.0.0.1";
    let bind_port = 8080;
    
    log::info!("🚀 服务器启动在 http://{}:{}", bind_address, bind_port);
    log::info!("📊 数据库文件: {}", database_url);
    
    HttpServer::new(move || {
        // 配置 CORS
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .max_age(3600);
        
        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(pool_data.clone())
            // 注册路由
            .service(index)
            .service(get_users)
            .service(get_user)
            .service(create_new_user)
            .service(update_existing_user)
            .service(delete_existing_user)
    })
    .bind((bind_address, bind_port))?
    .run()
    .await
}
