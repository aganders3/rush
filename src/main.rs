use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::{exit, Command};
use std::str::FromStr;

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
            // split the command into parallel tasks, collect result
            let mut children = Vec::new();
            for cmd in command.split("&") {
                // split into command + args
                let split_cmd: Vec<&str> = cmd.trim().split_whitespace().collect();
                // skip if there is no command...
                if split_cmd.len() > 0 {
                    // check if it's a builtin command, and handle it
                    if let Ok(builtin) = split_cmd[0].parse::<Builtin>() {
                        builtin.run(&split_cmd[1..]);
                    // if it's not a builtin we just run it in a subproc
                    } else {
                        let result = Command::new(split_cmd[0])
                            .args(&split_cmd[1..])
                            .spawn();
                        match result {
                            Ok(child) => children.push(child),
                            Err(error) => println!(
                                "Error running command {}: {}",
                                split_cmd[0],
                                error,
                            ),
                        }
                    }
                }
            }
            // join all spawned (parallel) processes
            for result in children.into_iter().map(|mut child| child.wait()) {
                match result {
                    Ok(status) => println!("Proc exited with {}", status),
                    Err(error) => println!("Error in proc {}", error),
                }
            }
            print!("\n");
        }
    }
}

#[derive(Debug)]
enum Builtin {
    CD,
    Exit,
}

impl FromStr for Builtin {
    type Err = String;
    fn from_str(cmd: &str) -> Result<Builtin, Self::Err> {
        match cmd {
            "cd" => Ok(Builtin::CD),
            "exit" => Ok(Builtin::Exit),
            _ => Err(Self::Err::from("Not a builtin command")),
        }
    }
}

impl Builtin {
    fn run(&self, args: &[&str]) {
        println!("Builtin: {:?}!", self);
        match self {
            Builtin::CD => {
                let new_pwd = Path::new(args[0]);
                if env::set_current_dir(new_pwd).is_ok() {
                    println!("Changed direcotry to {}", new_pwd.display());
                } else {
                    println!("Not a valid directory! CWD unchanged.");
                }
            }
            Builtin::Exit => exit(0),
        }
    }
}
