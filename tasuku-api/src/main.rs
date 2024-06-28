use std::env;
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};

#[tokio::main]
async fn main() {

    let localhost = "0.0.0.0".to_owned();

    let port = match env::var_os("PORT") {
        Some(v) => v.into_string().unwrap(),
        None => "80".to_string()
    };

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(localhost+":"+&port).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World! let there be light !!"
}
