use rocket::response::Redirect;

#[get("/", format = "json")]
pub fn api() -> Redirect {
  Redirect::to("/dashboard")
}

#[get("/", format = "html", rank = 1)]
pub fn ui() -> Redirect {
  Redirect::to("/dashboard")
}
