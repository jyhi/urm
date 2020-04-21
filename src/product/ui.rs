use rocket_contrib::databases::mongodb::{
  self,
  bson,
  doc,
  db::ThreadedDatabase,
};
use serde::Serialize;
use crate::database::UrmDb;
use crate::config::UrmConfig;
use super::structure::Product;

#[derive(Serialize)]
pub struct Context<'a> {
  pub urm: &'a UrmConfig,
  pub product: Product,
}

impl<'a> Context<'a> {
  pub fn from_db(db: &'a UrmDb, config: &'a UrmConfig, pn: String)
    -> Result<Option<Self>, mongodb::Error>
  {
    match db.collection(&config.collection.products)
      .find_one(Some(doc!{ "pn": pn }), None)?
    {
      Some(doc) => Ok(Some(Context {
        urm: &config,
        product: Product::from(doc)
      })),
      None => Ok(None)
    }
  }
}
