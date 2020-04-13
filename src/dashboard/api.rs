use serde::Serialize;
use rocket_contrib::databases::mongodb;
use rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::database::UrmDb;
use crate::config::UrmConfig;

#[derive(Serialize)]
pub struct Context {
  pub nprod: u64,
}

impl Context {
  pub fn from_db(db: &UrmDb, config: &UrmConfig)
    -> Result<Self, mongodb::Error>
  {
    let nprod = db.collection(&config.collection.products)
      .count(None, None)? as u64;

    Ok(Context {
      nprod: nprod,
    })
  }
}
