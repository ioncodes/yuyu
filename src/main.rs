#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate rand;
extern crate sqlite;

mod constants;
mod db;
mod server;
mod url;

use crate::server::Server;

fn main() {
    let server = Server::new();
    server.start();
}
