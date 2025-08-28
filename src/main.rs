use serde::{Deserialize, Serialize};
use std::{ffi::OsStr, io::Read, path::PathBuf, process::Command, time::UNIX_EPOCH};

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
    New {
        #[arg(long, short = 'n')]
        name: String,
        #[arg(long, short = 'r')]
        root: Option<String>,
        #[arg(long, short = 'i', default_value_t = false)]
        interactive: bool,
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

fn get_env_var(key: &str, or: &str) -> String {
    match std::env::var(key) {
        Ok(env_var) => env_var,
        Err(_) => or.into(),
    }
}

fn user_edit_temp_file() -> Result<String, Box<dyn std::error::Error>> {
    let temp_dir = std::env::temp_dir();
    let pid = std::process::id();
    let now = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs();
    let temp_filepath = temp_dir.join(PathBuf::from(format!("{}_{}.md", pid, now)));
    let editor = get_env_var("EDITOR", "vi");
    let _ = Command::new(editor).arg(&temp_filepath).status()?;
    let mut temp_file = std::fs::File::open(&temp_filepath)?;
    let mut buffer = String::new();
    let _ = temp_file.read_to_string(&mut buffer)?;
    let _ = std::fs::remove_file(temp_filepath)?;
    Ok(buffer)
}

fn new(
    _name: String,
    _root: Option<String>,
    interactive: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // let file = std::fs::File::create_new(format!("{}.md", &name))?;
    if interactive {
        let user_content = user_edit_temp_file()?;
        println!("{}", user_content)
    }
    Ok(())
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
        Commands::New {
            name,
            root,
            interactive,
        } => match new(name, root, interactive) {
            Ok(_) => println!("new done success"),
            Err(_) => eprintln!("new error"),
        },
    };
}
