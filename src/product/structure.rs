use serde::Serialize;
use rocket_contrib::databases::mongodb;
use crate::context::{Tag, Attribute};

#[derive(Serialize)]
pub struct Product {
  pub pn: String,
  pub name: String,
  pub amount: f64,
  pub r#in: String,
  pub on: String,
  pub tags: Vec<Tag>,
  pub attributes: Vec<Attribute>,
}

impl Default for Product {
  fn default() -> Self {
    Product {
      pn: "Unknown".to_string(),
      name: "Unknown".to_string(),
      amount: 0.0,
      r#in: "Unknown".to_string(),
      on: "Unknown".to_string(),
      tags: vec![],
      attributes: vec![],
    }
  }
}

impl From<mongodb::Document> for Product {
  fn from(doc: mongodb::Document) -> Self {
    doc.iter().fold(Default::default(), |mut p, f| {
      // p: Product, f: (&'a String, &'a Bson)
      match f.0.as_str() {
        "pn" => {
          p.pn = f.1.as_str().unwrap_or("Unknown").to_string()
        }
        "name" => {
          p.name = f.1.as_str().unwrap_or("Unknown").to_string()
        }
        "amount" => {
          // XXX: amount should be in u64, however:
          //
          // 1. The MongoDB driver treats all numbers as i64;
          // 2. Serde deserializes only negative numbers to i64;
          //
          // ... as a workaround, all numbers are stored as f64, and f64 display
          // is also supported (although it looks quite weird).
          p.amount = f.1.as_f64().unwrap_or(0.0)
        }
        "in" => {
          p.r#in = f.1.as_str().unwrap_or("Unknown").to_string()
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
