use clap::Parser;

/// ccwhich - Locate a program file in the user's path
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// List of command names
    #[arg()]
    programs: Vec<String>,
}

fn main() {
    let args = Args::parse();

    for  program in args.programs {
        println!("{:?}", program);
    }
}