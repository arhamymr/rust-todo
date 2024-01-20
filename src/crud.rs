use std::io;

pub fn input_user() -> String {
    let mut input_user = String::new();
    io::stdin()
        .read_line(&mut input_user)
        .expect("Failed to read input");
    input_user
}

pub struct TodoItem {
    pub note: String,
    pub completed: bool,
}

pub fn add_todo(todo: &mut Vec<TodoItem>) {
    println!("Enter a TODO item (enter to submit): ");
    let input = input_user();

    // add to file
    todo.push(TodoItem {
        note: input,
        completed: false,
    });
}

pub fn list_todo(todo: &Vec<TodoItem>) {
    for (index, todo_item) in todo.iter().enumerate() {
        if todo_item.completed {
            println!("{}. {} status: done", index + 1, todo_item.note);
        } else {
            println!("{}. {} status: not done", index + 1, todo_item.note);
        }
    }
}

pub fn mark_todo_as_done(todo: &mut Vec<TodoItem>) {
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

pub fn remove_todo(todo: &mut Vec<TodoItem>) {
    list_todo(&todo);
    println!("Input number todo to remove");

    let input = input_user();

    let input: usize = input.trim().parse().unwrap();

    if input > todo.len() {
        println!("Todo with number {} is not found", input);
    } else {
        todo.remove(input - 1);
        println!("Successfully remove todo");
    }
}

pub fn count_todo(todo: &Vec<TodoItem>) {
    println!("{}", todo.len())
}
