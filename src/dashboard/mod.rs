mod api;
mod ui;

use rocket::State;
use rocket_contrib::json::JsonValue;
use rocket_contrib::databases::mongodb;
use rocket_contrib::templates::Template;
use crate::database::UrmDb;
use crate::config::UrmConfig;

#[get("/dashboard", format = "json")]
pub fn api(config: State<UrmConfig>, db: UrmDb) -> JsonValue {
  let ctx = api::Context::from_db(&db, &config);
  json!(ctx)
}

#[get("/dashboard", format = "html", rank = 1)]
pub fn ui(config: State<UrmConfig>, db: UrmDb)
  -> Result<Template, mongodb::error::Error>
{
  Ok(Template::render("dashboard", ui::Context::from_db(&db, &config)?))
}
