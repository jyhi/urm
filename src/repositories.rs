use rocket::State;
use rocket_contrib::json::JsonValue;
use rocket_contrib::templates::Template;
use crate::context::*;

#[get("/repositories", format = "json")]
pub fn api() -> JsonValue {
  json!({
    "error": true,
    "desc": "Not implemented"
  })
}

#[get("/repositories", format = "html", rank = 1)]
pub fn ui(urm_info: State<UrmInfo>) -> Template {
  // TODO: Fetch from DB
  let repo_list = vec![
    Repository { name: "Repository 1".to_string(), load: 114000 },
    Repository { name: "Repository 2".to_string(), load: 514 },
  ];
  let repositories = Repositories { number: repo_list.len() as u64, list: repo_list };
  let products = Products { number: 0 };

  let ctx = UrmContext {
    urm: &urm_info,
    repositories: &repositories,
    products: &products,
  };

  Template::render("repositories", ctx)
}
