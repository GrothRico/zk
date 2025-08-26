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

fn init(zk_directory: &Option<std::path::PathBuf>) {
    let zk_working_dir_relative = match zk_directory {
        Some(directory) => {
            let _ = std::fs::create_dir(directory);
            directory.clone()
        }
        None => match std::env::current_dir() {
            Ok(path) => path,
            Err(_) => panic!("Cannot find zk directory"),
        },
    };
    let zk_working_dir_absolute = match std::fs::canonicalize(&zk_working_dir_relative) {
        Ok(path) => path,
        Err(_) => panic!("Cannot find zk directory"),
    };
    println!("{:?}", zk_working_dir_absolute)
}

fn main() {
    let args = Args::parse();
    match &args.cmd {
        Commands::Init { zk_directory } => init(zk_directory),
    }
}
