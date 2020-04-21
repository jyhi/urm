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

pub use structure::Product;

pub struct PostedProduct(pub String);

impl FromDataSimple for PostedProduct {
  type Error = std::io::Error;

  fn from_data(_: &Request, data: Data) -> Outcome<Self, Self::Error> {
    let mut req_body_str = String::new();
    if let Err(e) = data.open().take(4096).read_to_string(&mut req_body_str) {
      return Outcome::Failure((Status::InternalServerError, e))
    }

    Outcome::Success(PostedProduct(req_body_str))
  }
}

#[post("/product", format = "json", data = "<product>")]
pub fn api_create(config: State<UrmConfig>, db: UrmDb, cred: UrmAuth, product: PostedProduct)
  -> Result<Status, mongodb::Error>
{
  match auth::check_db(&db, &config, &cred)? {
    Some(_) => {
      api::to_db(&db, &config, serde_json::from_str(&product.0).unwrap())?;
      Ok(Status::Created)
    }
    None => {
      Ok(Status::Unauthorized)
    }
  }
}

pub struct PatchedField(pub String);

impl FromDataSimple for PatchedField {
  type Error = std::io::Error;

  fn from_data(_: &Request, data: Data) -> Outcome<Self, Self::Error> {
    let mut req_body_str = String::new();
    if let Err(e) = data.open().take(4096).read_to_string(&mut req_body_str) {
      return Outcome::Failure((Status::InternalServerError, e))
    }

    Outcome::Success(PatchedField(req_body_str))
  }
}

#[patch("/product/<pn>", format = "json", data = "<field>")]
pub fn api_set_field(config: State<UrmConfig>, db: UrmDb, cred: UrmAuth, pn: String, field: PatchedField)
  -> Result<Status, mongodb::Error>
{
  match auth::check_db(&db, &config, &cred)? {
    Some(_) => {
      api::update_db(&db, &config, &pn, serde_json::from_str(&field.0).unwrap())?;
      Ok(Status::NoContent)
    }
    None => {
      Ok(Status::Unauthorized)
    }
  }
}

pub struct PutProduct(pub String);

impl FromDataSimple for PutProduct {
  type Error = std::io::Error;

  fn from_data(_: &Request, data: Data) -> Outcome<Self, Self::Error> {
    let mut req_body_str = String::new();
    if let Err(e) = data.open().take(4096).read_to_string(&mut req_body_str) {
      return Outcome::Failure((Status::InternalServerError, e))
    }

    Outcome::Success(PutProduct(req_body_str))
  }
}

#[put("/product/<pn>", format = "json", data = "<product>")]
pub fn api_replace(config: State<UrmConfig>, db: UrmDb, cred: UrmAuth, pn: String, product: PutProduct)
  -> Result<Status, mongodb::Error>
{
  match auth::check_db(&db, &config, &cred)? {
    Some(_) => {
      api::replace_to_db(&db, &config, &pn, serde_json::from_str(&product.0).unwrap())?;
      Ok(Status::NoContent)
    }
    None => {
      Ok(Status::Unauthorized)
    }
  }
}


#[delete("/product/<pn>")]
pub fn api_remove(config: State<UrmConfig>, db: UrmDb, cred: UrmAuth, pn: String)
  -> Result<Status, mongodb::Error>
{
  match auth::check_db(&db, &config, &cred)? {
    Some(_) => {
      api::delete_from_db(&db, &config, &pn)?;
      Ok(Status::NoContent)
    }
    None => {
      Ok(Status::Unauthorized)
    }
  }
}

#[get("/product/<pn>", format = "json")]
pub fn api(config: State<UrmConfig>, db: UrmDb, pn: String)
  -> Result<Option<Json<mongodb::Document>>, Json<mongodb::Error>>
{
  match api::from_db(&db, &config, pn) {
    Ok(r) => match r {
      Some(doc) => Ok(Some(Json(doc))),
      None => Ok(None)
    },
    Err(e) => Err(Json(e))
  }
}

#[get("/product/<pn>", format = "html", rank = 1)]
pub fn ui(config: State<UrmConfig>, db: UrmDb, pn: String)
  -> Result<Option<Template>, mongodb::Error>
{
  match ui::Context::from_db(&db, &config, pn) {
    Ok(r) => match r {
      Some(ctx) => Ok(Some(Template::render("product", ctx))),
      None => Ok(None)
    }
    Err(e) => Err(e)
  }
}
