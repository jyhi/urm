use rocket_contrib::databases::mongodb;
use rocket_contrib::databases::mongodb::{
  bson,
  doc,
  db::ThreadedDatabase,
};
use serde::Serialize;
use crate::database::UrmDb;
use crate::config::UrmConfig;
use crate::context::PageInfo;
use crate::product::Product;
use super::Repository;

#[derive(Serialize)]
pub struct Context<'a> {
  pub urm: &'a UrmConfig,
  pub page: PageInfo,
  pub repository: Repository,
}

impl<'a> Context<'a> {
  pub fn from_db(db: &UrmDb, config: &'a UrmConfig, ln_p: String, page: u64, nitem: u64)
    -> Result<Option<Self>, mongodb::error::Error>
  {
    let nprod = db.collection(&config.collection.products).count(None, None)? as u64;
    let nskip = (page - 1) * nitem;
    let page_info = PageInfo {
      current: page,
      min: 1,
      max: nprod / (nitem + 1) + 1
    };

    let mut ctx = match db.collection(&config.collection.repositories)
      .find_one(Some(doc!{ "ln_p": ln_p }), None)?
    {
      Some(doc) => Some(Context {
        urm: &config,
        page: page_info,
        repository: Repository::from(doc)
      }),
      None => None
    };

    if let Some(ref mut ctx) = ctx {
      // Override Repository::load with the real number
      ctx.repository.load = db.collection(&config.collection.products)
        .count(Some(doc!{ "in": &ctx.repository.ln_p }), None)? as u64;

      // Override Repository::has with the real list
      ctx.repository.has = db.collection(&config.collection.products)
        .find(Some(doc!{ "in": &ctx.repository.ln_p }), None)?
        .skip(nskip as usize)
        .take(nitem as usize)
        .map(|doc| Product::from(doc.unwrap()))
        .collect();
    };

    Ok(ctx)
  }
}
