use serde::Serialize;
use rocket_contrib::databases::mongodb;
use rocket_contrib::databases::mongodb::{
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
  pub repositories: Vec<Repository>,
}

impl<'a> Context<'a> {
  pub fn from_db(db: &'a UrmDb, config: &'a UrmConfig, page: u64, nitem: u64)
    -> Result<Self, mongodb::error::Error>
  {
    let nprod = db.collection("products").count(None, None)? as u64;
    let nskip = (page - 1) * nitem;
    let page_info = PageInfo {
      current: page,
      min: 1,
      max: nprod / (nitem + 1) + 1
    };

    let repositories = db.collection("repositories")
      .find(None, None)?
      .skip(nskip as usize)
      .take(nitem as usize)
      .filter_map(|r| r.ok()) // XXX: TODO: Error handling
      .map(|r| Repository::from(r))
      .map(|mut r| {
        r.load = db.collection("products")
          .count(Some(doc!{ "in": &r.ln_p }), None)
          .unwrap_or(0) as u64;
        r
      })
      .collect();

    Ok(Context {
      urm: &config,
      page: page_info,
      repositories: repositories,
    })
  }
}
