use ncurses::*;
use std::{
    fs::File,
    io::{BufReader, BufRead},
};

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

fn main() {

    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut todos: Vec<String> = Vec::new();
    let mut done: Vec<String> = Vec::new();

    let file = File::open("TODO.txt").expect("Could not read file");

    // Create a BufReader to read lines from the file
    let reader = BufReader::new(file);

    // Iterate over each line in the file
    for line in reader.lines() {
        match line {
            Ok(line_content) => {
                todos.push(line_content);
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }

    let mut current_todo: usize = 0;

    let mut exit = false;

    while !exit {

        for (index, task) in todos.iter().enumerate() {
            let pair = {
                if current_todo == index {
                    HIGHLIGHT_PAIR
                } else {
                    REGULAR_PAIR
                }
            };

            attron(COLOR_PAIR(pair));
            mv(index as i32, 1);
            addstr(&task);
            attroff(COLOR_PAIR(pair));
        }

        refresh();

        let key = getch();

        match key as u8 as char {
            'q' => exit = true,
            'k' => if current_todo > 0 {
                current_todo -= 1;
            }
            'j' => if current_todo < todos.len() - 1 {
                current_todo += 1;
            }
            'd' => {
                done.push(todos[current_todo].clone());
                todos.remove(current_todo);
                clear();
                refresh();
            },
            _ => {},
        }
    }
    endwin();
    curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
}
