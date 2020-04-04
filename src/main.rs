#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde;

mod context;
mod index;
mod dashboard;

use rocket_contrib::helmet::SpaceHelmet;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use context::UrmInfo;

fn main() {
  let urm_info: UrmInfo = Default::default();

  rocket::ignite()
    .mount(
      "/",
      routes![
        index::ui,
        index::api,
        dashboard::ui,
        dashboard::api
      ]
    )
    .mount("/", StaticFiles::from("static"))
    .attach(SpaceHelmet::default())
    .attach(Template::fairing())
    .manage(urm_info)
    .launch();
}
