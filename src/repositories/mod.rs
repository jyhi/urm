mod api;
mod ui;

use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::databases::mongodb;
use rocket_contrib::templates::Template;
use crate::database::UrmDb;
use crate::context::UrmInfo;

#[get("/repositories?<page>&<nitem>", format = "json")]
pub fn api(db: UrmDb, page: Option<u64>, nitem: Option<u64>)
  -> Result<Json<Vec<mongodb::Document>>, Json<mongodb::error::Error>>
{
  let page = page.unwrap_or(1);
  let nitem = nitem.unwrap_or(10);

  match api::from_db(&db, page, nitem) {
    Ok(r) => Ok(Json(r)),
    Err(e) => Err(Json(e))
  }
}

#[get("/repositories?<page>&<nitem>", format = "html", rank = 1)]
pub fn ui(urm_info: State<UrmInfo>, db: UrmDb, page: Option<u64>, nitem: Option<u64>)
  -> Result<Template, mongodb::error::Error>
{
  let page = page.unwrap_or(1);
  let nitem = nitem.unwrap_or(10);

  Ok(
    Template::render(
      "repositories", ui::Context::from_db(&db, &urm_info, page, nitem)?
    )
  )
}
