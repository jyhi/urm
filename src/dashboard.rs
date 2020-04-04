use rocket::State;
use rocket_contrib::json::JsonValue;
use rocket_contrib::templates::Template;
use crate::context::UrmContext;

#[get("/dashboard", format = "json")]
pub fn api() -> JsonValue {
  json!({
    "error": true,
    "desc": "Not implemented"
  })
}

#[get("/dashboard", format = "html", rank = 1)]
pub fn ui(ctx: State<UrmContext>) -> Template {
  // XXX: "Using this method is typically unnecessary as State implements Deref
  // with a Deref::Target of T." Why inner() is required for rustc to satisfy
  // the trait constraint of Template::render()?
  // https://api.rocket.rs/v0.4/rocket/struct.State.html#method.inner
  Template::render("dashboard", ctx.inner())
}
