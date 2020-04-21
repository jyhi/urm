extern crate rand;
extern crate argon2;
#[macro_use] extern crate serde;
extern crate serde_json;

use rand::RngCore;

#[derive(Serialize)]
struct Credential<'a> {
  username: &'a str,
  password: &'a str,
}

fn main() -> Result<(), argon2::Error> {
  let mut args = std::env::args();
  let progname = args.next();
  let username = args.next();
  let password = args.next();

  if username == None || password == None {
    eprintln!("Usage: {} <username> <password>", progname.unwrap());
    std::process::exit(1);
  }

  let mut salt: [u8; 16] = [0; 16];
  rand::thread_rng().fill_bytes(&mut salt);

  let config = argon2::Config {
    thread_mode: argon2::ThreadMode::Parallel,
    variant: argon2::Variant::Argon2id,
    ..argon2::Config::default()
  };

  let hash = argon2::hash_encoded(password.unwrap().as_bytes(), &salt, &config)?;

  let cred = Credential {
    username: &username.unwrap(),
    password: &hash,
  };
  let json = serde_json::to_string(&cred).unwrap();

  println!("{}", json);

  Ok(())
}
