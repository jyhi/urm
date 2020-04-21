#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
extern crate base64;
extern crate argon2;
extern crate toml;

mod config;
mod context;
mod database;
mod index;
mod dashboard;
mod repositories;
mod repository;
mod products;
mod product;
mod search;
mod auth;

use rocket_contrib::helmet::SpaceHelmet;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use database::UrmDb;
use config::UrmConfig;

fn main() {
  // TODO: Read file name from command line
  let urm_config = UrmConfig::from_file("urm.toml");

  rocket::ignite()
    .mount(
      &urm_config.mount_point,
      routes![
        index::ui,
        index::api,
        dashboard::ui,
        dashboard::api,
        repositories::ui,
        repositories::api,
        repository::ui,
        repository::api,
        repository::api_create,
        repository::api_set_field,
        repository::api_replace,
        repository::api_remove,
        products::ui,
        products::api,
        product::ui,
        product::api,
        product::api_create,
        product::api_set_field,
        product::api_replace,
        product::api_remove,
        search::ui,
        search::api,
      ]
    )
    .mount(&urm_config.mount_point, StaticFiles::from("static"))
    .register(catchers![auth::unauthorized])
    .attach(SpaceHelmet::default())
    .attach(Template::fairing())
    .attach(UrmDb::fairing())
    .manage(urm_config)
    .launch();
}
