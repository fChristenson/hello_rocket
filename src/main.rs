#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, response::NamedFile, routes};
use rocket_contrib::{json::Json, serve::StaticFiles, templates::Template};
use std::path::Path;

mod lib;

#[get("/")]
fn index() -> NamedFile {
    NamedFile::open(Path::new("static/foobar.html")).unwrap()
}

#[get("/foo")]
fn foo() -> Template {
    let context = lib::foo::Foo {
        message: "This is the foo page",
    };
    Template::render("index", &context)
}

#[get("/bar")]
fn bar() -> Json<lib::bar::Bar> {
    Json(lib::bar::Bar { bar: 123 })
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, foo, bar])
        .mount("/", StaticFiles::from("static"))
        .attach(Template::fairing())
        .launch();
}
