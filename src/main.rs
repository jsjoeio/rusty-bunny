#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::NamedFile;
use rocket::response::Redirect;

mod utils;

#[get("/favicon.ico")]
fn favicon() -> Option<NamedFile> {
    NamedFile::open("./static/favicon.co").ok()
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
// rename cmd to query
#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    // We need a way to match only on the cmd, without losing the rest of the query
    // "tw something"
    let command = utils::get_command_from_query_string(&cmd);

    // Keep in alphabetic order
    let redirect_url = match command {
        "cal" => String::from("https://calendar.google.com/"),
        "ck" => String::from("https://app.convertkit.com/"),
        "clojurefam" | "cljfam" => String::from("https://github.com/athensresearch/clojurefam"),
        "drive" => String::from("https://drive.google.com/"),
        "dp" | "disney" | "disneyplus" => String::from("https://disneyplus.com"),
        "foam" => String::from("https://foambubble.github.io/foam"),
        "gh foam" => String::from("https://github.com/foambubble/foam"),
        "gh" => utils::github::construct_github_url(&cmd),
        "hey" => String::from("https://app.hey.com/"),
        "jp" => String::from("https://joeprevite.com"),
        "ip" => String::from("https://instapaper.com"),
        "ih" => String::from("https://indiehackers.com"),
        "l3" => String::from("http://localhost:3000/"),
        "l8" => String::from("http://localhost:8000/"),
        "mail" => String::from("https://mail.google.com/"),
        "map" | "maps" => String::from("https://maps.google.com/"),
        "photo" | "photos" => String::from("https://photos.google.com/"),
        "og" => String::from("https://onegraph.com/"),
        "tw" => utils::twitter::construct_twitter_url(&cmd),
        // If no match, we search on Google
        _ => utils::google::construct_google_search_url(&cmd),
    };

    Redirect::to(redirect_url)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, search, favicon])
        .launch();
}
