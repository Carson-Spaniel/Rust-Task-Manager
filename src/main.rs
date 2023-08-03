#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::serve::StaticFiles;
use std::path::PathBuf;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let static_path: PathBuf = PathBuf::from("C:/Users/carso/Github/Rust-Task-Manager/webpages");
    rocket::ignite()
        .mount("/", StaticFiles::from(static_path))
        .mount("/static", routes![index])
        .launch();
}