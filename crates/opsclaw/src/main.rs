use clap::{Parser, Subcommand};
mod skill_install;

#[derive(Parser)]
#[command(name = "opsclaw", version)]
struct Cli {
    #[arg(long)]
    verbose: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Skill {
        #[command(subcommand)]
        command: SkillCommands,
    },
}

#[derive(Subcommand)]
enum SkillCommands {
    Install { path: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Skill {
            command: SkillCommands::Install { path },
        }) => match skill_install::install_skill_to_default_location(path.as_ref()) {
            Ok(installed_path) => {
                println!("installed skill at {}", installed_path.display());
            }
            Err(err) => {
                eprintln!("skill install failed: {err}");
                std::process::exit(1);
            }
        },
        None => {
            if cli.verbose {
                println!("opsclaw: no subcommand provided");
            }
        }
    }
}
