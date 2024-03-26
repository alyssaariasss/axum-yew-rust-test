use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Router, ServiceExt,
};
use std::io;
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
};

mod handlers;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    dotenv::dotenv().ok();
    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    // add postgres
    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Error with pool connection");

    // add postgres table
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS products (
                    id serial,
                    name text,
                    price integer
                );"#,
    )
    .execute(&pool)
    .await
    .expect("Error with table creation");

    // Cross-Origin Resource Sharing
    // authorize resource sharing with external third parties
    // example: for pulling data from external APIs that are public or authorized
    let cors: CorsLayer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // serve frontend static files
    let serve_dir =
        ServeDir::new("../dist").not_found_service(ServeFile::new("../dist/index.html"));

    // build our application with a route
    let app = Router::new()
        .route("/home", get(home))
        .route(
            "/api/products",
            get(handlers::get_products).post(handlers::create_product),
        )
        .route(
            "/api/products/:id",
            get(handlers::get_one_product)
                .delete(handlers::delete_product)
                .put(handlers::update_product),
        )
        .with_state(pool)
        .layer(cors)
        .nest_service("/", serve_dir.clone())
        .fallback_service(serve_dir.clone());

    tracing::debug!("listening on {}", "0.0.0.0:3000");
    println!("Listening on port 0.0.0.0:3000");

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn home() -> &'static str {
    "Home Page here !"
}
