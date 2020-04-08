use rocket_contrib::databases::mongodb;

#[database("mongo_main")]
pub struct UrmDb(mongodb::db::Database);
