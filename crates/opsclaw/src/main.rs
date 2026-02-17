use clap::Parser;

#[derive(Parser)]
#[command(name = "opsclaw", version)]
struct Cli {
    #[arg(long)]
    verbose: bool,
}

fn main() {
    let _cli = Cli::parse();
}
