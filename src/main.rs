use std::{env, fs, path::PathBuf, process::ExitCode};

fn main() -> ExitCode {
    let mut args = env::args_os().skip(1);
    let Some(path) = args.next().map(PathBuf::from) else {
        eprintln!("usage: cedar <markdown-file>");
        return ExitCode::from(2);
    };
    match fs::read_to_string(&path) {
        Ok(input) => {
            println!("{}", cedar::render(&input));
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("cedar: {}: {error}", path.display());
            ExitCode::from(1)
        }
    }
}
