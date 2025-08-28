use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

fn init_zk_working_dir<'a>(path: PathBuf) -> Result<PathBuf, std::io::Error> {
    let zk_conf_path = path.join(PathBuf::from(".zk.json"));
    std::fs::File::create(&zk_conf_path).and_then(|file| {
        let init_config = Config { test: 5 };
        match serde_json::to_writer_pretty(file, &init_config) {
            Ok(_) => Ok(path),
            Err(ser_err) => match std::fs::remove_file(&zk_conf_path) {
                Ok(_) => Err(ser_err.into()),
                Err(rm_file_err) => {
                    let error_msg = format!(
                        "Serialization error: {}\n\nRemove file error: {}",
                        &ser_err, &rm_file_err
                    );
                    Err(std::io::Error::new(std::io::ErrorKind::Other, error_msg))
                }
            },
        }
    })
}

fn init(zk_directory: Option<std::path::PathBuf>) -> Result<PathBuf, std::io::Error> {
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
        Commands::Init { zk_directory } => match init(zk_directory) {
            Ok(init_dir) => println!(
                "zk project created in {}",
                init_dir.as_os_str().to_str().unwrap()
            ),
            Err(e) => eprintln!("{}", e),
        },
    };
}
