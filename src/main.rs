use std::io::{self, BufRead};

fn input_user() -> String {
    let mut input_user = String::new();
    io::stdin()
        .read_line(&mut input_user)
        .expect("Failed to read input");
    input_user
}

fn count_todo(todo: &Vec<TodoItem>) {
    println!("{}", todo.len())
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
    for (index, todo_item) in todo.iter().enumerate() {
        if todo_item.completed {
            print!("{}. {} status: done", index + 1, todo_item.note);
        } else {
            print!("{}. {} status: not done", index + 1, todo_item.note);
        }
        println!(); // Add a newline after printing all todo items
    }
}

fn mark_todo_as_done(todo: &mut Vec<TodoItem>) {
    list_todo(&todo);
    println!("Input number todo to mark todo as done");

    let input = input_user();

    let input: usize = input.trim().parse().unwrap();

    if let Some(todo_item) = todo.get_mut(input - 1) {
        todo_item.completed = true;
        println!("Successfully edit todo");
    } else {
        println!("Todo with number {} is not found", input);
    }
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
                "done" => mark_todo_as_done(&mut todo),
                "count-todo" => count_todo(&todo),
                "remove" => remove_todo(),
                "quit" => break,
                _ => println!("unknown command"),
            }
        }
    }

    println!("quit");
}
