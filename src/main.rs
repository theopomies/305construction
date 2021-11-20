mod args;
use args::Arguments;

mod scheduler;

fn main() {
    let args = &mut Arguments::from_args();
    let file = args.get_file();
    match scheduler::Scheduler::try_from(file) {
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(84);
        }
        Ok(mut scheduler) => match scheduler.execute() {
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(84);
            }
            _ => println!("{}", scheduler),
        },
    };
}
