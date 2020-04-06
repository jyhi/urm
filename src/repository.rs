use rocket::State;
use rocket_contrib::json::JsonValue;
use rocket_contrib::templates::Template;
use serde::Serialize;
use crate::product::Product;
use crate::context::{UrmInfo, PageInfo, Tag, Attribute};

#[derive(Serialize)]
pub struct Repository {
  pub ln_p: String,
  pub name: String,
  pub load: u64,
  pub tags: Vec<Tag>,
  pub has: Option<Vec<Product>>,
}

#[derive(Serialize)]
pub struct RepositoryContext<'a> {
  urm: &'a UrmInfo,
  pub page: &'a PageInfo,
  repository: Repository,
}

impl<'a> RepositoryContext<'a> {
  fn test(urm_info: &'a UrmInfo, page_info: &'a PageInfo, ln_p: String) -> Self {
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

    RepositoryContext {
      urm: &urm_info,
      page: &page_info,
      repository: repository,
    }
  }
}

#[get("/repository/<ln_p>", format = "json")]
pub fn api(ln_p: String) -> JsonValue {
  json!({
    "error": true,
    "desc": "Not implemented",
    "L/N-P": ln_p,
  })
}

#[get("/repository/<ln_p>", format = "html", rank = 1)]
pub fn ui(urm_info: State<UrmInfo>, ln_p: String) -> Template {
  let page_info = PageInfo { current: 1, min: 1, max: 1 };

  let ctx = RepositoryContext::test(&urm_info, &page_info, ln_p.clone());
  Template::render("repository", ctx)
}
