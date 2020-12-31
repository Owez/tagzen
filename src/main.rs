#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod tv;
mod utils;

use utils::ResponseModel;

#[get("/")]
fn index() -> ResponseModel<()> {
    ResponseModel::basic(200, format!("Microservice called 'tagzen' on v{} made for tagging television shows and movies made by https://ogriffiths.com", env!("CARGO_PKG_VERSION")))
}

fn main() {
    rocket::ignite()
        .attach(rocket_contrib::helmet::SpaceHelmet::default())
        .mount("/", routes![index, tv::single])
        .launch();
}
