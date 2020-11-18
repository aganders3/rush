use std::env;
use std::path::Path;
use std::process::exit;
use std::str::FromStr;

#[derive(Debug)]
pub enum Builtin {
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
    pub fn run(&self, args: &[&str]) {
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
