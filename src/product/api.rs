use rocket_contrib::databases::mongodb;
use rocket_contrib::databases::mongodb::{
  bson,
  doc,
  db::ThreadedDatabase,
};
use crate::database::UrmDb;
use crate::config::UrmConfig;

pub fn from_db(db: &UrmDb, config: &UrmConfig, pn: String)
  -> Result<Option<mongodb::Document>, mongodb::Error>
{
  match db.collection(&config.collection.products)
    .find_one(Some(doc!{ "pn": pn }), None)?
  {
    Some(doc) => Ok(Some(doc)),
    None => Ok(None)
  }
}
