use std::io::{self, Write};


fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    loop {
        print!("$ "); // Print the prompt
        stdout.flush()?;

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                print!("{}", input);
                break
            }
            Ok(_)=> {
                if input.is_empty() {
                    continue; // Skip processing and re-prompt on empty input
                }
        
                let trimmed_input = input.trim(); // Remove leading/trailing whitespace
        
                if let Some(key) = parse_ctrl_key(&trimmed_input) {
                    if key == 'x' {
                        println!("Exiting program");
                        break;
                    }
                } else {
                    println!("You entered: {}", trimmed_input);
                   // add(trimmed_input.to_string());
                }
            }
            Err(_)=>{
                return  Ok(());
            }
        }



    }

    Ok(())
}

// fn add(a: String) {
//     println!("Func returned ---> {}", a);
// }

fn parse_ctrl_key(input: &str) -> Option<char> {
    let mut chars = input.chars();
    if let Some('^') = chars.next() {
        if let Some(ctrl_char) = chars.next() {
            if chars.next().is_none() && ctrl_char.is_ascii_lowercase() {
                return Some(ctrl_char);
            }
        }
    }
    None
}
