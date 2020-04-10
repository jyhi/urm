use rocket_contrib::databases::mongodb;
use rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::database::UrmDb;

pub fn from_db(db: &UrmDb)
  -> Result<Vec<mongodb::Document>, mongodb::error::Error>
{
  Ok(
    db.collection("products")
      .find(None, None)?
      .filter_map(|p| p.ok()) // XXX: TODO: Error handling
      .collect()
  )
}
