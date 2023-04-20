use actix_web::{HttpServer, App};

#[actix_web::main] // Attr to allow async main
async fn main() {
    HttpServer::new(|| App::new()).bind(("127.0.0.1", 8090)).unwrap().run().await.unwrap()
}
