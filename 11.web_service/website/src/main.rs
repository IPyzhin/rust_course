#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, json::Json};

#[get("/")]
fn index() -> &'static str {
    "Hello from Rocket!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[derive(Serialize)]
struct Todo {
    task: String,
    priority: String
}

#[get("/json")]
fn json_response() -> Json<Todo> {
    Json(Todo {
        task: String::from("Bring Doshik to Oleh"),
        priority: String::from("High"),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello, json_response])
}