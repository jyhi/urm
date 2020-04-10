use rocket_contrib::databases::mongodb;
use rocket_contrib::databases::mongodb::{
  bson,
  doc,
  db::ThreadedDatabase,
};
use serde::Serialize;
use crate::database::UrmDb;
use crate::context::{UrmInfo, PageInfo};
use super::Repository;

#[derive(Serialize)]
pub struct Context<'a> {
  pub urm: &'a UrmInfo,
  pub page: &'a PageInfo,
  pub repository: Repository,
}

impl<'a> Context<'a> {
  pub fn from_db(urm_info: &'a UrmInfo, page_info: &'a PageInfo, db: &UrmDb, ln_p: String)
    -> Result<Option<Self>, mongodb::error::Error>
  {
    match db.collection("repositories")
      .find_one(Some(doc!{ "ln_p": ln_p }), None)?
    {
      Some(doc) => Ok(Some(Context {
        urm: &urm_info,
        page: &page_info,
        repository: Repository::from(doc)
      })),
      None => Ok(None)
    }
  }
}
