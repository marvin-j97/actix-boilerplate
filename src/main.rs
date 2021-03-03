use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn hello_world() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  println!("Running actix web boilerplate, visit http://localhost:8080");
  HttpServer::new(|| App::new().route("/", web::get().to(hello_world)))
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
