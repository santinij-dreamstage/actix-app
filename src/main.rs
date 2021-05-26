use actix_web::{middleware, web, App, HttpResponse, HttpServer};

#[derive(Clone, Debug, Default)]
pub struct Context;

#[actix_web::get("/health")]
pub async fn health(_context: web::Data<Context>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(Context::default())
            .service(health)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
