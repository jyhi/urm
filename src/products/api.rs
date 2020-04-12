use rocket_contrib::databases::mongodb;
use rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::database::UrmDb;
use crate::config::UrmConfig;

pub fn from_db(db: &UrmDb, config: &UrmConfig, page: u64, nitem: u64)
  -> Result<Vec<mongodb::Document>, mongodb::error::Error>
{
  let nskip = (page - 1) * nitem;

  Ok(
    db.collection(&config.collection.products)
      .find(None, None)?
      .skip(nskip as usize)
      .take(nitem as usize)
      .filter_map(|p| p.ok()) // XXX: TODO: Error handling
      .collect()
  )
}
