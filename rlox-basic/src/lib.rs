use callable::StorableThings;
use interpretor::Interpretor;
use resolver::Resolver;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use parser::*;
use scanner::*;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
pub mod tokens;
mod interpretor;
mod parser;
mod scanner;
mod environment;
mod callable;
mod resolver;
#[derive(Debug)]
pub enum MainError {
    Standard(Box<dyn Error>),
    Language(Option<StorableThings>),
    ParseError((i32, String, String)),
    RuntimeError((i32, String, String)),
    ScanningError((i32, String, String)),
    ResolvingError((i32, String, String))
}
impl fmt::Display for MainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MainError::Standard(e) => write!(f, "Standard Error: {}", e),
            MainError::Language(_) => { 
                write!(
                    f,
                    "Language Error"
                )
            }
            MainError::ParseError((line, place, message)) => {
                write!(
                    f,
                    "Parse Error:: [line {}] Error  {}: {}",
                    line, place, message
                )
            }
            MainError::RuntimeError((line, place, message)) => {
                write!(
                    f,
                    "Runtime Error:: [line {}] Error  {}: {}",
                    line, place, message
                )
            }
            MainError::ScanningError((line, place, message)) => {
                write!(
                    f,
                    "Scanning Error:: [line {}] Error  {}: {}",
                    line, place, message
                )
            },
            MainError::ResolvingError((line, place, message)) => {
                write!(
                    f,
                    "Resolving Error:: [line {}] Error  {}: {}",
                    line, place, message
                )
            }
        }
    }
}
impl Error for MainError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MainError::Standard(e) => Some(&**e),
            MainError::Language(_) => None,
            MainError::ParseError(_) => None,
            MainError::RuntimeError(_) => None,
            MainError::ScanningError(_) => None,
            MainError::ResolvingError(_) => None
        }
    }
}
pub struct Lox {
    had_error: bool,
    had_runtime_error: bool,
    interpretor: Interpretor,
    help:String,
}
impl Lox {
    pub fn new() -> Lox {
        Lox {
            had_error: false,
            had_runtime_error: false,
            interpretor: Interpretor::new(),
            help:String::from(".exit; -- For exiting the REPL terminal.\nPress Ctrl+C to abort the current process.")
        }
    }
    pub fn run_file(&mut self, filepath: &String) {
        //Opening the file
        let mut file = match File::open((*filepath).clone()) {
            Ok(file) => file,
            Err(e) => {
                panic!("Erorr opening the file:{}", e);
            }
        };
        //Reading the contents of the file
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => {
                //we will manage these errors later
                self.run(&contents, false);
            }
            Err(e) => {
                self.report(MainError::Standard(Box::new(e)));
            }
        };
        // Indicate an error in the exit code.
        if self.had_error {
            std::process::exit(65)
        };
        if self.had_runtime_error {
            std::process::exit(70)
        };
    }
    pub fn run_prompt(&mut self, running: Arc<AtomicBool>) {
        println!(
            "Welcome to r_lox_basic version[{}].\nType \".help\" for more information.",
            env!("CARGO_PKG_VERSION")
        );
        while running.load(Ordering::SeqCst) {
            print!(">> ");
            io::stdout().flush().expect("Failed to flush stdout");

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let input = input.trim().to_string();
                    self.run(&input, true);
                    self.had_error = false;
                }
                Err(e) => {
                    eprintln!("Failed to read line: {}", e);
                    break;
                }
            }
        }
        println!("Exiting prompt due to SIGINT.");
    }
    fn run(&mut self, contents: &String,repl:bool) {
        if contents == ".help"{
            println!("{}", self.help);
            return;
        }
        if contents == ".exit"{
            std::process::exit(0);
        }
        let mut scanner = Scanner::new(contents);
        let tokens = match scanner.scan_tokens() {
            Ok(tokens) => tokens,
            Err(e) => {
                self.report(e);
                std::process::exit(65);// Exit the function early in case of scanning error
            }
        };
        let mut parser = Parser::new(tokens.clone(), repl);
        let mut expr = parser.parse();
        let mut resolver = Resolver::new(&mut self.interpretor);
        match resolver.resolve(expr.clone()){
            Ok(_)=>(),
            Err(e)=> {
                // println!("Here");
                self.report(e);
                std::process::exit(100);// Runtime Error
            }
        }
        match self.interpretor.interpret(&mut expr){
            Ok(_)=>(),
            Err(e)=> {
                self.report(e);
                std::process::exit(100);// Runtime Error
            }
        }
    }

    pub fn report(&mut self, e: MainError) {
        eprint!("{}", e);
        match e {
            MainError::RuntimeError(_) => {
                self.had_runtime_error = true;
            }
            MainError::ParseError(_) => {
                self.had_error = true;
            }
            _ => {
                self.had_runtime_error = true;
            }
        }
    }
}
