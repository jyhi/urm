mod api;
mod ui;

use rocket::State;
use rocket_contrib::json::JsonValue;
use rocket_contrib::templates::Template;
use crate::context::{UrmInfo, PageInfo};

#[get("/repositories", format = "json")]
pub fn api() -> JsonValue {
  let ctx = api::Context::test();
  json!(ctx)
}

#[get("/repositories", format = "html", rank = 1)]
pub fn ui(urm_info: State<UrmInfo>) -> Template {
  let page_info = PageInfo { current: 1, min: 1, max: 1 };

  let ctx = ui::Context::test(&urm_info, &page_info);
  Template::render("repositories", ctx)
}
