use serde::Serialize;
use rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::database::UrmDb;
use crate::config::UrmConfig;

#[derive(Serialize)]
pub struct Context<'a> {
  pub urm: &'a UrmConfig,
  pub nprod: u64,
}

impl<'a> Context<'a> {
  pub fn from_db(config: &'a UrmConfig, db: &'a UrmDb) -> Self {
    // TODO: Error handling
    // XXX: Why count returns Result<i64>?
    let nprod = db.collection("products")
      .count(None, None)
      .unwrap() as u64;

    Context {
      urm: config,
      nprod: nprod,
    }
  }
}
