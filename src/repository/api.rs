use rocket_contrib::databases::mongodb;
use rocket_contrib::databases::mongodb::{
  bson,
  doc,
  db::ThreadedDatabase,
};
use crate::database::UrmDb;

pub fn from_db(db: &UrmDb, ln_p: String)
  -> Result<Option<mongodb::Document>, mongodb::error::Error>
{
  match db.collection("repositories")
    .find_one(Some(doc!{ "ln_p": ln_p }), None)?
  {
    Some(doc) => Ok(Some(doc)),
    None => Ok(None)
  }
}
