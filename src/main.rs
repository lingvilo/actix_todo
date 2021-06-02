use std::{env, io};

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

mod api;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_todo=debug,actix_web=info");

    // let app = move || {
    //     println!("Constructing the App");

    //     App::new()
    //         .wrap(Logger::default())
    //         .route("/", web::get().to(api::index))
    // };

    // println!("Starting server");
    // HttpServer::new(app).bind("0.0.0.0:8088")?.run().await

    println!("Starting server");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(api::index))
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}
