use serde::Serialize;
use crate::repository::Repository;
use crate::context::Tag;

#[derive(Serialize)]
pub struct Context {
  pub repositories: Vec<Repository>,
}

impl Context {
  pub fn test() -> Self {
    let repositories = vec![
      Repository {
        ln_p: "012345".to_string(),
        name: "Test Repository".to_string(),
        load: 42,
        tags: vec![
          Tag { name: "test repo".to_string() }
        ],
        has: vec![],
        attributes: vec![],
      }
    ];

    Context {
      repositories: repositories,
    }
  }
}
