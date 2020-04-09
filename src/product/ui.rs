use rocket_contrib::databases::mongodb;
use rocket_contrib::databases::mongodb::{
  bson,
  doc,
  db::ThreadedDatabase,
};
use serde::Serialize;
use crate::database::UrmDb;
use crate::context::UrmInfo;
use super::structure::Product;

#[derive(Serialize)]
pub struct Context<'a> {
  pub urm: &'a UrmInfo,
  pub product: Option<Product>,
}

impl<'a> Context<'a> {
  pub fn from_db(urm_info: &'a UrmInfo, db: &'a UrmDb, pn: String)
    -> Result<Self, mongodb::error::Error>
  {
    let product: Option<Product> = match db.collection("products")
      .find_one(Some(doc!{ "pn": pn }), None)? {
        Some(p) => Some(Product::from(p)),
        None => None
      };

    Ok(Context {
      urm: &urm_info,
      product: product,
    })
  }
}
