use serde::Serialize;
use rocket_contrib::databases::mongodb;
use crate::repository::Repository;
use crate::context::{Tag, Attribute};

#[derive(Default, Serialize)]
pub struct Product {
  pub pn: String,
  pub name: String,
  pub amount: u64,
  pub r#in: Repository,
  pub on: String,
  pub tags: Vec<Tag>,
  pub attributes: Vec<Attribute>,
}

impl From<mongodb::Document> for Product {
  fn from(document: mongodb::Document) -> Self {
    document.iter().fold(Default::default(), |mut p, f| {
      // p: Product, f: (&'a String, &'a Bson)
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

      p
    })
  }
}
