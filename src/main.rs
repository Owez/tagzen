#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod tv;
mod utils;

fn main() {
    rocket::ignite()
        .attach(rocket_contrib::helmet::SpaceHelmet::default())
        .mount("/", routes![tv::single])
        .launch();
}
