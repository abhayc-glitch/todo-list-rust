
use std::env;
use std::convert::TryInto;


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

struct TodoList{
    list: Vec<TodoItem>
}

// Implements new todolist function
impl TodoList {
    fn new() -> TodoList{   
        return TodoList{list: Vec::new()};
    }

    fn add_task(&mut self, name: String) {
        let new_todo = TodoItem::new(name);
        self.list.push(new_todo);
    }

    fn remove(&mut self, index: usize) {
        self.list.remove(index);
    }

    fn done(&mut self, index: usize) {

        if self.list[index].completed == ' ' {
            self.list[index].completed = 'x';
        } else {
            self.list[index].completed = ' ';
        }
    }

    fn print(&self) {
        for i in &self.list {
            println!("[{}] - {}", i.completed, i.name)
        }
    }
}

enum Command {
     get,
    add_task(String),
    remove(usize),
    done(usize),
}


fn main() {
    let args: Vec<String> = env::args().collect();
    
    let user_command = match args[1].as_str() {
        "get" => Command::get,
        "add" => Command::add_task(args[2].clone()),
        "done" => Command::done(args[2].parse().expect("error converting to integer")),
        "remove" => Command::remove(args[2].parse().expect("error converting to integer")),
        _ => panic!("You must provide an accepted command")
    };

    let mut todos = TodoList::new();

    todos.add_task("Finish rust tutorial".to_string());


    match user_command {
        Command::get => todos.print(),
        Command::add_task(user_task) => {
            println!("Adding task to list...");

            todos.add_task(user_task);

            println!("Here is the updated list...");
            todos.print();
        }
        Command::remove(index) => {
            let adjusted_index = index - 1;
            todos.remove(adjusted_index);

            println!("Here is the updated list...");
            todos.print();
        },
        Command::done(index) => {
            let adjusted_index = index - 1;
            todos.done(adjusted_index);

            println!("Here is the updated list...");
            todos.print();
        },
    }
}
