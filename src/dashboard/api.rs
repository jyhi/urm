use serde::Serialize;
use rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::database::UrmDb;
use crate::config::UrmConfig;

#[derive(Serialize)]
pub struct Context {
  pub nprod: u64,
}

impl Context {
  pub fn from_db(db: &UrmDb, config: &UrmConfig) -> Self {
    // TODO: Error handling
    // XXX: Why count returns Result<i64>?
    let nprod = db.collection(&config.collection.products)
      .count(None, None)
      .unwrap() as u64;

    Context {
      nprod: nprod,
    }
  }
}
