use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};

fn input_user() -> String {
    let mut input_user = String::new();
    io::stdin()
        .read_line(&mut input_user)
        .expect("Failed to read input");
    input_user
}
fn add_todo(path: &str) {
    println!("Enter a TODO item (enter to submit): ");

    let input = input_user();

    // open file in append mode
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)
        .expect("Failed to open file");

    // add to file
    write!(file, "{}", input).expect("Failed to write to file");
}

fn open_file() -> File {
    let file = File::open("todo.txt").expect("Failed to open file");
    file
}

struct FileAndReader {
    file: File,
    reader: BufReader<File>,
}

fn show_list_todo() -> FileAndReader {
    let file = open_file();
    let reader = io::BufReader::new(file);

    FileAndReader { file, reader }
}

fn list_todo() {
    println!("list todo");
    let file_and_reader = show_list_todo();

    for (line_number, line) in file_and_reader.reader.lines().enumerate() {
        println!("{}: {}", line_number + 1, line.unwrap());
    }
}

fn mark_todo_as_done() {
    println!("mark todo as done");
}

fn remove_todo() {
    let mut file_and_reader = show_list_todo();
    let input = input_user().parse::<usize>().unwrap();

    for (line_number, line) in file_and_reader.reader.lines().enumerate() {
        if line_number + 1 != input {
            write!(file_and_reader.file, "{}", line.unwrap()).expect("Failed to write to file");
        }
    }

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
    let file_path = "todo.txt";

    loop {
        println!("Enter a command: ");
        if let Some(cmd) = read_command() {
            match cmd.as_str() {
                "add" => {
                    add_todo(file_path);
                }
                "list" => list_todo(),
                "done" => mark_todo_as_done(),
                "remove" => remove_todo(),
                "quit" => break,
                _ => println!("unknown command"),
            }
        }
    }

    println!("quit");
}
