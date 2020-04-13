use serde::Serialize;
use rocket_contrib::databases::mongodb;
use rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::database::UrmDb;
use crate::product::Product;
use crate::context::PageInfo;
use crate::config::UrmConfig;

#[derive(Serialize)]
pub struct Context<'a> {
  pub urm: &'a UrmConfig,
  pub page: PageInfo,
  pub products: Vec<Product>,
}

impl<'a> Context<'a> {
  pub fn from_db(db: &'a UrmDb, config: &'a UrmConfig, page: u64, nitem: u64)
    -> Result<Self, mongodb::Error>
  {
    let nprod = db.collection(&config.collection.products).count(None, None)? as u64;
    let nskip = (page - 1) * nitem;
    let page_info = PageInfo {
      current: page,
      min: 1,
      max: nprod / (nitem + 1) + 1
    };

    let products = db.collection(&config.collection.products)
      .find(None, None)?
      .skip(nskip as usize)
      .take(nitem as usize)
      .filter_map(|p| p.ok()) // XXX: TODO: Error handling
      .map(|p| Product::from(p))
      .collect();

    Ok(Context {
      urm: &config,
      page: page_info,
      products: products,
    })
  }
}
