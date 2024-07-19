use rlox_basic::Lox;
use signal_hook::consts::SIGINT;
use signal_hook::iterator::Signals;
use std::env;
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool};
use std::sync::Arc;
use std::thread;

fn main() {
    // Set up an atomic flag to indicate whether to continue running
    let running = Arc::new(AtomicBool::new(true));
    let _r = Arc::clone(&running);

    // Set up a separate thread to handle SIGINT
    thread::spawn(move || {
        let mut signals = Signals::new(&[SIGINT]).expect("Failed to create signals iterator");
        for sig in signals.forever() {
            if sig == SIGINT {
                print!("\nCaught KeyboardInterrupt (SIGINT), but continuing...\n>> ");
                io::stdout().flush().expect("Failed to flush stdout");
            }
        }
    });

    // Collect the command-line arguments
    let args: Vec<String> = env::args().collect();
    let mut interpreter = Lox::new();

    if args.len() > 2 {
        // If args are too many then exit the code
        eprintln!("Usage: jlox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        // Run file
        println!("Running file: {}", &args[1]);
        interpreter.run_file(&args[1]);
    } else {
        // Run prompt and handle Ctrl+C
        interpreter.run_prompt(running);
    }
}
