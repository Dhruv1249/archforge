use builder::BuildType;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::config::load_config;

mod builder;
mod config;
mod profile;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Build {
        #[arg(short, long)]
        build_type: Option<BuildType>,

        #[arg(short, long)]
        output_dir: Option<PathBuf>,
    },
    Scan,
    Tui,
    Profile,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Build {
            build_type,
            output_dir,
        }) => {
            if let Some(bt) = build_type {
                println!("{:?}", bt);
            }

            if let Some(dir) = output_dir {
                println!("{:?}", dir);

                match load_config(&dir) {
                    Ok(config) => println!("{:?}", config),
                    Err(e) => println!("{:?}", e),
                }
            }
        }
        Some(Commands::Scan) => println!("alright we gonna scan something today"),
        Some(Commands::Tui) => println!("alright we gonna use tui today"),
        Some(Commands::Profile) => println!("alright we gonna see them profile"),
        None => println!("No commands were given"),
    }
}
