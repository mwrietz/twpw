#![allow(dead_code)]

use colored::Colorize;
use crossterm::{cursor, execute};
use getch::Getch;
use std::env;
use std::io::{stdout, Write};

pub fn cls() {
    std::process::Command::new("clear").status().unwrap();
}

pub fn cmove(x: usize, y: usize) {
    execute!(stdout(), cursor::MoveTo(x as u16, y as u16)).unwrap();
}

pub fn get_prog_name() -> String {
    let prog_name = env::current_exe()
        .expect("Can't get the exec path")
        .file_name()
        .expect("Can't get the exec name")
        .to_string_lossy()
        .into_owned();
    prog_name
}

pub fn horiz_line(color: &str) {
    for _i in 0..80 {
        print!("{}", "─".color(color).bold());
    }
    println!();
}

pub fn pause() {
    let (w, h) = tsize();
    let clear_message = "                            ";
    let message = "Press any key to continue...".blue();
    let message_len: usize = message.len();
    cmove((w - message_len) / 2, h - 2);
    print!("{}", message);
    std::io::stdout().flush().unwrap();
    let g = Getch::new();
    let _keypress = g.getch().unwrap();
    cmove((w - message_len) / 2, h - 2);
    print!("{}", clear_message);
}

pub fn print_title(title_string: &str, color: &str) {
    println!();
    for c in title_string.chars() {
        print!(" ");
        print!("{}", c.to_string().color(color).bold());
    }
    println!();
    horiz_line(color);
    println!();
}

pub fn splash_screen(line1: &str, line2: &str) {
    //const VERSION: &str = env!("CARGO_PKG_VERSION");

    cls();
    let (width, height) = tsize();

    let line1_length: usize = line1.len();
    cmove(width / 2 - line1_length / 2, height / 2 - 1);
    println!("{}", line1.bold());

    let line2_length: usize = line2.len();
    cmove(width / 2 - line2_length / 2, height / 2 + 1);
    println!("{}", line2);

    execute!(stdout(), cursor::Hide).unwrap();

    // pause for splash screen
    //let one_sec = std::time::Duration::from_millis(1000);
    let dur = std::time::Duration::new(2, 0);
    std::thread::sleep(dur);
    cls();

    execute!(stdout(), cursor::Show).unwrap();
}

//
// TermStat usage:
// let mut termstat = TermStat::default();
//

pub struct TermStat {
    pub line_count: usize,
    pub width: usize,
    pub height: usize,
}

impl Default for TermStat {
    fn default() -> TermStat {
        let (w, h) = tsize();
        TermStat {
            line_count: 0,
            width: w,
            height: h,
        }
    }
}

impl TermStat {
    pub fn line_check(&mut self) {
        self.line_count += 1;
        if self.line_count > (self.height - 5) {
            pause();
            self.line_count = 0;
            cls();
            cmove(0, 0);
        }
    }
}

pub fn timestamp() -> String {
    let now = chrono::Local::now();
    now.to_string()
}

pub fn tsize() -> (usize, usize) {
    let size = crossterm::terminal::size();
    let (w, h) = match size {
        Ok((w, h)) => (w, h),
        Err(error) => panic!("tsize error: {:?}", error),
    };
    (w as usize, h as usize)
}
