use std::process::Command;
use std::io::{self, Write};

fn main() {
    loop {
        // print the prompt
        print!("rush % ");
        io::stdout().flush().unwrap();

        // get the input from the user
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error reading command!");

        // split the command into serial tasks
        for command in user_input.split(";") {
            // split the command into parallel tasks
            // TODO: spawn these instead, and process the results in a separate
            // thread
            for cmd in command.split("&") {
                // split into command + args
                let split_cmd: Vec<&str> = cmd.trim().split_whitespace().collect();
                // skip if there is no command...
                if split_cmd.len() > 0 {
                    let status = Command::new(split_cmd[0])
                        .args(&split_cmd[1..])
                        .status();
                    // let exit_status = runnable.status();
                    match status {
                        Ok(status) => print!("Exit code {:?}!", status.code()),
                        Err(error) => print!("Error {}!", error),
                    }
                }
            }
            print!("\n");
        }
    }
}

// fn run(&str command) -> Result<u8> {
    // 
// }