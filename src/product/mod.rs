mod api;
mod ui;

use rocket::State;
use rocket::response::status::NotFound;
use rocket_contrib::json::JsonValue;
use rocket_contrib::templates::Template;
use serde::Serialize;
use crate::database::UrmDb;
use crate::repository::Repository;
use crate::context::{UrmInfo, Tag, Attribute};

#[derive(Default, Serialize)]
pub struct Product {
  pub pn: String,
  pub name: String,
  pub amount: u64,
  pub r#in: Repository,
  pub on: String,
  pub tags: Vec<Tag>,
  pub attributes: Vec<Attribute>,
}

#[get("/product/<pn>", format = "json")]
pub fn api(urm_info: State<UrmInfo>, db: UrmDb, pn: String) -> Result<JsonValue, NotFound<JsonValue>> {
  let ctx = api::Context::from_db(&urm_info, &db, pn.clone()).unwrap();
  if let Some(_) = ctx.product {
    Ok(json!(ctx))
  } else {
    Err(NotFound(json!({ "error": format!("P/N {} does not exist.", pn) })))
  }
}

#[get("/product/<pn>", format = "html", rank = 1)]
pub fn ui(urm_info: State<UrmInfo>, db: UrmDb, pn: String) -> Result<Template, NotFound<()>> {
  let ctx = ui::Context::from_db(&urm_info, &db, pn.clone()).unwrap();
  if let Some(_) = ctx.product {
    Ok(Template::render("product", ctx))
  } else {
    Err(NotFound(()))
  }
}
