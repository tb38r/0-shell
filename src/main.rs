use std::io::{self, Write};
use termion::event::Key;
use termion::input::TermRead;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    loop {
        print!("$ "); // Print the prompt
        stdout.flush()?;

        let stdin = io::stdin();
        let keys = stdin.keys(); // Capture the keys iterator outside of the loop

        for key in keys {
            match key? {
                Key::Ctrl('x') => {
                    println!("\nExiting program");
                    return Ok(());
                }
                Key::Ctrl(_) => {
                   // print!("$ ");

                    // Ignore other Ctrl key combinations
                }
                Key::Char('\n') => {
                    // Process the input buffer here
                    if !buffer.is_empty() {
                        println!("You entered: {}", buffer);

                        // Call the add function with the input buffer
                        add(buffer.clone());

                        buffer.clear();
                    }
                }
                Key::Char(c) => {
                    buffer.push(c);
                }
                _ => {}
            }
            // Print the prompt on the same line
            print!("$ ");
            stdout.flush()?;
        }
    }
}

fn add(a: String) {
    println!("Func returned ---> {}", a);
}
