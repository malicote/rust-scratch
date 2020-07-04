use std::io;
use std::fs;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct CliArgs {
    /// The filepath to read
    #[structopt(parse(from_os_str))]
    root: std::path::PathBuf,
}

fn visit_dirs<F>(dir: &Path, depth: usize, cb: F) -> io::Result<()>
    where F: Fn(&Path, usize, &Option<PathBuf>) + Copy
{
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        let target = path.read_link().ok();

        cb(&path, depth, &target);
        if path.is_dir() && target.is_none(){
            visit_dirs(&path, depth + 1, cb)?;
        }

    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args = CliArgs::from_args();
    let root = args.root.as_path();

    println!("{}", root.display());

    let result = visit_dirs(&root, 0, |path: &Path, depth: usize, target: &Option<PathBuf>| {
        let filename = path.file_name().unwrap().to_string_lossy();
        match target {
            Some(target) => println!("{}|-- {} -> {}", "|\t".repeat(depth), filename, target.display()),
            None => println!("{}|-- {}", "|\t".repeat(depth), filename),
        }
    });

    match result {
        Err(error) => println!("Error traversing directory {}: {}", root.display(), error),
        Ok(_) => (),
    }

    Ok(())
}
