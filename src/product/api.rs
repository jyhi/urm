use rocket_contrib::databases::mongodb;
use rocket_contrib::databases::mongodb::{
  bson,
  doc,
  db::ThreadedDatabase,
};
use crate::database::UrmDb;

pub fn from_db(db: &UrmDb, pn: String)
  -> Result<Option<mongodb::Document>, mongodb::error::Error>
{
  match db.collection("products").find_one(Some(doc!{ "pn": pn }), None)? {
    Some(document) => Ok(Some(document)),
    None => Ok(None)
  }
}
