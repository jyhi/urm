mod structure;
mod api;
mod ui;

use std::io::Read;
use rocket::State;
use rocket::Data;
use rocket::Request;
use rocket::http::Status;
use rocket::data::{FromDataSimple, Outcome};
use rocket_contrib::json::Json;
use rocket_contrib::databases::mongodb;
use rocket_contrib::templates::Template;
use crate::auth::{self, UrmAuth};
use crate::database::UrmDb;
use crate::config::UrmConfig;

pub use structure::Repository;

pub struct PostedRepository(pub String);

impl FromDataSimple for PostedRepository {
  type Error = std::io::Error;

  fn from_data(_: &Request, data: Data) -> Outcome<Self, Self::Error> {
    let mut req_body_str = String::new();
    if let Err(e) = data.open().take(4096).read_to_string(&mut req_body_str) {
      return Outcome::Failure((Status::InternalServerError, e))
    }

    Outcome::Success(PostedRepository(req_body_str))
  }
}

#[post("/repository", format = "json", data = "<repository>")]
pub fn api_create(config: State<UrmConfig>, db: UrmDb, cred: UrmAuth, repository: PostedRepository)
  -> Result<Status, mongodb::Error>
{
  match auth::check_db(&db, &config, &cred)? {
    Some(_) => {
      api::to_db(&db, &config, serde_json::from_str(&repository.0).unwrap())?;
      Ok(Status::Created)
    }
    None => {
      Ok(Status::Unauthorized)
    }
  }
}

#[get("/repository/<ln_p>", format = "json")]
pub fn api(config: State<UrmConfig>, db: UrmDb, ln_p: String)
  -> Result<Option<Json<mongodb::Document>>, Json<mongodb::Error>>
{
  match api::from_db(&db, &config, ln_p) {
    Ok(r) => match r {
      Some(doc) => Ok(Some(Json(doc))),
      None => Ok(None)
    },
    Err(e) => Err(Json(e))
  }
}

#[get("/repository/<ln_p>", format = "html", rank = 1)]
pub fn ui(config: State<UrmConfig>, db: UrmDb, ln_p: String)
  -> Result<Option<Template>, mongodb::Error>
{
  match ui::Context::from_db(&db, &config, ln_p) {
    Ok(r) => match r {
      Some(ctx) => Ok(Some(Template::render("repository", ctx))),
      None => Ok(None)
    }
    Err(e) => Err(e)
  }
}
