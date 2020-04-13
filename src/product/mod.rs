mod structure;
mod api;
mod ui;

use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::databases::mongodb;
use rocket_contrib::templates::Template;
use crate::database::UrmDb;
use crate::config::UrmConfig;

pub use structure::Product;

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
