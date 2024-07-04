// main.rs
use minigrep::Config;
use std::{env, process};
fn main() {
    /*
     * main函数功能
     * 1.解析命令行参数
     * 2.初始化其他配置
     * 3.调用lib.rs中的run函数，以启动逻辑代码的运行
     * 4.如果run返回一个错误，需要对该错误进行处理
     */
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

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments:{err}");
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
