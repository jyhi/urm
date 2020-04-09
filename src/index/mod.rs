use rocket::response::Redirect;
use rocket_contrib::json::JsonValue;

#[get("/", format = "json")]
pub fn api() -> JsonValue {
  json!({
    "error": true,
    "desc": "Not implemented"
  })
}

#[get("/", format = "html", rank = 1)]
pub fn ui() -> Redirect {
  Redirect::to("/dashboard")
}
