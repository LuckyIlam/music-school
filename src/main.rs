use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use dotenv::dotenv;
use sqlx::PgPool;

mod db;
mod models;
mod handlers;

// Basic health check endpoint
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("API is healthy!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load .env file

    // Establish database connection pool
    let pool: PgPool = match db::establish_connection().await {
        Ok(p) => {
            println!("Successfully connected to the database.");
            p
        }
        Err(e) => {
            eprintln!("Failed to connect to the database: {:?}", e);
            std::process::exit(1); // Exit if connection fails
        }
    };

    // Start HTTP server
    println!("Starting server on 127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Share connection pool
            .route("/health", web::get().to(health_check)) // Add health check
            .service(handlers::create_student_handler) // Register student creation handler
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
