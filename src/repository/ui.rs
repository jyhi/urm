use serde::Serialize;
use crate::product::Product;
use crate::context::{UrmInfo, PageInfo, Tag, Attribute};
use super::Repository;

#[derive(Serialize)]
pub struct Context<'a> {
  pub urm: &'a UrmInfo,
  pub page: &'a PageInfo,
  pub repository: Repository,
}

impl<'a> Context<'a> {
  pub fn test(urm_info: &'a UrmInfo, page_info: &'a PageInfo, ln_p: String) -> Self {
    let repository = Repository {
      ln_p: ln_p,
      name: "Test Repository".to_string(),
      load: 42,
      tags: vec![
        Tag { name: "test repo".to_string() }
      ],
      has: Some(vec![
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
      ]),
    };

    Context {
      urm: &urm_info,
      page: &page_info,
      repository: repository,
    }
  }
}
