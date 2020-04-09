mod structure;
mod api;
mod ui;

use rocket::State;
use rocket::response::status::NotFound;
use rocket_contrib::json::JsonValue;
use rocket_contrib::templates::Template;
use crate::database::UrmDb;
use crate::context::UrmInfo;

pub use structure::Product;

#[get("/product/<pn>", format = "json")]
pub fn api(db: UrmDb, pn: String) -> Result<JsonValue, NotFound<JsonValue>> {
  let ctx = api::Context::from_db(&db, pn.clone()).unwrap();
  if let Some(_) = ctx.product {
    Ok(json!(ctx))
  } else {
    Err(NotFound(json!({ "error": format!("P/N {} does not exist.", pn) })))
  }
}

#[get("/product/<pn>", format = "html", rank = 1)]
pub fn ui(urm_info: State<UrmInfo>, db: UrmDb, pn: String) -> Result<Template, NotFound<()>> {
  let ctx = ui::Context::from_db(&urm_info, &db, pn).unwrap();
  if let Some(_) = ctx.product {
    Ok(Template::render("product", ctx))
  } else {
    Err(NotFound(()))
  }
}
