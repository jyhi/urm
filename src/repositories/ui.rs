use serde::Serialize;
use crate::repository::Repository;
use crate::context::{UrmInfo, PageInfo, Tag};

#[derive(Serialize)]
pub struct Context<'a> {
  pub urm: &'a UrmInfo,
  pub page: &'a PageInfo,
  pub repositories: Vec<Repository>,
}

impl<'a> Context<'a> {
  pub fn test(urm_info: &'a UrmInfo, page_info: &'a PageInfo) -> Self {
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

    Context {
      urm: &urm_info,
      page: &page_info,
      repositories: repositories,
    }
  }
}
