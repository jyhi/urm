use serde::Serialize;
use rocket_contrib::databases::mongodb::{
  self,
  spec::ElementType,
};
use crate::context::{Tag, Attribute};

#[derive(Serialize)]
pub struct Product {
  pub pn: String,
  pub name: String,
  pub amount: String,
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
      amount: "Unknown".to_string(),
      r#in: "Unknown".to_string(),
      on: "Unknown".to_string(),
      tags: vec![],
      attributes: vec![],
    }
  }
}

impl From<mongodb::Document> for Product {
  fn from(doc: mongodb::Document) -> Self {
    doc.iter().fold(Product::default(), |mut p, f| {
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
          // 3. JavaScript does not allow trailing zeros after the decimal point;
          //
          // ... as a workaround, we allow strings be set in the database, and
          // present amount in string as well. It's insane, but anyway.
          p.amount = match f.1.element_type() {
            ElementType::Utf8String => {
              f.1.as_str().unwrap().to_owned()
            }
            _ => {
              f.1.to_string()
            }
          };
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
            p.tags = vec![];
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
