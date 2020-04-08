use rocket::State;
use rocket_contrib::json::JsonValue;
use rocket_contrib::templates::Template;
use rocket_contrib::databases::mongodb::db::ThreadedDatabase;
use serde::Serialize;
use crate::database::UrmDb;
use crate::context::UrmInfo;

#[derive(Serialize)]
pub struct DashboardContext<'a> {
  urm: &'a UrmInfo,
  nprod: u64,
}

impl<'a> DashboardContext<'a> {
  fn from_db(urm_info: &'a UrmInfo, db: &'a UrmDb) -> Self {
    // TODO: Error handling
    // XXX: Why count returns Result<i64>?
    let nprod = db.collection("products")
      .count(None, None)
      .unwrap() as u64;

    DashboardContext {
      urm: urm_info,
      nprod: nprod,
    }
  }
}

#[get("/dashboard", format = "json")]
pub fn api(urm_info: State<UrmInfo>, db: UrmDb) -> JsonValue {
  let ctx = DashboardContext::from_db(&urm_info, &db);
  json!(ctx)
}

#[get("/dashboard", format = "html", rank = 1)]
pub fn ui(urm_info: State<UrmInfo>, db: UrmDb) -> Template {
  let ctx = DashboardContext::from_db(&urm_info, &db);
  Template::render("dashboard", ctx)
}
