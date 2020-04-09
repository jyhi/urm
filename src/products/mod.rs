mod api;
mod ui;

use rocket::State;
use rocket_contrib::json::JsonValue;
use rocket_contrib::templates::Template;
use crate::database::UrmDb;
use crate::context::{UrmInfo, PageInfo};

#[get("/products", format = "json")]
pub fn api(db: UrmDb) -> JsonValue {
  match api::Context::from_db(&db) {
    Ok(ctx) => json!(ctx),
    Err(e) => json!({
      "error": e.to_string()
    })
  }
}

#[get("/products", format = "html", rank = 1)]
pub fn ui(urm_info: State<UrmInfo>, db: UrmDb) -> Result<Template, ()> {
  let page_info = PageInfo { current: 1, min: 1, max: 1 };

  match ui::Context::from_db(&urm_info, &page_info, &db) {
    Ok(ctx) => Ok(Template::render("products", ctx)),
    Err(_) => Err(())
  }
}
