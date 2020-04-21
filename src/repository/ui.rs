use rocket_contrib::databases::mongodb::{
  self,
  bson,
  doc,
  db::ThreadedDatabase,
};
use serde::Serialize;
use crate::database::UrmDb;
use crate::config::UrmConfig;
use super::Repository;

#[derive(Serialize)]
pub struct Context<'a> {
  pub urm: &'a UrmConfig,
  pub repository: Repository,
}

impl<'a> Context<'a> {
  pub fn from_db(db: &UrmDb, config: &'a UrmConfig, ln_p: String)
    -> Result<Option<Self>, mongodb::Error>
  {
    let mut ctx = match db.collection(&config.collection.repositories)
      .find_one(Some(doc!{ "ln_p": ln_p }), None)?
    {
      Some(doc) => Some(Context {
        urm: &config,
        repository: Repository::from(doc)
      }),
      None => None
    };

    // Override Repository::load with the real number
    if let Some(ref mut ctx) = ctx {
      ctx.repository.load = db.collection(&config.collection.products)
        .count(Some(doc!{ "in": &ctx.repository.ln_p }), None)? as u64;
    };

    Ok(ctx)
  }
}
