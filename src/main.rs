use std::fs::File;
use std::io::{self, BufRead, BufReader}; // Add missing import statement // Add missing import statement

fn input_user() -> String {
    let mut input_user = String::new();
    io::stdin()
        .read_line(&mut input_user)
        .expect("Failed to read input");
    input_user
}

fn count_todo(file: &File) -> String {
    let mut count = 0;
    let reader = BufReader::new(file); // Call expect method on BufReader::new

    // loop through each line
    for (_line_number, _line) in reader.lines().enumerate() {
        count += 1;
    }

    return count.to_string();
}

struct TodoItem {
    note: String,
    completed: bool,
}

fn add_todo(todo: &mut Vec<TodoItem>) {
    println!("Enter a TODO item (enter to submit): ");
    let input = input_user();

    // add to file
    todo.push(TodoItem {
        note: input,
        completed: false,
    });
}

fn list_todo(todo: &Vec<TodoItem>) {
    for todo_item in todo {
        if todo_item.completed {
            println!("{} status: done", todo_item.note);
        } else {
            println!("{} status: not done", todo_item.note);
        }
    }
}

fn mark_todo_as_done() {
    println!("mark todo as done");
}

fn remove_todo() {
    println!("remove todo");
}

fn read_command() -> Option<String> {
    let mut buffer = String::new();
    match io::stdin().lock().read_line(&mut buffer) {
        Ok(_) => Some(buffer.trim().to_string()),
        Err(_) => None,
    }
}

fn main() {
    let mut todo: Vec<TodoItem> = Vec::new();

    loop {
        println!("Enter a command: ");
        if let Some(cmd) = read_command() {
            match cmd.as_str() {
                "add" => {
                    add_todo(&mut todo);
                }
                "list" => list_todo(&todo),
                "done" => mark_todo_as_done(),
                "remove" => remove_todo(),
                "quit" => break,
                _ => println!("unknown command"),
            }
        }
    }

    println!("quit");
}
