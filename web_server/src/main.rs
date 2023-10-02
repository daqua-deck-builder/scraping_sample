use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(handler));

    let port = 3000;

    // run it with hyper on localhost:3000
    let server = axum::Server::bind(&format!("0.0.0.0:{}", port).as_str().parse().unwrap())
        .serve(app.into_make_service());
    println!("start listening on port {}", port);
    server.await
        .unwrap();

}

async fn handler() -> Html<&'static str> {
    Html("<html><head><title>Home</title></head><body><h1>Hello, World!</h1></body></html>")
}