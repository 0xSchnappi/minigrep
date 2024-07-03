// main.rs
use std::{env, fs};
fn main() {
    let args: Vec<String> = env::args().collect();
    // 所有的用户输入不可信！不可信！不可信！
    dbg!(&args);
    if args.len() < 2 {
        let usage = "usage:\
        minigrep --searchstring example-filename.txt\
        ";
        println!("{}", usage);
        return;
    }

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}
