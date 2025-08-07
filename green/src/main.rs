use std;
use std::env;
use std::collections;
use std::fs::File;
use std::io::Read;
use green_lib;
use green_lib::lexer::Lexer;

fn print_help(){
    println!("GREENLang\n---------------\ngreen ver: {}\ngreen-lib ver: {}\n---------------\n", env!("CARGO_PKG_VERSION"), green_lib::get_version());
    println!("Usage: green <action> <param>");
    println!("-h | --help - print help message");
    println!("\nBuilding\ngreen build/make <params>");
    println!("-p <path to file> - path to file to build(required)");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().count() == 1{
        print_help();
        return;
    }

    let command: &String = &args[1];
    let mut params: collections::HashMap<String, &String> = collections::HashMap::new();

    let mut i : usize = 2;
    while i  < args.iter().count(){
        if args[i] == "-h" || args[i] == "--help" || args[i] == "-v" || args[i] == "--version"{
            print_help();
            return;
        }

        if args[i].starts_with("-") && i + 1 < args.iter().count() {
            params.insert(args[i].clone(), &args[i + 1]);
            i += 1;
        }
        else{
            panic!("Incorrect arguments format!")
        }
        i += 1;
    }

    if command == "build" || command == "make" {
        let mut source: String = String::new();

        {
            if !params.contains_key("-p") {
                panic!("Path to file is required!");
            }
            let path: String = params.get("-p").unwrap().to_string();

            let mut source_file: File = File::open(path).expect("Can't open file");
            source_file.read_to_string(&mut source).expect("Something went wrong reading the file");
        }
        source = source.trim().trim_start_matches('\u{FEFF}').to_string();

        let mut lexer: Lexer = Lexer::new(&source);
        let _tokens: Vec<green_lib::token::Token> = lexer.tokenize();
        if !lexer.is_success(){
            for msg in lexer.errors{
                println!("{}", msg);
            }
            return;
        }
    }
    else {
        panic!("Command {} not found!", command);
    }

}