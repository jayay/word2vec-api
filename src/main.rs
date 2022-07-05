// only c version of trainer works for this
extern crate word2vec;

extern crate word2vec_api_lib;
use word2vec_api_lib::build_rocket;
use std::env;
use rocket::launch;

#[launch]
fn rocket() -> _ {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("No file name given.");
    build_rocket(filename)
}