use rocket_contrib::databases::mongodb;
use rocket_contrib::databases::mongodb::{
  bson,
  doc,
  db::ThreadedDatabase,
};
use serde::Serialize;
use crate::database::UrmDb;
use super::Repository;

#[derive(Serialize)]
pub struct Context {
  pub repository: Option<Repository>,
}

impl Context {
  pub fn from_db(db: &UrmDb, ln_p: String)
    -> Result<Self, mongodb::error::Error>
  {
    let repository: Option<Repository> = match db.collection("repositories")
      .find_one(Some(doc!{ "ln_p": ln_p }), None)? {
        Some(doc) => Some(Repository::from(doc)),
        None => None
      };

    Ok(Context {
      repository: repository,
    })
  }
}
