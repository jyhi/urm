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
  pub fn from_db(db: &'a UrmDb, urm_info: &'a UrmInfo, page_info: &'a PageInfo)
    -> Result<Self, mongodb::error::Error>
  {
    let products = db.collection("products")
      .find(None, None)?
      .filter_map(|p| p.ok()) // XXX: TODO: Error handling
      .map(|p| Product::from(p))
      .collect();

    Ok(Context {
      urm: &urm_info,
      page: &page_info,
      products: products,
    })
  }
}
