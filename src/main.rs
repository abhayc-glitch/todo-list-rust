
use std::env;


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

    fn print(&self) {
        for i in &self.list {
            println!("[{}] - {}", i.completed, i.name)
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let user_command = args[1].clone();
    let mut todos = TodoList::new();

    todos.add_task("Finish rust tutorial".to_string());


    if user_command == "get" {
        todos.print();

    } else if user_command == "add" {
        println!("Adding task to list...");

        let user_task = args[2].clone();
        let new_todo = TodoItem::new(user_task);

        todos.add_task(new_todo.name);

        println!("Here is the updated list...");
        todos.print();
    }


}
