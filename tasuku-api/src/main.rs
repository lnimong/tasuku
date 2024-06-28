use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World! let there be light !!"
}
