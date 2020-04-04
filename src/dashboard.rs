use rocket::State;
use rocket_contrib::json::JsonValue;
use rocket_contrib::templates::Template;
use crate::context::*;

#[get("/dashboard", format = "json")]
pub fn api() -> JsonValue {
  json!({
    "error": true,
    "desc": "Not implemented"
  })
}

#[get("/dashboard", format = "html", rank = 1)]
pub fn ui(urm_info: State<UrmInfo>) -> Template {
  // TODO: Fetch from DB
  let repositories = Repositories { number: 0 };
  let products = Products { number: 0 };

  let ctx = UrmContext {
    urm: &urm_info,
    repositories: &repositories,
    products: &products,
  };

  Template::render("dashboard", ctx)
}
