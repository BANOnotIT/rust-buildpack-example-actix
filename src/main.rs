//! Example actix-web application.
//!
//! This code is adapted from the front page of the [Actix][] website.
//!
//! [actix]: https://actix.rs/docs/

extern crate actix_web;
extern crate base64;
extern crate crc16;
extern crate reqwest;
extern crate telegram_typings;

use std::env;

use actix_web::{App, http, HttpRequest, Responder, server, State};

mod bot;

struct BotState {
    callback_signature: u16,
    channel_id: i64,
    chat_ids: Vec<i64>,
    bot: bot::Bot,
}

fn greet(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", to)
}

fn main() {
    // Get the port number to listen on.
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    // Start a server, configuring the resources to serve.
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(greet))
            .resource("/{name}", |r| r.f(greet))
    })
    .bind(("0.0.0.0", port))
    .expect("Can not bind to port")
    .run();
}
