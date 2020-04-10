mod api;
mod ui;

use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::databases::mongodb;
use rocket_contrib::templates::Template;
use crate::database::UrmDb;
use crate::context::{UrmInfo, PageInfo};

#[get("/repositories", format = "json")]
pub fn api(db: UrmDb)
  -> Result<Json<Vec<mongodb::Document>>, Json<mongodb::error::Error>>
{
  match api::from_db(&db) {
    Ok(r) => Ok(Json(r)),
    Err(e) => Err(Json(e))
  }
}

#[get("/repositories", format = "html", rank = 1)]
pub fn ui(urm_info: State<UrmInfo>, db: UrmDb)
  -> Result<Template, mongodb::error::Error>
{
  let page_info = PageInfo { current: 1, min: 1, max: 1 };

  Ok(
    Template::render(
      "repositories", ui::Context::from_db(&urm_info, &page_info, &db)?
    )
  )
}
