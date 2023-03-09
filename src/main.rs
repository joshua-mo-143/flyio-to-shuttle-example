use axum::Router;
use axum::routing::get;
use axum::Server;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let router = create_router();

    let addr = SocketAddr::from(([0,0,0,0],5140));
    
    Server::bind(&addr).serve(router.into_make_service()).await.unwrap()
}

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(|| async {"Hello, world!"}))
}