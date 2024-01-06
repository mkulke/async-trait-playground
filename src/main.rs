use actix_web::{get, web, App, HttpServer, Responder};
use frobnicate::{get_frobnicator, Frobnicate};

mod frobnicate;

#[get("/")]
async fn handler(frobnicator: web::Data<impl Frobnicate>) -> impl Responder {
    let msg = frobnicator.frobnicate().await;
    String::from(msg)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let frobnicator = get_frobnicator();
        App::new()
            .service(handler)
            .app_data(web::Data::new(frobnicator))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
