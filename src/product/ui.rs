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
  pub product: Product,
}

impl<'a> Context<'a> {
  pub fn from_db(db: &'a UrmDb, urm_info: &'a UrmInfo, pn: String)
    -> Result<Option<Self>, mongodb::error::Error>
  {
    match db.collection("products").find_one(Some(doc!{ "pn": pn }), None)? {
      Some(doc) => Ok(Some(Context {
        urm: &urm_info,
        product: Product::from(doc)
      })),
      None => Ok(None)
    }
  }
}
