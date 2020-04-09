use serde::Serialize;
use rocket_contrib::databases::mongodb;
use rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::database::UrmDb;
use crate::product::Product;
use crate::context::{UrmInfo, PageInfo};

#[derive(Serialize)]
pub struct Context<'a> {
  pub urm: &'a UrmInfo,
  pub page: &'a PageInfo,
  pub products: Vec<Product>,
}

impl<'a> Context<'a> {
  pub fn from_db(urm_info: &'a UrmInfo, page_info: &'a PageInfo, db: &'a UrmDb) -> Result<Self, mongodb::error::Error> {
    let products: Vec<Product> = db.collection("products")
      .find(None, None)?
      .map(|p| Product::from(p.unwrap_or(Default::default()))) // XXX: TODO: Error handling
      .collect();

    Ok(Context {
      urm: &urm_info,
      page: &page_info,
      products: products,
    })
  }
}
