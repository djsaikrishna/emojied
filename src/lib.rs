mod config;

pub mod layout {
    pub use maud::{html, Markup, DOCTYPE};

    fn header(title: &str) -> Markup {
        html! {
            (DOCTYPE)
            meta charset="utf-8";
            link rel="stylesheet" href="app.css";
            title { (title) }
        }
    }

    pub async fn home() -> Markup {
        html! {
            (header("emojiURL"))
            h1 class="text-red-400" { "Hello, world!" };

            @let a = 2;

            @if a == 1 {
                h2 class="text-red-500" { "Hey again, world!" }
            }
        }
    }
}

pub mod fallback {
    pub async fn not_found(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
        (
            axum::http::StatusCode::NOT_FOUND,
            format!("No route {}", uri),
        )
    }
}

pub mod assets {
    use axum::http::StatusCode;
    use hyper::{
        header::{HeaderName, HeaderValue},
        HeaderMap,
    };
    use std::fs;

    pub async fn stylesheet() -> (StatusCode, HeaderMap, String) {
        let mut headers = HeaderMap::new();

        match fs::read_to_string("public/app.css") {
            Ok(content) => {
                headers.insert(
                    HeaderName::from_static("content-type"),
                    HeaderValue::from_static("text/css; charset=utf-8"),
                );

                (StatusCode::OK, headers, content)
            }
            Err(_e) => (StatusCode::NOT_FOUND, headers, String::from("Not found")),
        }
    }
}