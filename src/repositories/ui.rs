use serde::Serialize;
use rocket_contrib::databases::mongodb::{
  self,
  bson,
  doc,
  db::ThreadedDatabase,
};
use crate::database::UrmDb;
use crate::repository::Repository;
use crate::context::PageInfo;
use crate::config::UrmConfig;

#[derive(Serialize)]
pub struct Context<'a> {
  pub urm: &'a UrmConfig,
  pub page: PageInfo,
  pub nrepo: u64,
  pub repositories: Vec<Repository>,
}

impl<'a> Context<'a> {
  pub fn from_db(db: &'a UrmDb, config: &'a UrmConfig, page: u64, nitem: u64)
    -> Result<Self, mongodb::Error>
  {
    let nrepo = db.collection(&config.collection.repositories).count(None, None)? as u64;
    let nskip = (page - 1) * nitem;
    let page_info = PageInfo {
      current: page,
      min: 1,
      max: (nrepo - 1) / nitem + 1
    };

    let mut repositories: Vec<Repository> = db
      .collection(&config.collection.repositories)
      .find(None, None)?
      .skip(nskip as usize)
      .take(nitem as usize)
      .filter_map(|r| r.ok()) // XXX: TODO: Error handling
      .map(|r| Repository::from(r))
      .collect();

    for mut r in &mut repositories {
      r.load = db.collection(&config.collection.products)
        .count(Some(doc!{ "in": &r.ln_p }), None)? as u64;
    }

    Ok(Context {
      urm: &config,
      page: page_info,
      nrepo: nrepo,
      repositories: repositories,
    })
  }
}
