mod api;
mod ui;

use rocket::State;
use rocket_contrib::json::JsonValue;
use rocket_contrib::templates::Template;
use serde::Serialize;
use crate::product::Product;
use crate::context::{UrmInfo, PageInfo, Tag};

#[derive(Default, Serialize)]
pub struct Repository {
  pub ln_p: String,
  pub name: String,
  pub load: u64,
  pub tags: Vec<Tag>,
  pub has: Option<Vec<Product>>,
}

#[get("/repository/<ln_p>", format = "json")]
pub fn api(ln_p: String) -> JsonValue {
  let ctx = api::Context::test(ln_p);
  json!(ctx)
}

#[get("/repository/<ln_p>", format = "html", rank = 1)]
pub fn ui(urm_info: State<UrmInfo>, ln_p: String) -> Template {
  let page_info = PageInfo { current: 1, min: 1, max: 1 };

  let ctx = ui::Context::test(&urm_info, &page_info, ln_p);
  Template::render("repository", ctx)
}
