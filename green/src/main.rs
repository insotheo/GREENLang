use std;
use std::env;
use std::collections;
use green_lib;

fn print_help(){
    println!("GREENLang\n---------------\ngreen ver: {}\ngreen-lib ver: {}\n---------------\n", env!("CARGO_PKG_VERSION"), green_lib::get_version());
    println!("Usage: green <param>");
    println!("-h | --help - print help message");
    println!("-b <path to file> - build file");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().count() == 1{
        print_help();
        return;
    }

    let mut params: collections::HashMap<String, String> = collections::HashMap::new();

    for i in 1..args.iter().count(){
        if args[i] == "-h" || args[i] == "--help" || args[i] == "-v" || args[i] == "--version"{
            print_help();
            return;
        }

        if args[i].starts_with("-") && i + 1 < args.iter().count() {
            params.insert(args[i].clone(), args[i + 1].clone());
        }
        else{
            panic!("Incorrect arguments format!")
        }
    }



}