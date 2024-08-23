use clap::Parser;
use pwdr::generate_password;

/// Program for some password (text) manipulations (repeat, alternate direction).
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    password: String,

    /// The number of times to repeat the password
    #[arg(short('r'), long, default_value_t = 1)]
    repeat: i8,

    /// If the repetitions of the password should alternate their direction (e.g. abc -> abccba)
    #[arg(short('a'), long)]
    alternate_direction: bool,
}

fn main() {
    let args = Args::parse();
    let repeater = match args.repeat.try_into() {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    let password = generate_password(&args.password, repeater, args.alternate_direction);
    println!("{}", password);
}
