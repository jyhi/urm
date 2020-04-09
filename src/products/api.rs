use serde::Serialize;
use rocket_contrib::databases::mongodb;
use rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::database::UrmDb;
use crate::product::Product;

#[derive(Serialize)]
pub struct Context {
  pub products: Vec<Product>,
}

impl Context {
  pub fn from_db(db: &UrmDb) -> Result<Self, mongodb::error::Error> {
    let products: Vec<Product> = db.collection("products")
      .find(None, None)?
      .map(|p| Product::from(p.unwrap_or(Default::default()))) // XXX: TODO: Error handling
      .collect();

    Ok(Context {
      products: products,
    })
  }
}
