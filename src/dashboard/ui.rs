use serde::Serialize;
use rocket_contrib::databases::mongodb;
use rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::database::UrmDb;
use crate::config::UrmConfig;
use crate::context::SearchInfo;

#[derive(Serialize)]
pub struct Context<'a> {
  pub urm: &'a UrmConfig,
  pub search: SearchInfo,
  pub nprod: u64,
  pub nrepo: u64,
}

impl<'a> Context<'a> {
  pub fn from_db(db: &'a UrmDb, config: &'a UrmConfig)
    -> Result<Self, mongodb::Error>
  {
    let ops = ["matches"].iter()
      .map(|op| op.to_string())
      .collect();
    let collections = vec![
      config.collection.products.clone(),
      config.collection.repositories.clone()
    ];

    let search_info = SearchInfo {
      ops: ops,
      collections: collections,
    };

    let nprod = db.collection(&config.collection.products)
      .count(None, None)? as u64;
    let nrepo = db.collection(&config.collection.repositories)
      .count(None, None)? as u64;

    Ok(Context {
      urm: config,
      search: search_info,
      nprod: nprod,
      nrepo: nrepo,
    })
  }
}
