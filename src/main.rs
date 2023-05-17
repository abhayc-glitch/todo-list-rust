use std::io;
use std::env;
use std::string;

struct TodoItem{
    name: String,
    completed: char
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        return TodoItem{
            name: name,
            completed: ' '
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let userCommand = args[1].clone();

    let todoItem = TodoItem{
        name: "Finish the Rust Tutorial".to_string(),
        completed: ' '
    };

    let mut todos: Vec<TodoItem> = vec![];

    if userCommand == "get" {
        for i in todos {
            println!("[{}] - {}", i.completed, i.name)
        }
    } else if userCommand == "add" {
        println!("Adding task to list...");
        let user_task = args[2].clone();
        let new_todo = TodoItem::new(user_task);
        todos.push(new_todo);


    } else if userCommand == "remove" {
        println!("Removing task from list...")
    } else if userCommand ==  "Finish" {

    }


    println!("{:?}", args)

}
