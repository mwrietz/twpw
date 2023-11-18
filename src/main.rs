use colored::*;
use rand::Rng;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

mod tui_frm;
mod tui_gen;

fn main() {
    tui_gen::cls();

    let data_path: String = dirs::home_dir()
        .expect("Cannot get home path")
        .join("Syncthing/active/computer/data/twpw/corncob_lowercase.txt")
        .into_os_string()
        .into_string()
        .unwrap();

    let mut words = Vec::new();

    //read_file_to_vector(data_path, &mut words);
    file_to_vector(&data_path, &mut words);

    // pick three random words and put in vector
    let mut pwv: Vec<String> = Vec::new();
    for _i in 0..3 {
        let mut rng = rand::thread_rng();
        let val = rng.gen_range(1, words.len());
        pwv.push(words[val].to_string().clone());
    }

    // format password string
    let pw = format!("{}.{}.{}", pwv[0], pwv[1], pwv[2]);

    // draw frame of size required for password
    let frm = tui_frm::Frame {
        title: &format!(
            "{} v{}",
            tui_gen::get_prog_name(),
            env!("CARGO_PKG_VERSION")
        ),
        title_color: "blue",
        frame_color: "white",
        x: 0,
        y: 0,
        w: pw.len() + 4,
        h: 2,
    };
    frm.display();

    // display the password in the frame
    tui_gen::cmove(2, 1);
    print!("{}", pw.green().bold());

    tui_gen::cmove(0, 4);
}

fn file_to_vector(file_path: &str, line_vector: &mut Vec<String>) {
    let file = File::open(file_path).unwrap_or_else(|_| panic!("Could not find data file: {}", file_path));
    let lines = io::BufReader::new(file).lines();

    for line in lines.into_iter() {
        match line {
            Ok(l) => line_vector.push(l),
            Err(e) => {
                panic!("Error msg: {}", e);
            }
        }
    }
}
