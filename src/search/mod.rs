mod api;
mod ui;

use rocket::State;
use rocket::request::Form;
use rocket_contrib::json::Json;
use rocket_contrib::databases::mongodb;
use rocket_contrib::templates::Template;
use crate::database::UrmDb;
use crate::config::UrmConfig;

#[derive(FromForm)]
pub struct SearchQuery {
  pub k: String,
  pub op: String,
  pub v: String,
  pub coll: String,
}

#[get("/search?<query..>", format = "json")]
pub fn api(db: UrmDb, query: Form<SearchQuery>)
  -> Result<Json<Vec<mongodb::Document>>, Json<mongodb::Error>>
{
  match api::from_db(&db, &query) {
    Ok(r) => Ok(Json(r)),
    Err(e) => Err(Json(e))
  }
}

#[get("/search?<query..>", format = "html", rank = 1)]
pub fn ui(config: State<UrmConfig>, db: UrmDb, query: Form<SearchQuery>)
  -> Result<Template, mongodb::Error>
{
  Ok(
    Template::render("search", ui::Context::from_db(&db, &config, &query)?)
  )
}
