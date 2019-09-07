use diesel::{prelude::*, sqlite::SqliteConnection};

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    let db = "./testdb.sqlite3";
    SqliteConnection::establish(db).unwrap_or_else(|_| panic!("Error connection to {}", db))
}

pub fn create_task(connection: &SqliteConnection, title: &str) {
    let task = models::NewTask { title, done: false };

    diesel::insert_into(schema::task::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new task");
}

pub fn query_task(connection: &SqliteConnection) -> Vec<models::Task> {
    use schema::task::dsl::task;

    task.load::<models::Task>(&*connection)
        .expect("Error loading tasks")
}

pub fn mark_complete(connection: &SqliteConnection, task_id: i32) {
    use schema::task::dsl::{done, task};

    let _ = diesel::update(task.find(task_id))
        .set(done.eq(true))
        .execute(connection)
        .unwrap_or_else(|_| panic!("unable to find post {}", task_id));
}

pub fn remove_task(connection: &SqliteConnection, task_id: i32) {
    use schema::task::dsl::task;

    let _ = diesel::delete(task.find(task_id)).execute(connection);
}
