#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde;
extern crate mytodo;

use mytodo::db::models::Task;
use mytodo::db::{establish_connection, query_task};
use rocket_contrib::json::Json;

#[derive(Serialize)]
struct JsonAPIResponse {
    data: Vec<Task>,
}

fn main() {
    rocket::ignite().mount("/", routes![tasks_get]).launch();
}

#[get("/api/v1/tasks")]
fn tasks_get() -> Json<JsonAPIResponse> {
    let mut response = JsonAPIResponse { data: vec![] };

    let conn = establish_connection();
    for task in query_task(&conn) {
        response.data.push(task);
    }

    Json(response)
}
