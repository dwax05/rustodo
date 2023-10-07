use ncurses::*;
use std::{
    io::Write,
    fs::{File, self}, ops::Index,
};

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

type Id = usize;

#[derive(Default)]
struct Ui {
    list_curr: Option<Id>,
    row: usize,
    col: usize,
}

impl Ui {
    fn begin(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }

    fn begin_list(&mut self, id: Id) {
        assert!(self.list_curr.is_none(), "Cannot create a nested list");
        self.list_curr = Some(id);
    }

    fn list_element(&mut self, label: &str, id: Id) -> bool {
        let id_curr = self
            .list_curr
            .expect("Cannot create elements outside of list");

        self.label(label, {
            if id_curr == id {
                HIGHLIGHT_PAIR
            } else {
                REGULAR_PAIR
            }
        });

        return false;
    }

    fn label(&mut self, text: &str, pair: i16) {
        mv(self.row as i32, self.col as i32);
        attron(COLOR_PAIR(pair));
        addstr(text);
        attroff(COLOR_PAIR(pair));
        self.row += 1;
    }

    fn end_list(&mut self) {
        self.list_curr = None;
    }

    fn end(&mut self) {}
}

enum Tab {
    Todo,
    Done,
}

impl Tab {
    fn toggle(&self) -> Self {
        match self {
            Tab::Todo => Tab::Done,
            Tab::Done => Tab::Todo,
        }
    }
}

fn list_up(list_curr: &mut usize) {
    if *list_curr > 0 {
        *list_curr -= 1;
    }
}

fn list_down(list: &Vec<String>, list_curr: &mut usize) {
    if *list_curr + 1 < list.len() {
        *list_curr += 1;
    }
}

fn list_transfer(list_dst: &mut Vec<String>, list_src: &mut Vec<String>, list_src_curr: &mut usize) {
    if *list_src_curr < list_src.len() {
        list_dst.push(list_src.remove(*list_src_curr));
        if *list_src_curr >= list_src.len() && list_src.len() > 0 {
            *list_src_curr = list_src.len() - 1;
        }
    }
}

// TODO: edit items

fn main() -> std::io::Result<()>{
    
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut quit = false;
    let (mut todos, mut dones) = load_todos();
    let mut todo_curr: usize = 0;
    let mut done_curr: usize = 0;
    let mut tab = Tab::Todo;

    let mut ui = Ui::default();
    while !quit {
        erase();

        ui.begin(0, 0);
        {
            match tab {
                Tab::Todo => {
                    ui.label("[TODO] DONE ", REGULAR_PAIR);
                    ui.label("------------", REGULAR_PAIR);
                    ui.begin_list(todo_curr);
                    for (index, todo) in todos.iter().enumerate() {
                        ui.list_element(&format!("- [ ] {}", todo), index);
                    }
                    ui.end_list();
                }
                Tab::Done => {
                    ui.label(" TODO [DONE]", REGULAR_PAIR);
                    ui.label("------------", REGULAR_PAIR);
                    ui.begin_list(done_curr);
                    for (index, done) in dones.iter().enumerate() {
                        ui.list_element(&format!("- [x] {}", done), index);
                    }
                    ui.end_list();
                }
            }
        }
        ui.end();

        refresh();

        let key = getch();
        match key as u8 as char {
            'q' => {
                quit = true;
            },
            'k' => match tab {
                Tab::Todo => list_up(&mut todo_curr),
                Tab::Done => list_up(&mut done_curr),
            },
            'j' => match tab {
                Tab::Todo => list_down(&todos, &mut todo_curr),
                Tab::Done => list_down(&dones, &mut done_curr),
            },
            'd' => match tab {
                Tab::Todo => {
                    if todos.len() != 0 {
                        todos.remove(todo_curr);
                    }
                },
                Tab::Done => {
                    if dones.len() != 0 {
                        dones.remove(done_curr);
                    }
                },
            },
            'n' => {
                match tab {
                    Tab::Done => {},
                    Tab::Todo => {
                        let mut new_todo = String::new();
                        let mut exit = false;
                        while !exit {
                            let key = getch() as u8 as char;
                            match key {
                                '\n' => exit = true,
                                _ => new_todo.push(key),
                            }
                        }
                        todos.push(new_todo);
                    },
                }
            },
            '\n' => match tab {
                Tab::Todo => list_transfer(&mut dones, &mut todos, &mut todo_curr),
                Tab::Done => list_transfer(&mut todos, &mut dones, &mut done_curr),
            },
            '\t' => {
                tab = tab.toggle();
            },
            _ => {}
        }
    }

    save_todos(&todos, &dones)?;
    endwin();
    Ok(())
}

fn load_todos() -> (Vec<String>, Vec<String>) {

    let file_contents = fs::read_to_string("TODO").expect("Could not read file");

    let mut todos = Vec::<String>::new();
    let mut dones = Vec::<String>::new();

    for line in file_contents.lines() {
        let (status, task) = line.split_at(5);
        if status == "TODO " {
            todos.push(task.to_string());
        }else{
            dones.push(task.to_string());
        }
    }

    (todos, dones)
}

fn save_todos(todos: &Vec<String>, dones: &Vec<String>) -> std::io::Result<()>{

    fs::remove_file("TODO")?;
    let mut file = File::create("TODO")?;

    for todo in todos {
        writeln!(&mut file, "TODO {}", todo)?;
    }
    for todo in dones {
        writeln!(&mut file, "DONE {}", todo)?;
    }

    Ok(())
}
