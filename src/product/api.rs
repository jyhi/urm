use rocket_contrib::databases::mongodb::{
  self,
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

pub fn to_db(db: &UrmDb, config: &UrmConfig, doc: mongodb::Document)
  -> Result<(), mongodb::Error>
{
  match db.collection(&config.collection.products).insert_one(doc, None) {
    Ok(_) => Ok(()),
    Err(e) => Err(e)
  }
}

pub fn update_db(db: &UrmDb, config: &UrmConfig, pn: &str, field: mongodb::Document)
  -> Result<(), mongodb::Error>
{
  match db.collection(&config.collection.products)
    .update_one(doc!{ "pn": pn }, doc!{ "$set": field }, None)
  {
    Ok(_) => Ok(()),
    Err(e) => Err(e)
  }
}

pub fn replace_to_db(db: &UrmDb, config: &UrmConfig, pn: &str, doc: mongodb::Document)
  -> Result<(), mongodb::Error>
{
  match db.collection(&config.collection.products)
    .replace_one(doc!{ "pn": pn }, doc, None)
  {
    Ok(_) => Ok(()),
    Err(e) => Err(e)
  }
}

pub fn delete_from_db(db: &UrmDb, config: &UrmConfig, pn: &str)
  -> Result<(), mongodb::Error>
{
  match db.collection(&config.collection.products)
    .delete_one(doc!{ "pn": pn }, None)
  {
    Ok(_) => Ok(()),
    Err(e) => Err(e)
  }
}
