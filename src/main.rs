use serde::{Deserialize, Serialize};
use std::{error::Error, io::Error, path::PathBuf};

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
    #[arg(long, short = 'd', global = true)]
    zk_directory: Option<std::path::PathBuf>,
}

#[derive(Subcommand, Debug, Clone)]
#[command(version = "0.0.1")]
#[command(author = "Rico Groth")]
#[command(about = "Zettelkasten command-line tool")]
enum Commands {
    Init {
        zk_directory: Option<std::path::PathBuf>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    test: i32,
}

fn init_zk_working_dir(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let zk_conf_path = path.join(PathBuf::from(".zk.json"));
    std::fs::File::create(zk_conf_path).and_then(|config_file| {
        let init_config = Config { test: 5 };
        serde_json::to_writer_pretty(config_file, &init_config)
    })
}

fn init(zk_directory: Option<std::path::PathBuf>) -> Result<(), Error> {
    match zk_directory {
        Some(path) => Ok(path),
        None => std::env::current_dir(),
    }
    .and_then(std::fs::canonicalize)
    .and_then(init_zk_working_dir)
}

fn main() {
    let args = Args::parse();
    match args.cmd {
        Commands::Init { zk_directory } => {
            let _init_res = init(zk_directory);
        }
    };
}
