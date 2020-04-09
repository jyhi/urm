use serde::Serialize;
use crate::product::Product;
use crate::repository::Repository;
use crate::context::{Tag, Attribute};

#[derive(Serialize)]
pub struct Context {
  pub products: Vec<Product>,
}

impl Context {
  pub fn test() -> Self {
    let products = vec![
      Product {
        pn: "012345".to_string(),
        name: "Epic Bacon".to_string(),
        amount: 42,
        r#in: Repository { ln_p: "Z12345678".to_string(), name: "Test Repository".to_string(), load: 42, tags: vec![], has: None },
        on: "Y12345678".to_string(),
        tags: vec![
          Tag { name: "test product".to_string() },
        ],
        attributes: vec![
          Attribute { key: "testattrkey".to_string(), value: "Test attribute value".to_string() },
        ],
      },
    ];

    Context {
      products: products,
    }
  }
}
