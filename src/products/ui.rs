use serde::Serialize;
use crate::product::Product;
use crate::repository::Repository;
use crate::context::{UrmInfo, PageInfo, Tag, Attribute};

#[derive(Serialize)]
pub struct Context<'a> {
  pub urm: &'a UrmInfo,
  pub page: &'a PageInfo,
  pub products: Vec<Product>,
}

impl<'a> Context<'a> {
  pub fn test(urm_info: &'a UrmInfo, page_info: &'a PageInfo) -> Self {
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
      urm: &urm_info,
      page: &page_info,
      products: products,
    }
  }
}
