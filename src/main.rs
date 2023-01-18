mod model;
mod services;
use crate::services::getall::getallproducts;
use actix_cors::Cors;
use actix_web::{
    web::{self},
    App, HttpServer,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new().wrap(cors).service(
            web::scope("/api")
                .service(web::scope("/v1").service(web::scope("/item").service(getallproducts)))
                ,
        )
    })
    .bind(("127.0.0.1", 9191))?
    .run()
    .await?;

    Ok(())
}
