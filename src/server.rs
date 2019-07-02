use rocket_contrib::json::Json;
use crate::db::Db;
use crate::url::Url;
use crate::constants::{PROTOCOL, HOSTNAME, PORT};
use rocket::response::Redirect;
use rocket::Config;
use rocket::config::Environment;

pub struct Server { }

#[post("/create", format = "application/json", data = "<url>")]
fn create(url: Json<Url>) -> Json<Url> {
    let mut id: String = String::from("");
    if url.id == "" {
        id = Db::add_url(url.url.clone());
    } else {
        id = Db::add_custom_url(url.id.clone(), url.url.clone());
    }

    Json(Url {
        id: id.clone(),
        url: format!("{}://{}/{}", PROTOCOL, HOSTNAME, id)
    })
}

#[get("/<id>")]
fn redirect(id: String) -> Redirect {
    let url = Db::get_url(id.clone());

    Redirect::to(url)
}

impl Server {
    pub fn new() -> Server { Server { } }

    pub fn start(&self) {
        Db::initialize();

        let config = Config::build(Environment::Staging)
            .address("0.0.0.0")
            .port(PORT)
            //.tls("./certs.pem", "./key.pem")
            .finalize()
            .unwrap();
        rocket::custom(config).mount("/", routes![create, redirect]).launch();
    }
}