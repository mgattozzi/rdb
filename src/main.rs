use structopt::*;
use std::path::PathBuf;
use std::process::{self, Command};
use std::fs;
use std::thread;

type Error = Box<std::error::Error>;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    let args = Args::from_args();
    let program = find_progam(&args)?;

    // We only care about lldb for now!
    let db = Command::new("rust-lldb").arg(program).spawn()?.wait();

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

fn find_progam(args: &Args) -> Result<PathBuf, Error> {
    let mut path = args.root.clone();
    let program = args.prog.clone();

    path.push("target");
    path.push("debug");
    path.push("deps");

    for entry in fs::read_dir(&path)? {
        let entry = entry?.file_name();

        // If the file is a . whatever file ignore it
        if entry.to_str().ok_or("Failed to make OsString into &str")?.contains(".") {
            continue;
        }

        path.push(entry);
        break;
    }

    Ok(path)
}
