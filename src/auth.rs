use std::str::{self, FromStr};
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use rocket::response::Response;
use rocket_contrib::databases::mongodb;
use rocket_contrib::databases::mongodb::{
  bson,
  doc,
  db::ThreadedDatabase,
};
use crate::database::UrmDb;
use crate::config::UrmConfig;

#[derive(Debug)]
pub enum UrmAuthError {
  NotFound,
  Invalid,
}

#[derive(Debug)]
pub struct UrmAuth {
  pub username: String,
  pub password: String,
}

impl FromStr for UrmAuth {
  type Err = UrmAuthError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let auth: Vec<&str> = s.split(' ').collect();
    match auth[0] {
      "Basic" => {
        match base64::decode(auth[1]) {
          Ok(decoded) => {
            match str::from_utf8(&decoded) {
              Ok(s) => {
                let cred: Vec<&str> = s.split(':').collect();
                Ok(UrmAuth {
                  username: cred[0].to_owned(),
                  password: cred[1].to_owned(),
                })
              }
              Err(_) => {
                Err(UrmAuthError::Invalid)
              }
            }
          }
          Err(_) => {
            Err(UrmAuthError::Invalid)
          }
        }
      }
      _ => {
        // Unsupported authentication method
        Err(UrmAuthError::Invalid)
      }
    }
  }
}

impl<'a, 'r> FromRequest<'a, 'r> for UrmAuth {
  type Error = UrmAuthError;

  fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
    match request.headers().get_one("Authorization") {
      Some(auth) => {
        match auth.parse::<UrmAuth>() {
          Ok(parsed) => {
            Outcome::Success(parsed)
          }
          Err(e) => {
            Outcome::Failure((Status::BadRequest, e))
          }
        }
      }
      None => {
        Outcome::Failure((Status::Unauthorized, UrmAuthError::NotFound))
      }
    }
  }
}

pub fn check_db(db: &UrmDb, config: &UrmConfig, cred: &UrmAuth) -> Result<Option<()>, mongodb::Error> {
  match db.collection(&config.collection.users)
    .find_one(Some(doc!{ "username": &cred.username }), None)?
  {
    Some(doc_user) => {
      if let Ok(pass) = doc_user.get_str("password") {
        if let Ok(_) = pbkdf2::pbkdf2_check(&cred.password, pass) {
          // Both username and password match
          Ok(Some(()))
        } else {
          // Username matches but password does not match
          Ok(None)
        }
      } else {
        // ValueAccessError; the stored data may not be a string
        Ok(None) // XXX
      }
    }
    None => {
      // No such user
      Ok(None)
    }
  }
}

#[catch(401)]
pub fn unauthorized() -> Response<'static> {
  Response::build()
    .status(Status::Unauthorized)
    .raw_header("WWW-Authenticate", "Basic realm=\"This operation requires authentication\"")
    .finalize()
}
