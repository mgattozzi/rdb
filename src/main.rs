use std::fs;
use std::path::PathBuf;
use std::process::{self, Command};
use structopt::*;

type Error = Box<std::error::Error>;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let args = Args::from_args();
    let program = find_progam(args)?;

    // We only care about lldb for now!
    let _ = Command::new("rust-lldb").arg(program).spawn()?.wait();

    Ok(())
}

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "rdb", about = "A wrapper to make debugging Rust pleasant")]
struct Args {
    /// Program to process
    #[structopt(name = "PROGRAM")]
    prog: String,

    /// Root directory for target dir
    #[structopt(name = "ROOT_DIR", parse(from_os_str), default_value = "")]
    root: PathBuf,
}

fn find_progam(args: Args) -> Result<PathBuf, Error> {
    let mut path = args.root;
    let program = args.prog;

    path.push("target");
    path.push("debug");
    path.push("deps");

    let mut entries = Vec::new();
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let name = entry.file_name();

        // If the file is a .whatever file or not our program's name ignore it
        let n = name.to_str().ok_or("Failed to make OsString into &str")?;

        if n.contains(".") || !n.contains(&program) {
            continue;
        }

        entries.push((name, entry.metadata()?.created()?));
    }

    let (file, time) = entries.pop().ok_or(
        "No file found. Is the target dir correct? Did you compile your code at least once?",
    )?;

    let mut file = file;
    for (f, t) in entries {
        if t > time {
            file = f;
        }
    }

    path.push(file);

    Ok(path)
}
