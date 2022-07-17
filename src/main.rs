use colored::Colorize;
use rand::Rng;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    i_o::cls();

    let mut words = Vec::new();
    read_file_to_vector("./data/corncob_lowercase.txt", &mut words);

    let mut pw: Vec<String> = Vec::new();
    for _i in 0..3 {
        let mut rng = rand::thread_rng();
        let val = rng.gen_range(1, words.len());
        pw.push(words[val].clone());
    }

    let pwstr = format!("{}.{}.{}", pw[0], pw[1], pw[2]);

    let title = format!("twpw {}", env!("CARGO_PKG_VERSION"));
    let frm = i_o::Frame {
        title: title.to_string(),
        title_color: "blue".to_string(),
        x: 0,
        y: 0,
        w: pwstr.len() as u16 + 4,
        h: 2,
    };
    frm.display();

    i_o::cmove(2, 1);
    print!("{}", pwstr.green().bold());

    i_o::cmove(0, 4);

}

fn read_file_to_vector(filename: &str, vector: &mut Vec<String>) {
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                vector.push(ip);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Could not find data file");
    Ok(io::BufReader::new(file).lines())
}
