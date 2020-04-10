use serde::Serialize;
use rocket_contrib::databases::mongodb;
use rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::database::UrmDb;
use crate::repository::Repository;
use crate::context::{UrmInfo, PageInfo};

#[derive(Serialize)]
pub struct Context<'a> {
  pub urm: &'a UrmInfo,
  pub page: PageInfo,
  pub repositories: Vec<Repository>,
}

impl<'a> Context<'a> {
  pub fn from_db(db: &'a UrmDb, urm_info: &'a UrmInfo, page: u64, nitem: u64)
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
      .filter_map(|p| p.ok()) // XXX: TODO: Error handling
      .map(|p| Repository::from(p))
      .collect();

    Ok(Context {
      urm: &urm_info,
      page: page_info,
      repositories: repositories,
    })
  }
}
