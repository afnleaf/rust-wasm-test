
use actix_web::{get, web, App, HttpServer, Result};
use maud::{html, Markup, DOCTYPE};
use serde::Serialize;

#[derive(Serialize)]
struct PageData {
    title: String,
    items: Vec<String>,
    user: Option<String>,
}

fn base(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                title { (title) }
                style {
                    "nav { background: #f0f0f0; padding: 1em; margin-bottom: 2em; }"
                    "main { padding: 0 2em; }"
                    ".container { max-width: 800px; margin: 0 auto; }"
                }
            }
            body {
                div class="container" {
                    nav {
                        a href="/" { "Home" }
                        " | "
                        a href="/about" { "About" }
                    }
                    main {
                        (content)
                    }
                }
            }
        }
    }
}

#[get("/")]
async fn index() -> Result<Markup> {
    let data = PageData {
        title: "Welcome".into(),
        items: vec!["Item 1".into(), "Item 2".into(), "Item 3".into()],
        user: Some("John Doe".into()),
    };

    let content = html! {
        @if let Some(user) = &data.user {
            p class="welcome" { "Welcome, " (user) "!" }
        }

        h1 { (data.title) }

        @if !data.items.is_empty() {
            ul {
                @for item in &data.items {
                    li { (item) }
                }
            }
        } @else {
            p { "No items to display." }
        }
    };

    Ok(base(&data.title, content))
}

#[get("/about")]
async fn about() -> Result<Markup> {
    let content = html! {
        h1 { "About Us" }
        p { "This is a cleaner implementation using Maud templates with Actix." }
        
        section class="features" {
            h2 { "Why Maud?" }
            ul {
                li { "Type-safe templates" }
                li { "Compile-time checking" }
                li { "Zero runtime overhead" }
                li { "Clean, Rust-like syntax" }
            }
        }
    };

    Ok(base("About Us", content))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:7878");
    
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(about)
    })
    .bind("127.0.0.1:7878")?
    .run()
    .await
}
