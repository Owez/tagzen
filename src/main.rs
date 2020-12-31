//! An api microservice to tag and catagorise tv & movies at ease
//! 
//! # Running
//! 
//! Clone then build with `cargo build --release`. The outputted binary from this
//! will be outputted inside of `target/release` which can be optionally stripped
//! and ran as a static binary. All help infomation is contained inside of the api
//! itself, so just visit the index page once it's up and running 👐

#![feature(proc_macro_hygiene, decl_macro)]

/// Crate version
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[macro_use]
extern crate rocket;

mod tv;
mod utils;

#[get("/")]
fn index() -> String {
    format!("ROUTE /\n\n\nAbout\n    Microservice api for tagging television shows and movies for use publically\n    for free, forever. Created by https://ogriffiths.com. Help is available for\n    each route and endpoint on GET access. Running on v{} currently with the\n    foss repository contained inside of https://github.com/owez/tagzen/.\n\n\nChild routes/endpoints\n    - /tv: Television show tagging, allowing single episode or seasonal tagging", VERSION)
}

fn main() {
    rocket::ignite()
        .attach(rocket_contrib::helmet::SpaceHelmet::default())
        .mount(
            "/",
            routes![
                index,
                tv::help,
                tv::episode_help,
                tv::episode,
                tv::season_help,
                tv::season
            ],
        )
        .launch();
}
