mod api;
mod ui;

use rocket::State;
use rocket_contrib::json::JsonValue;
use rocket_contrib::templates::Template;
use crate::database::UrmDb;
use crate::context::UrmInfo;

#[get("/dashboard", format = "json")]
pub fn api(db: UrmDb) -> JsonValue {
  let ctx = api::Context::from_db(&db);
  json!(ctx)
}

#[get("/dashboard", format = "html", rank = 1)]
pub fn ui(urm_info: State<UrmInfo>, db: UrmDb) -> Template {
  let ctx = ui::Context::from_db(&urm_info, &db);
  Template::render("dashboard", ctx)
}
