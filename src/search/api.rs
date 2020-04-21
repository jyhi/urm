use rocket_contrib::databases::mongodb::{
  self,
  Bson::RegExp,
  bson,
  doc,
  db::ThreadedDatabase,
};
use crate::database::UrmDb;
use super::SearchQuery;

pub fn from_db(db: &UrmDb, query: &SearchQuery)
  -> Result<Vec<mongodb::Document>, mongodb::Error>
{
  Ok(
    db.collection(&query.coll)
      .find(Some(doc!{ &query.k: RegExp(query.v.clone(), "i".to_string() )}), None)?
      .filter_map(|p| p.ok()) // XXX: TODO: Error handling
      .collect()
  )
}
