use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Products {
    seller: String,
    price: i32,
    productId: String,
    img: String,
}
