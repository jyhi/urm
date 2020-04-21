use serde::Serialize;
use rocket_contrib::databases::mongodb::{
  self,
  Bson::RegExp,
  bson,
  doc,
  db::ThreadedDatabase,
};
use crate::database::UrmDb;
use crate::config::UrmConfig;
use super::SearchQuery;

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
  pub search: SearchInfo,
}

impl<'a> Context<'a> {
  pub fn from_db(db: &UrmDb, config: &'a UrmConfig, query: &SearchQuery)
    -> Result<Self, mongodb::Error>
  {
    let base_path = if &query.coll == &config.collection.products {
      "/product".to_string()
    } else if &query.coll == &config.collection.repositories {
      "/repository".to_string()
    } else {
      "".to_string() // Don't care
    };

    let results = db.collection(&query.coll)
      .find(Some(doc!{ &query.k: RegExp(query.v.clone(), "i".to_string() )}), None)?
      .filter_map(|rdoc| rdoc.ok()) // XXX: TODO: Error handling
      .map(|rdoc| {
        let value = rdoc.get(&query.k).unwrap().to_string();
        let mut r = SearchResult::from(rdoc);
        r.value = value;
        r
      })
      .collect();

    let search_info = SearchInfo {
      key: query.k.clone(),
      base_path: base_path,
      results: results,
    };

    Ok(Context {
      urm: &config,
      search: search_info,
    })
  }
}
