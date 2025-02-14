use actix_web::{get, middleware, App, HttpServer, Result};
use maud::{html, Markup, DOCTYPE, PreEscaped};
use std::path::PathBuf;
use std::env;

mod builder;

#[get("/")]
async fn index() -> Result<Markup> {
    Ok(html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                title { "Rust + WASM Demo" }
                style {
                    "body { font-family: sans-serif; max-width: 800px; margin: 0 auto; padding: 2rem; }"
                }
            }
            body {
                h1 { "Rust + WebAssembly Demo" }
                button #changebg { "background" }
                div #counter {
                    p { "Counter: " span #count { "0" } }
                    button #increment { "Increment" }
                }
                // what imports the wasm bindgen functions
                (PreEscaped(
                    r#"<script type="module">
                    import init, { increment_counter, change_background } from '/pkg/wasm_client.js';
                    async function run() {
                        await init();
                        document.getElementById('increment').addEventListener('click', increment_counter);
                        document.getElementById('changebg').addEventListener('click', change_background);
                    }
                    run();
                    </script>"#
                ))
            }
        }
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // build wasm app to pkg
    // cargo manifest dir is where cargo of the running executable is
    let server_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")
                            .unwrap());
    let wasm_dir = server_dir.parent().unwrap().join("wasm-app");
    println!("{:?}", wasm_dir);
    builder::build_wasm(wasm_dir)?;


    //let port = "7878";
    //let ip = "127.0.0.1:";
    let pkg_path = std::path::Path::new("./wasm-app/pkg");
    assert!(pkg_path.is_dir(), "Pkg folder does not exist!");

    println!("Server running at http://127.0.0.1:7878");
    
    HttpServer::new(move || {
        App::new()
            // stop cors issues
            .wrap(
                middleware::DefaultHeaders::new()
                    .add(("Cross-Origin-Opener-Policy", "same-origin"))
                    .add(("Cross-Origin-Embedder-Policy", "require-corp"))
            )
            // main html
            .service(index)
            // serve entire pkg directory as static files
            .service(
                actix_files::Files::new("/pkg", &pkg_path)
                    .prefer_utf8(true)
            )
    })
    .bind("127.0.0.1:7878")?
    .run()
    .await
}
