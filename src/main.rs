use askama::Template;
use axum::{extract::Path, routing::get, Router};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();
    let assets_path = "./assets".to_string();
    let app = Router::new()
        .route("/", get(home_handler))
        .route("/*key", get(hello_handler))
        .route("/click", get(click_handler))
        .nest_service("/assets", ServeDir::new(assets_path));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn home_handler() -> HomeTemplate {
    HomeTemplate::new("Hello World".to_string())
}

async fn hello_handler(Path(key): Path<String>) -> HomeTemplate {
    HomeTemplate::new(format!("hello {}", key))
}

#[derive(Template)]
#[template(path = "pages/home.html")]
struct HomeTemplate {
    greeting: String,
}
impl HomeTemplate {
    fn new(greeting: String) -> Self {
        Self { greeting }
    }
}

async fn click_handler() -> ClickTemplate {
    ClickTemplate
}

#[derive(Template)]
#[template(path = "components/click.html")]
struct ClickTemplate;
