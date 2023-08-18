use std::io::{self, Write};
use termion::event::Key;
use termion::input::TermRead;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    loop {
        
        print!("{} ", "$");
        stdout.flush()?;
        

        let stdin = io::stdin(); // Create a new instance of io::Stdin for each iteration

        for key in stdin.keys() {
            match key? {
                Key::Ctrl('x') => {
                    println!("\nExiting program");
                    return Ok(());
                }
              
                Key::Char('\n') => {
                    // Process the input buffer here
                    if !buffer.is_empty() {

                        //println!("You entered: {}", buffer);
                        
                        // Clear the buffer after processing
                        add(buffer.clone());
                        buffer.clear();
                        
                       // println!("{} ", "$");
                    }
                }
                Key::Char(c) => {
                    buffer.push(c);
                }
                _ => {}
            }
        }
        print!("{} ", "$");

    }
}


fn add(a : String) -> () {
    println!("Func returned ---> {}", a);
    print!("{} ", "$");


}