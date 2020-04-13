mod api;
mod ui;

use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::databases::mongodb;
use rocket_contrib::templates::Template;
use crate::database::UrmDb;
use crate::config::UrmConfig;

#[get("/products?<page>&<nitem>", format = "json")]
pub fn api(config: State<UrmConfig>, db: UrmDb, page: Option<u64>, nitem: Option<u64>)
  -> Result<Json<Vec<mongodb::Document>>, Json<mongodb::Error>>
{
  let page = page.unwrap_or(1);
  let nitem = nitem.unwrap_or(10);

  match api::from_db(&db, &config, page, nitem) {
    Ok(r) => Ok(Json(r)),
    Err(e) => Err(Json(e))
  }
}

#[get("/products?<page>&<nitem>", format = "html", rank = 1)]
pub fn ui(config: State<UrmConfig>, db: UrmDb, page: Option<u64>, nitem: Option<u64>)
  -> Result<Template, mongodb::Error>
{
  let page = page.unwrap_or(1);
  let nitem = nitem.unwrap_or(10);

  Ok(
    Template::render(
      "products", ui::Context::from_db(&db, &config, page, nitem)?
    )
  )
}
