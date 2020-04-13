use serde::Serialize;
use rocket_contrib::databases::mongodb;
use rocket_contrib::databases::mongodb::{
  Bson::RegExp,
  bson,
  doc,
  db::ThreadedDatabase,
};
use crate::database::UrmDb;
use crate::context::PageInfo;
use crate::config::UrmConfig;

#[derive(Default, Serialize)]
pub struct SearchResult {
  pub n: String,
  pub name: String,
  pub value: String,
}

impl From<mongodb::Document> for SearchResult {
  fn from(doc: mongodb::Document) -> Self {
    doc.iter().fold(Default::default(), |mut r, f| {
      // r: SearchResult, f: (&String, &Bson)
      match f.0.as_str() {
        "pn" | "ln_p" => {
          r.n = f.1.as_str().unwrap_or("Unknown").to_string()
        }
        "name" => {
          r.name = f.1.as_str().unwrap_or("Unknown").to_string()
        }
        // NOTE: SearchResult::value cannot be assigned here, since we can't get
        // the key in From::from()...
        _ => {} // Do nothing on fields that we don't care
      }

      r
    })
  }
}

#[derive(Serialize)]
pub struct SearchInfo {
  pub key: String,
  pub base_path: String,
  pub results: Vec<SearchResult>,
}

#[derive(Serialize)]
pub struct Context<'a> {
  pub urm: &'a UrmConfig,
  pub page: PageInfo,
  pub search: SearchInfo,
}

impl<'a> Context<'a> {
  pub fn from_db(db: &UrmDb, config: &'a UrmConfig, k: &String, _op: &String, v: &String, coll: &String, page: u64, nitem: u64)
    -> Result<Self, mongodb::Error>
  {
    let nresult = db.collection(coll)
      .count(Some(doc!{ k: RegExp(v.clone(), "i".to_string() )}), None)? as u64;
    let nskip = (page - 1) * nitem;
    let page_info = PageInfo {
      current: page,
      min: 1,
      max: nresult / (nitem + 1) + 1
    };

    let base_path = if coll == &config.collection.products {
      "/product".to_string()
    } else if coll == &config.collection.repositories {
      "/repository".to_string()
    } else {
      "".to_string() // Don't care
    };

    let results = db.collection(coll)
      .find(Some(doc!{ k: RegExp(v.clone(), "i".to_string() )}), None)?
      .skip(nskip as usize)
      .take(nitem as usize)
      .filter_map(|rdoc| rdoc.ok()) // XXX: TODO: Error handling
      .map(|rdoc| {
        let value = rdoc.get(k).unwrap().to_string();
        let mut r = SearchResult::from(rdoc);
        r.value = value;
        r
      })
      .collect();

    let search_info = SearchInfo {
      key: k.clone(),
      base_path: base_path,
      results: results,
    };

    Ok(Context {
      urm: &config,
      page: page_info,
      search: search_info,
    })
  }
}
