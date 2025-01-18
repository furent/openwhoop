mod routes;

use actix_files as fs;
use actix_web::{App, HttpServer};
use actix_cors::Cors;
use std::io;

use crate::routes::parse_route::parse_history;


#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("Serving webapp...");

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method(),
            )
            .service(parse_history)
            .service(
                fs::Files::new("/", "../webapp/whoomp/dist")
                .index_file("index.html")
            )
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
