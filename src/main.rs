use serde::{Deserialize, Serialize};
use std::{ffi::OsStr, io::Read, path::PathBuf, process::Command, time::UNIX_EPOCH};

use clap::{Parser, Subcommand};

type DynRes<T> = Result<T, Box<dyn std::error::Error>>;

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

fn create_tempfile_path() -> DynRes<PathBuf> {
    let temp_dir = std::env::temp_dir();
    let pid = std::process::id();
    let now = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs();
    Ok(temp_dir.join(PathBuf::from(format!("{}_{}", pid, now))))
}

fn use_editor(pre_existing_content: Option<String>) -> DynRes<String> {
    let temp_filepath = create_tempfile_path()?;
    if let Some(c) = pre_existing_content {
        std::fs::write(&temp_filepath, &c)?;
    }
    let editor = get_env_var("EDITOR", "vi");
    Command::new(editor).arg(&temp_filepath).status()?;
    let buffer = std::fs::read_to_string(&temp_filepath)?;
    std::fs::remove_file(&temp_filepath)?;
    Ok(buffer)
}

fn zk_workdir(global_arg: &Option<PathBuf>) -> DynRes<PathBuf> {
    if let Some(dir) = global_arg {
        return Ok(dir.to_path_buf());
    }
    let cwd = std::env::current_dir()?;
    if cwd.join(PathBuf::from(".zk.json")).exists() {
        return Ok(cwd);
    }
    let error = std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Cannot find current zk working directory.",
    );
    Err(error.into())
}

fn new(
    name: String,
    _root: Option<String>,
    interactive: bool,
    zk_directory: Option<PathBuf>,
) -> DynRes<()> {
    let header = format!("# {}", &name);
    let content = match interactive {
        true => use_editor(Some(header))?,
        false => header,
    };
    let filepath = PathBuf::from(format!("{}.md", &name)).join(zk_workdir(&zk_directory)?);
    std::fs::write(&filepath, &content)?;
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
            Err(e) => panic!("{}", e),
        },
        Commands::New {
            name,
            root,
            interactive,
        } => match new(name, root, interactive, args.zk_directory) {
            Ok(_) => println!("new done success"),
            Err(e) => panic!("new error, {}", e),
        },
    };
}
