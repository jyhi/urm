use rocket_contrib::databases::mongodb;
use rocket_contrib::databases::mongodb::{
  bson,
  doc,
  db::ThreadedDatabase,
};
use serde::Serialize;
use crate::database::UrmDb;
use crate::context::UrmInfo;
use super::Repository;

#[derive(Serialize)]
pub struct Context<'a> {
  pub urm: &'a UrmInfo,
  pub repository: Repository,
}

impl<'a> Context<'a> {
  pub fn from_db(db: &UrmDb, urm_info: &'a UrmInfo, ln_p: String)
    -> Result<Option<Self>, mongodb::error::Error>
  {
    match db.collection("repositories")
      .find_one(Some(doc!{ "ln_p": ln_p }), None)?
    {
      Some(doc) => Ok(Some(Context {
        urm: &urm_info,
        repository: Repository::from(doc)
      })),
      None => Ok(None)
    }
  }
}
