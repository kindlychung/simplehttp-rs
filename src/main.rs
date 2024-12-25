use axum::{response::Html, routing::get, Router};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	    // Read the route prefix from environment variable, default to "api" if not set
    let mut route_prefix = env::var("HTTP_ROUTE_PREFIX").unwrap_or_else(|_| "api".to_string());
	if !route_prefix.starts_with("/") {
		route_prefix = format!("/{}", route_prefix);
	}
	if route_prefix.ends_with("/") {
		route_prefix = route_prefix.trim_end_matches("/").to_string();
	}
    // build our application with a route
    let app = Router::new().route(&route_prefix, get(handler));

    // run it
	let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
	let addr = format!("0.0.0.0:{}", port.parse::<u32>()?.to_string());
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
	Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}