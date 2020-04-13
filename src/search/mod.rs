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

#[get("/search?<page>&<nitem>&<query..>", format = "json")]
pub fn api(db: UrmDb, query: Form<SearchQuery>, page: Option<u64>, nitem: Option<u64>)
  -> Result<Json<Vec<mongodb::Document>>, Json<mongodb::Error>>
{
  let page = page.unwrap_or(1);
  let nitem = nitem.unwrap_or(10);

  match api::from_db(&db, &query, page, nitem) {
    Ok(r) => Ok(Json(r)),
    Err(e) => Err(Json(e))
  }
}

#[get("/search?<page>&<nitem>&<query..>", format = "html", rank = 1)]
pub fn ui(config: State<UrmConfig>, db: UrmDb, query: Form<SearchQuery>, page: Option<u64>, nitem: Option<u64>)
  -> Result<Template, mongodb::Error>
{
  let page = page.unwrap_or(1);
  let nitem = nitem.unwrap_or(10);

  Ok(
    Template::render("search", ui::Context::from_db(&db, &config, &query, page, nitem)?)
  )
}
