mod structure;
mod api;
mod ui;

use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::databases::mongodb;
use rocket_contrib::templates::Template;
use crate::database::UrmDb;
use crate::config::UrmConfig;

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

#[get("/repository/<ln_p>?<page>&<nitem>", format = "html", rank = 1)]
pub fn ui(config: State<UrmConfig>, db: UrmDb, ln_p: String, page: Option<u64>, nitem: Option<u64>)
  -> Result<Option<Template>, mongodb::error::Error>
{
  let page = page.unwrap_or(1);
  let nitem = nitem.unwrap_or(10);

  match ui::Context::from_db(&db, &config, ln_p, page, nitem) {
    Ok(r) => match r {
      Some(ctx) => Ok(Some(Template::render("repository", ctx))),
      None => Ok(None)
    }
    Err(e) => Err(e)
  }
}
