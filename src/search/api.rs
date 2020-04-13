use rocket_contrib::databases::mongodb;
use rocket_contrib::databases::mongodb::{
  Bson::RegExp,
  bson,
  doc,
  db::ThreadedDatabase,
};
use crate::database::UrmDb;

pub fn from_db(db: &UrmDb, k: &String, _op: &String, v: &String, coll: &String, page: u64, nitem: u64)
  -> Result<Vec<mongodb::Document>, mongodb::Error>
{
  let nskip = (page - 1) * nitem;

  Ok(
    db.collection(coll)
      .find(Some(doc!{ k: RegExp(v.clone(), "i".to_string() )}), None)?
      .skip(nskip as usize)
      .take(nitem as usize)
      .filter_map(|p| p.ok()) // XXX: TODO: Error handling
      .collect()
  )
}
