use rocket_contrib::databases::mongodb::{
  self,
  bson,
  doc,
  db::ThreadedDatabase,
};
use crate::database::UrmDb;
use crate::config::UrmConfig;

pub fn from_db(db: &UrmDb, config: &UrmConfig, ln_p: String)
  -> Result<Option<mongodb::Document>, mongodb::Error>
{
  match db.collection(&config.collection.repositories)
    .find_one(Some(doc!{ "ln_p": ln_p }), None)?
  {
    Some(doc) => Ok(Some(doc)),
    None => Ok(None)
  }
}

pub fn to_db(db: &UrmDb, config: &UrmConfig, doc: mongodb::Document)
  -> Result<(), mongodb::Error>
{
  match db.collection(&config.collection.repositories).insert_one(doc, None) {
    Ok(_) => Ok(()),
    Err(e) => Err(e)
  }
}

pub fn update_db(db: &UrmDb, config: &UrmConfig, ln_p: &str, field: mongodb::Document)
  -> Result<(), mongodb::Error>
{
  match db.collection(&config.collection.repositories)
    .update_one(doc!{ "ln_p": ln_p }, doc!{ "$set": field }, None)
  {
    Ok(_) => Ok(()),
    Err(e) => Err(e)
  }
}

pub fn delete_from_db(db: &UrmDb, config: &UrmConfig, ln_p: &str)
  -> Result<(), mongodb::Error>
{
  match db.collection(&config.collection.repositories)
    .delete_one(doc!{ "ln_p": ln_p }, None)
  {
    Ok(_) => Ok(()),
    Err(e) => Err(e)
  }
}
