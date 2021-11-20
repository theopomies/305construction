use std::fs::File;

const HELP_MESSAGE: &str = r#"USAGE
    ./305construction file

DESCRIPTION
    file    file describing the tasks"#;

pub struct Arguments {
    file: File,
}

impl Arguments {
    pub fn from_args() -> Self {
        let args = std::env::args().skip(1).collect::<Vec<_>>();
        match &args[..] {
            [arg, ..] if arg == "-h" || arg == "--help" => {
                println!("{}", HELP_MESSAGE);
                std::process::exit(0);
            }
            [file] => Self {
                file: match File::open(file) {
                    Ok(file) => file,
                    Err(err) => {
                        eprintln!("{}", HELP_MESSAGE);
                        eprintln!("{}", err);
                        std::process::exit(84);
                    }
                },
            },
            _ => {
                eprintln!("{}", HELP_MESSAGE);
                std::process::exit(84);
            }
        }
    }

    pub fn get_file(&mut self) -> &mut File {
        &mut self.file
    }
}
