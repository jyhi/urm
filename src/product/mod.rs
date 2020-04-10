mod structure;
mod api;
mod ui;

use rocket::State;
use rocket::response::status::NotFound;
use rocket_contrib::json::Json;
use rocket_contrib::databases::mongodb;
use rocket_contrib::templates::Template;
use crate::database::UrmDb;
use crate::context::UrmInfo;

pub use structure::Product;

#[get("/product/<pn>", format = "json")]
pub fn api(db: UrmDb, pn: String)
  -> Result<Option<Json<mongodb::Document>>, Json<mongodb::error::Error>>
{
  match api::from_db(&db, pn) {
    Ok(r) => match r {
      Some(doc) => Ok(Some(Json(doc))),
      None => Ok(None)
    },
    Err(e) => Err(Json(e))
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
