use sqlite::{Connection, Value};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub struct Db { }

impl Db {
    pub fn initialize() {
        let connection = sqlite::open("./urls.db").unwrap();
        connection.execute("CREATE TABLE IF NOT EXISTS urls (id TEXT UNIQUE, url TEXT)");
    }

    pub fn add_url(url: String) -> String {
        let id: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(5)
            .collect();

        let connection = sqlite::open("./urls.db").unwrap();

        let mut cursor = connection
            .prepare("INSERT INTO urls VALUES (?, ?)")
            .unwrap()
            .cursor();
        cursor.bind(&[Value::String(id.clone()), Value::String(url)]).unwrap();

        cursor.next();

        id
    }

    pub fn add_custom_url(id: String, url: String) -> String {
        let connection = sqlite::open("./urls.db").unwrap();

        let mut cursor = connection
            .prepare("INSERT INTO urls VALUES (?, ?)")
            .unwrap()
            .cursor();
        cursor.bind(&[Value::String(id.clone()), Value::String(url)]).unwrap();

        cursor.next();

        id
    }

    pub fn get_url(id: String) -> String {
        let connection = sqlite::open("./urls.db").unwrap();

        let mut cursor = connection
            .prepare("SELECT url FROM urls WHERE id = ?")
            .unwrap()
            .cursor();
        cursor.bind(&[Value::String(id.clone())]).unwrap();

        let row = cursor.next().unwrap().unwrap();

        dbg!(row);

        String::from(row[0].as_string().unwrap())
    }
}