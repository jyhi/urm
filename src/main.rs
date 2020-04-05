#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde;
extern crate toml;

mod config;
mod context;
mod index;
mod dashboard;
mod repositories;
mod products;

use rocket_contrib::helmet::SpaceHelmet;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use config::UrmConfig;
use context::UrmInfo;

fn main() {
  // TODO: Read file name from command line
  let urm_config = UrmConfig::from_file("urm.toml".to_string());

  let mut urm_info: UrmInfo = Default::default();
  if let Some(urm) = urm_config.urm {
    if let Some(brand) = urm.brand {
      urm_info.brand = brand;
    }
  }

  rocket::ignite()
    .mount(
      "/",
      routes![
        index::ui,
        index::api,
        dashboard::ui,
        dashboard::api,
        repositories::ui,
        repositories::api,
        products::ui,
        products::api,
      ]
    )
    .mount("/", StaticFiles::from("static"))
    .attach(SpaceHelmet::default())
    .attach(Template::fairing())
    .manage(urm_info)
    .launch();
}
