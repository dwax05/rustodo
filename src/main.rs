mod todo;
use todo::*;

fn main() {
    let mut todo_list: TodoList = TodoList::new();

    let file: String = String::from("TODO.txt");
    todo_list.read_todos_from_file(file);

    todo_list.print_todos();
    println!("{}", todo_list.get_task_index("apcsa").unwrap());
}
