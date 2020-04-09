use rocket_contrib::databases::mongodb;
use rocket_contrib::databases::mongodb::{
  bson,
  doc,
  db::ThreadedDatabase,
};
use serde::Serialize;
use crate::database::UrmDb;
use crate::repository::Repository;
use crate::context::{UrmInfo, Tag, Attribute};
use super::Product;

#[derive(Serialize)]
pub struct Context<'a> {
  pub urm: &'a UrmInfo,
  pub product: Option<Product>,
}

impl<'a> Context<'a> {
  pub fn from_db(urm_info: &'a UrmInfo, db: &'a UrmDb, pn: String)
    -> Result<Self, mongodb::error::Error>
  {
    let product: Option<Product> = match db.collection("products")
      .find_one(Some(doc!{ "pn": pn }), None)? {
        Some(r) => {
          r.iter().fold(Some(Default::default()), |mut p, f| {
              // p: Option<Product>, f: (&'a String, &'a Bson)
              if let Some(ref mut p) = p {
                match f.0.as_str() {
                  "pn" => {
                    p.pn = f.1.as_str().unwrap_or("Unknown").to_string()
                  }
                  "name" => {
                    p.name = f.1.as_str().unwrap_or("Unknown").to_string()
                  }
                  "amount" => {
                    p.amount = f.1.as_i64().unwrap_or(0) as u64
                  }
                  "in" => {
                    if let Some(r) = f.1.as_document() {
                      p.r#in = Repository {
                        ln_p: r.get_str("ln_p").unwrap_or("Error").to_string(),
                        name: r.get_str("name").unwrap_or("Error").to_string(),
                        load: r.get_i64("load").unwrap_or(0) as u64,
                        tags: Default::default(), // TODO
                        has: None, // We don't care about it here
                      };
                    } else {
                      p.r#in = Default::default();
                    }
                  }
                  "on" => {
                    p.on = f.1.as_str().unwrap_or("Unknown").to_string()
                  }
                  "tags" => {
                    if let Some(ts) = f.1.as_array() {
                      p.tags = ts.iter()
                        .filter(|t| t.as_str().is_some())
                        .map(|t| Tag { name: t.as_str().unwrap_or("Unknown").to_string() })
                        .collect();
                    } else {
                      p.tags = Default::default();
                    }
                  }
                  _ => {
                    p.attributes.push(
                      Attribute {
                        key: f.0.to_string(),
                        value: f.1.to_string(),
                      }
                    )
                  }
                };
              };

              p
            })
        }
        None => None
      };

    Ok(Context {
      urm: &urm_info,
      product: product,
    })
  }
}
