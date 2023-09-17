use axum::Router;
use std::net::SocketAddr;

#[tokio::main] 
async fn main() {
    let app = Router::new();

    let addr = SocketAddr::new([0, 0, 0, 0].into(), 3000);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
