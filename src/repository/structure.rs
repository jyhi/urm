use serde::Serialize;
use rocket_contrib::databases::mongodb;
use crate::product::Product;
use crate::context::{Tag, Attribute};

#[derive(Default, Serialize)]
pub struct Repository {
  pub ln_p: String,
  pub name: String,
  pub load: u64,
  pub tags: Vec<Tag>,
  pub attributes: Vec<Attribute>,
  pub has: Vec<Product>,
}

impl From<mongodb::Document> for Repository {
  fn from(document: mongodb::Document) -> Self {
    document.iter().fold(Default::default(), |mut r, f| {
      // r: Repository, f: (&String, &Bson)
      match f.0.as_str() {
        "ln_p" => {
          f.1.as_str().unwrap_or("Unknown").to_string();
        }
        "name" => {
          f.1.as_str().unwrap_or("Unknown").to_string();
        }
        "load" => {
          f.1.as_i64().unwrap_or(0) as u64;
        }
        // Note: Repository::has is not supposed to be filled here.
        _ => {
          r.attributes.push(
            Attribute {
              key: f.1.to_string(),
              value: f.1.to_string(),
            }
          );
        }
      };

      r
    })
  }
}
