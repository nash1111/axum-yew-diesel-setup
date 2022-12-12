use axum::{ http::StatusCode, response::IntoResponse, routing::{ get, post }, Json, Router };
use serde::{ Deserialize, Serialize };
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // initialize logger
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    // build our application with a single route
    let app = create_app();

    let addr = &"0.0.0.0:8000".parse().unwrap();
    axum::Server::bind(addr).serve(app.into_make_service()).await.unwrap();
    tracing::debug!("listening on {}", addr);
}

fn create_app() -> Router {
    Router::new()
        .route(
            "/",
            get(|| async { "Hello, World!" })
        )
        .route("/users", post(create_user))
}

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct CreateUser {
    username: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct User {
    id: u64,
    username: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{ body::Body, http::{ header, Method, Request } };
    use tower::ServiceExt;
    #[tokio::test]
    async fn should_return_hello_world() {
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = create_app().oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        assert_eq!(body, "Hello, World!");
    }

    #[tokio::test]
    async fn should_return_user_data() {
        let req = Request::builder()
            .uri("/users")
            .method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(r#"{"username": "John Doe"}"#))
            .unwrap();
        let res = create_app().oneshot(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        let user: User = serde_json::from_str(&body).expect("Failed to deserialize user");
        assert_eq!(user, User { id: 1337, username: "John Doe".to_string() });
    }
}