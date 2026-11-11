#![allow(warnings)]

mod cursor;
use crossterm::event::KeyEventKind;
use cursor::{Cursor, move_left, move_right, move_up, move_down};

mod screen;
use screen::{draw_file};

use std::{arch::x86_64::_mm512_maskz_cvt_roundepu64_ph, fs};

use std::env::{self, args};

use crossterm::{
    ExecutableCommand,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
    cursor::{MoveTo, Hide, Show},
    event::{read, Event, KeyCode, KeyModifiers},
    terminal
};
use std::io::stdout;
use std::path::Path;

fn draw_cursor(cursor: &Cursor, stdout: &mut std::io::Stdout) -> std::io::Result<()> {

    stdout.execute(MoveTo(cursor.x as u16 , cursor.y as u16))?;

    Ok(())

}

fn insert_char(file: &mut Vec<String>, line_y: usize, line_x: usize, ch: char) {

    if file.is_empty() {
        file.push(String::new());
    }

    while file.len() <= line_y {
        file.push(String::new());
    }

    file[line_y].insert(line_x, ch);

}

fn delete(file: &mut Vec<String>, line_y: usize, line_x: usize, cursor: &mut Cursor) -> i32 {

    if line_y == 0 && line_x == 0 {
        return 1;
    }

    if line_x > 0 && !file[0].is_empty() {

        file[line_y].remove(line_x - 1);
        return 0
    }

    if line_x == 0 {

        let current_line = file.remove(line_y);      

        // Append it to the end of the previous line
        file[line_y - 1].push_str(&current_line);   

        // Update cursor position
        cursor.x = file[line_x].len() + 1;
        cursor.y -= 1;

        return 0
    }


    0
  
}

fn insert_newline(file: &mut Vec<String>, line_y: usize, line_x: usize) -> i32 {
    let line = file[line_y].clone();
    let (first, second) = line.split_at(line_x);
    
    file[line_y] = first.to_string();
    file.insert(line_y + 1, second.to_string());

    return second.len() as i32
}

fn printUsage() {

    println!("knead <file-path>");

}

fn loadFile(path: &str) -> std::io::Result<Vec<String>> {

    let file = fs::read_to_string(path)?;
    Ok(file.lines().map(String::from).collect())

}

fn main() -> std::io::Result<()> {

    let mut stdout = stdout();

    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        printUsage();
        std::process::exit(1);
    }

    let filePath = &args[1];

    let mut file = match loadFile(filePath) {

        Ok(file) => file,
        Err(e) => {
            eprintln!("Could not open file: {}", e);
            std::process::exit(1);
        } 

    };

    let mut cursor = Cursor { x: 0, y: 0 };

    draw_file(&file, &mut stdout)?;

    draw_cursor(&cursor, &mut stdout)?;

    stdout.execute(Show)?;

    loop {

        if let Event::Key(key_event) = read()? {

            match key_event.code {

               KeyCode::Char(c) if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                    if c == 'q' {
                         break;
                    }
                    if c == 's' {
                        fs::write(filePath, file.join("\n"));
                    }
                }
                KeyCode::Left => move_left(&mut cursor),
                KeyCode::Right => move_right(&mut cursor, &file),
                KeyCode::Up => move_up(&mut cursor, &file),
                KeyCode::Down => move_down(&mut cursor, &file),
                KeyCode::Char(c) => {

                    insert_char(&mut file, cursor.y, cursor.x, c);
                    move_right(&mut cursor, &file);

                },
                KeyCode::Enter => {

                    let lenght = insert_newline(&mut file, cursor.y, cursor.x);
                    move_down(&mut cursor, &file);
                    for i in 0..lenght {

                        move_left(&mut cursor);

                    }

                }
                KeyCode::Backspace => {

                   let rel:i32 = delete(&mut file, cursor.y, cursor.x, &mut cursor);
                   if rel == 0 {

                     move_left(&mut cursor);

                   }
                   

                }
                _ => {}

            }

            draw_file(&file, &mut stdout)?;

            draw_cursor(&cursor, &mut stdout)?;            

        }

    }
    
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
    
}
