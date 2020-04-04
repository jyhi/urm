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

  // XXX: "Using this method is typically unnecessary as State implements Deref
  // with a Deref::Target of T." Why inner() is required for rustc to satisfy
  // the trait constraint of Template::render()?
  // https://api.rocket.rs/v0.4/rocket/struct.State.html#method.inner
  let ctx = UrmContext {
    urm: &urm_info,
    repositories: &repositories,
    products: &products,
  };

  Template::render("dashboard", ctx)
}
