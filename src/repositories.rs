use rocket::State;
use rocket_contrib::json::JsonValue;
use rocket_contrib::templates::Template;
use serde::Serialize;
use crate::repository::Repository;
use crate::context::{UrmInfo, PageInfo, Tag};

#[derive(Serialize)]
pub struct RepositoriesContext<'a> {
  pub urm: &'a UrmInfo,
  pub page: &'a PageInfo,
  pub repositories: Vec<Repository>,
}

impl<'a> RepositoriesContext<'a> {
  fn test(urm_info: &'a UrmInfo, page_info: &'a PageInfo) -> Self {
    let repositories = vec![
      Repository {
        ln_p: "012345".to_string(),
        name: "Test Repository".to_string(),
        load: 42,
        tags: vec![
          Tag { name: "test repo".to_string() }
        ],
        has: None,
      }
    ];

    RepositoriesContext {
      urm: &urm_info,
      page: &page_info,
      repositories: repositories,
    }
  }
}

#[get("/repositories", format = "json")]
pub fn api(urm_info: State<UrmInfo>) -> JsonValue {
  let page_info = PageInfo { current: 1, min: 1, max: 1 };

  let ctx = RepositoriesContext::test(&urm_info, &page_info);
  json!(ctx)
}

#[get("/repositories", format = "html", rank = 1)]
pub fn ui(urm_info: State<UrmInfo>) -> Template {
  let page_info = PageInfo { current: 1, min: 1, max: 1 };

  let ctx = RepositoriesContext::test(&urm_info, &page_info);
  Template::render("repositories", ctx)
}
