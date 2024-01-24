use std::io::{self, BufRead};
mod crud;
mod db;

fn read_command() -> Option<String> {
    let mut buffer = String::new();
    match io::stdin().lock().read_line(&mut buffer) {
        Ok(_) => Some(buffer.trim().to_string()),
        Err(_) => None,
    }
}

fn main() {
    let mut todo: Vec<crud::TodoItem> = Vec::new();
    let mut conn = db::establish_connection();

    db::load_todo_from_database(&mut conn, &mut todo);

    loop {
        println!("Enter a command: ");
        if let Some(cmd) = read_command() {
            match cmd.as_str() {
                "add" => {
                    crud::add_todo(&mut todo);
                }
                "list" => crud::list_todo(&todo),
                "done" => crud::mark_todo_as_done(&mut todo),
                "count-todo" => crud::count_todo(&todo),
                "remove" => crud::remove_todo(&mut todo),
                "quit" => {
                    db::save_to_database(&mut conn, &todo);
                    break;
                }
                _ => println!("unknown command"),
            }
        }
    }

    println!("quit");
}
