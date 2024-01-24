use crate::crud;
use rusqlite::Connection;

const DATABASE_URL: &str = "todo.db";
const TABLE_NAME: &str = "todo";

pub fn establish_connection() -> Connection {
    let connection = Connection::open(DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connection in memory to memory database"));
    println!("Successfully established connection to {}", DATABASE_URL);
    connection
}

pub fn create_table(conn: &mut Connection) {
    let query = format!(
        "CREATE TABLE IF NOT EXISTS {} (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      note TEXT NOT NULL,
      completed BOOLEAN NOT NULL
      )",
        TABLE_NAME
    );

    let _ = conn.execute(&query, ());
}

pub fn load_todo_from_database(conn: &mut Connection, todo: &mut Vec<crud::TodoItem>) {
    let mut stmt = conn
        .prepare("SELECT note, completed FROM todo")
        .expect("failed to prepare select statement");

    let todo_iter = stmt.query_map([], |row| {
        Ok(crud::TodoItem {
            note: row.get(0)?,
            completed: row.get(1)?,
        })
    });

    for todo_item in todo_iter.unwrap() {
        todo.push(todo_item.unwrap());
    }
}

pub fn save_to_database(conn: &mut Connection, data: &Vec<crud::TodoItem>) {
    // Create the table if it doesn't exist
    create_table(conn);

    // iterate todo
    for todo in data.iter() {
        print!("{}", todo.note);
        let note = todo.note.clone();
        conn.execute(
            "INSERT INTO todo (note, completed) VALUES (?1, ?2)",
            (&note, &todo.completed),
        )
        .expect("failed to insert data to database");
    }
    println!("save to database");
}
