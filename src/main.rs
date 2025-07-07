use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author = "Olive Casazza", version, about)]
/// Application configuration
struct Args {
    /// whether to be verbose
    #[arg(short = 'v')]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
