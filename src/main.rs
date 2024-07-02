mod db;
mod error;
mod handlers;

use actix_web::{web, App, HttpServer};
use db::MoaDB;
use dotenvy::dotenv;
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        dotenv().ok();
        let db_url = std::env::var("DATABASE_URL").unwrap_or_default();
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(
                MoaDB::new(&db_url).expect("Failed to start DB"),
            ))
            .service(handlers::get_all_handlers())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[macro_export]
macro_rules! impl_actix_responder {
    ($struct: ty) => {
        impl ::actix_web::Responder for $struct {
            type Body = ::actix_web::body::BoxBody;

            fn respond_to(
                self,
                _req: &::actix_web::HttpRequest,
            ) -> ::actix_web::HttpResponse<Self::Body> {
                ::actix_web::HttpResponse::Ok().json(self)
            }
        }
    };
}
