use serde::Serialize;
use rocket_contrib::databases::mongodb;
use crate::product::Product;
use crate::context::{Tag, Attribute};

#[derive(Serialize)]
pub struct Repository {
  pub ln_p: String,
  pub name: String,
  pub load: u64,
  pub tags: Vec<Tag>,
  pub attributes: Vec<Attribute>,
  pub has: Vec<Product>,
}

impl Default for Repository {
  fn default() -> Self {
    Repository {
      ln_p: "Unknown".to_string(),
      name: "Unknown".to_string(),
      load: 0,
      tags: vec![],
      attributes: vec![],
      has: vec![],
    }
  }
}

impl From<mongodb::Document> for Repository {
  fn from(doc: mongodb::Document) -> Self {
    doc.iter().fold(Default::default(), |mut r, f| {
      // r: Repository, f: (&String, &Bson)
      match f.0.as_str() {
        "ln_p" => {
          r.ln_p = f.1.as_str().unwrap_or("Unknown").to_string();
        }
        "name" => {
          r.name = f.1.as_str().unwrap_or("Unknown").to_string();
        }
        "load" => {
          r.load = f.1.as_i64().unwrap_or(0) as u64;
        }
        // Note: Repository::has is not supposed to be filled here, but we also
        // need to ensure that if it is accidently added into the database it's
        // not treated as an attribute.
        "has" => {}
        _ => {
          r.attributes.push(
            Attribute {
              key: f.0.to_string(),
              value: f.1.to_string(),
            }
          );
        }
      };

      r
    })
  }
}
