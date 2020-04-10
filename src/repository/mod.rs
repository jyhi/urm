mod structure;
mod api;
mod ui;

use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::databases::mongodb;
use rocket_contrib::templates::Template;
use crate::database::UrmDb;
use crate::context::UrmInfo;

pub use structure::Repository;

#[get("/repository/<ln_p>", format = "json")]
pub fn api(db: UrmDb, ln_p: String)
  -> Result<Option<Json<mongodb::Document>>, Json<mongodb::error::Error>>
{
  match api::from_db(&db, ln_p) {
    Ok(r) => match r {
      Some(doc) => Ok(Some(Json(doc))),
      None => Ok(None)
    },
    Err(e) => Err(Json(e))
  }
}

#[get("/repository/<ln_p>", format = "html", rank = 1)]
pub fn ui(urm_info: State<UrmInfo>, db: UrmDb, ln_p: String)
  -> Result<Option<Template>, mongodb::error::Error>
{
  match ui::Context::from_db(&db, &urm_info, ln_p) {
    Ok(r) => match r {
      Some(ctx) => Ok(Some(Template::render("repository", ctx))),
      None => Ok(None)
    }
    Err(e) => Err(e)
  }
}
