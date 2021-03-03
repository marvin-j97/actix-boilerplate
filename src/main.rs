use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello_world() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  println!("Running actix web boilerplate, visit http://localhost:8080");
  HttpServer::new(|| App::new().service(hello_world))
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
