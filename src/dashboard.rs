use rocket::State;
use rocket_contrib::json::JsonValue;
use rocket_contrib::templates::Template;
use serde::Serialize;
use crate::context::UrmInfo;

#[derive(Serialize)]
pub struct DashboardContext<'a> {
  urm: &'a UrmInfo,
  nprod: u64,
}

impl<'a> DashboardContext<'a> {
  fn test(urm_info: &'a UrmInfo) -> Self {
    DashboardContext {
      urm: urm_info,
      nprod: 42,
    }
  }
}

#[get("/dashboard", format = "json")]
pub fn api() -> JsonValue {
  json!({
    "error": true,
    "desc": "Not implemented"
  })
}

#[get("/dashboard", format = "html", rank = 1)]
pub fn ui(urm_info: State<UrmInfo>) -> Template {
  let ctx = DashboardContext::test(&urm_info);
  Template::render("dashboard", ctx)
}
