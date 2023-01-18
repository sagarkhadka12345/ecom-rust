use actix_web::{get, web, Responder};
use dotenv::dotenv;
use std::env;

use crate::model::main::Products;
use futures::stream::TryStreamExt;
use mongodb::Collection;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};

pub async fn get_all_products() -> Vec<Products> {
    dotenv().ok();
    let mongo_db_url = env::var("MONGOURL").expect("NO ENV SET");
    let options =
        ClientOptions::parse_with_resolver_config(&mongo_db_url, ResolverConfig::cloudflare())
            .await
            .expect("Connection Error");
    let client = Client::with_options(options).expect("Error while connecting");

    let collection: Collection<Products> = client.database("myFirstDatabase").collection("items");

    let mut cursor = collection.find(None, None).await.expect("NO product Found");
    let mut vector = Vec::new();
    while let Ok(Some(book)) = cursor.try_next().await {
        vector.push(book)
    }

    return vector;
}

#[get["/all"]]
pub async fn getallproducts() -> impl Responder {
    let data_array = get_all_products().await;
    web::Json(data_array)
}
