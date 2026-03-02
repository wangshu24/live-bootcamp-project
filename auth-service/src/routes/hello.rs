use axum::response::Html;

pub async fn hello() -> impl IntoResponse {
    Html("<h1>Hello, welcome to this little corner of the intraweb!</h1>")
}
