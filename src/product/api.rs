use rocket_contrib::databases::mongodb;
use rocket_contrib::databases::mongodb::{
  bson,
  doc,
  db::ThreadedDatabase,
};
use serde::Serialize;
use crate::database::UrmDb;
use super::structure::Product;

#[derive(Serialize)]
pub struct Context {
  pub product: Option<Product>,
}

impl Context {
  pub fn from_db(db: &UrmDb, pn: String)
    -> Result<Self, mongodb::error::Error>
  {
    let product: Option<Product> = match db.collection("products")
      .find_one(Some(doc!{ "pn": pn }), None)? {
        Some(p) => Some(Product::from(p)),
        None => None
      };

    Ok(Context {
      product: product,
    })
  }
}
