mod structure;
mod api;
mod ui;

use rocket::State;
use rocket::response::status::NotFound;
use rocket_contrib::json::JsonValue;
use rocket_contrib::templates::Template;
use crate::database::UrmDb;
use crate::context::{UrmInfo, PageInfo};

pub use structure::Repository;

#[get("/repository/<ln_p>", format = "json")]
pub fn api(db: UrmDb, ln_p: String) -> Result<JsonValue, NotFound<JsonValue>> {
  let ctx = api::Context::from_db(&db, ln_p.clone()).unwrap();
  if let Some(_) = ctx.repository {
    Ok(json!(ctx))
  } else {
    Err(NotFound(json!({ "error": format!("L/N-P {} does not exist.", ln_p) })))
  }
}

#[get("/repository/<ln_p>", format = "html", rank = 1)]
pub fn ui(urm_info: State<UrmInfo>, db: UrmDb, ln_p: String) -> Result<Template, ()> {
  let page_info = PageInfo { current: 1, min: 1, max: 1 };

  let ctx = ui::Context::from_db(&urm_info, &page_info, &db, ln_p).unwrap();
  if let Some(_) = ctx.repository {
    Ok(Template::render("repository", ctx))
  } else {
    Err(())
  }
}
