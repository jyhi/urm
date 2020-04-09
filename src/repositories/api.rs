use serde::Serialize;
use rocket_contrib::databases::mongodb;
use rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use crate::database::UrmDb;
use crate::repository::Repository;

#[derive(Serialize)]
pub struct Context {
  pub repositories: Vec<Repository>,
}

impl Context {
  pub fn from_db(db: &UrmDb)
    -> Result<Self, mongodb::error::Error>
  {
    let repositories: Vec<Repository> = db.collection("repositories")
      .find(None, None)?
      .map(|r| Repository::from(r.unwrap_or(Default::default()))) // XXX: TODO: Error handling
      .collect();

    Ok(Context{
      repositories: repositories
    })
  }
}
