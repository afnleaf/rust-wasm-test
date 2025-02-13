// extremely minimal code amount web server
// 800mb of dependencies

use actix_files as fs;
use actix_web::{App, HttpServer, middleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    unsafe {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                fs::Files::new("/", "./public")
                    .index_file("index.html")
                    .show_files_listing()
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
