#[macro_use] extern crate rocket;

use rocket::response::content;

#[cfg(test)] mod tests;

#[derive(FromFormField)]
enum Lang {
    #[field(value = "en")]
    English,
    #[field(value = "ru")]
    #[field(value = "ру")]
    Russian
}

#[derive(FromForm)]
struct Options<'r> {
    emoji: bool,
    name: Option<&'r str>,
}

// Try visiting:
//   http://127.0.0.1:8000/hello/world
#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

mod read_json;

use rocket::serde::json::{Json, Value};

#[get("/slither")]
fn slither() -> Json<Value> {

    let u = read_json::read_json_from_file("data/sample.json").unwrap();

    println!("{}", u["results"]["printers"][0]);
    Json(u)
}

// json response
#[get("/json")]
fn json() -> content::Json<Value> {

    let user = read_json::read_user_from_file("data/test.json").unwrap();
    println!("{:#?}", user);

    let u = read_json::read_json_from_file("data/test.json").unwrap();
    println!("{:#?}", u);

    content::Json(u)
}


// Try visiting:
//   http://127.0.0.1:8000/hello/мир
#[get("/мир")]
fn mir() -> &'static str {
    "Привет, мир!"
}

// Try visiting:
//   http://127.0.0.1:8000/wave/Rocketeer/100
#[get("/<name>/<age>")]
fn wave(name: &str, age: u8) -> String {
    format!("👋 Hello, {} year old named {}!", age, name)
}

#[get("/?<lang>&<opt..>")]
fn hello(lang: Option<Lang>, opt: Options<'_>) -> String {
    let mut greeting = String::new();
    if opt.emoji {
        greeting.push_str("👋 ");
    }

    match lang {
        Some(Lang::Russian) => greeting.push_str("Привет"),
        Some(Lang::English) => greeting.push_str("Hello"),
        None => greeting.push_str("Hi"),
    }

    if let Some(name) = opt.name {
        greeting.push_str(", ");
        greeting.push_str(name);
    }

    greeting.push('!');
    greeting
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/hello", routes![world, mir, json, slither])
        .mount("/wave", routes![wave])
}
