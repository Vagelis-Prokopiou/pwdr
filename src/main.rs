use clap::Parser;
use pwdr::generate_password;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The initial password
    #[arg(short('p'), long)]
    password: String,

    /// The number of times to repeat the initial password
    #[arg(short('r'), long, default_value_t = 1)]
    repeat: i8,

    /// If the repetitions of the password should alternate their direction (e.g. abc -> abccba)
    #[arg(short('a'), long)]
    alternate_direction: bool,
}

fn main() {
    let args = Args::parse();
    let repeater = args.repeat.into();
    let password = generate_password(&args.password, repeater, args.alternate_direction);
    println!("{}", password);
}
