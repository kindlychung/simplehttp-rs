use axum::{response::Html, routing::get, Router};
use std::env;

#[tokio::main]
async fn main() {
	    // Read the route prefix from environment variable, default to "api" if not set
    let route_prefix = env::var("HTTP_ROUTE_PREFIX").unwrap_or_else(|_| "api".to_string());
	let route = format!("/{}", route_prefix);

    // build our application with a route
    let app = Router::new().route(&route, get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}