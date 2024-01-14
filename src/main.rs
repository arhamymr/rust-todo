use std::fs::File;
use std::io;
use std::io::Write;

fn add_todo(mut file: File) {
    println!("Enter a TODO item (type 'done' when finished): ");

    let mut input_user = String::new();
    io::stdin().read_line(&mut input_user).unwrap();

    // add to file
    writeln!(file, "{}", input_user).unwrap();
}

fn list_todo() {
    println!("list todo");
}

fn mark_todo_as_done() {
    println!("mark todo as done");
}

fn remove_todo() {
    println!("remove todo");
}

fn read_command() -> Option<String> {
    let mut buffer = String::new();
    match std::io::stdin().read_line(&mut buffer) {
        Ok(_) => Some(buffer.trim().to_string()),
        Err(_) => None,
    }
}

fn main() {
    loop {
        if let Some(cmd) = read_command() {
            match cmd.as_str() {
                // "add" => add_todo(), and input file to add_todo
                "add" => {
                    let file_path = "todo.txt";
                    let file = File::create(file_path).unwrap();
                    add_todo(file);
                }
                "list" => list_todo(),
                "done" => mark_todo_as_done(),
                "remove" => remove_todo(),
                "quit" => break,
                _ => println!("unknown command"),
            }
        }
    }

    println!("hellow")
}
