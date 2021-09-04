mod errors;

use std::fs::File;
use std::fs::OpenOptions;
use std::env;
use std::io::{Write, BufReader, BufRead, Error};
use crate::errors::Errors;
use std::path::Path;
use std::env::args;

fn main() {
    match do_command().err() {
        None => {}
        Some(x) => { eprintln!("{}", x)}
    }
}

fn do_command() -> Result<(), Errors> {
    let args: Vec<String> = args().collect();
    let mut dir = env::current_dir().unwrap();
    if args.len() > 4 || args.len() < 3 {
        return Err(Errors::InvalidArgsException);
    }
    let mut input = Path::new(&args[1]);
    if !input.exists() {
        dir.push(&args[1]);
        input = dir.as_path();
    }
    if input.is_dir() {
        return Err(Errors::FileIsDirError);
    }
    if !input.exists() {
        return Err(Errors::FileNotExistsError);
    }
    let new: String = if args.len() == 3 {
        String::from("")
    } else {
        args[3].clone()
    };
    let str = read_replace(&input, &args[2], &new).or(Err(Errors::GeneralError))?;
    write(input, &str).or(Err(Errors::GeneralError))?;
    Ok(())
}

fn read_replace(input: &Path, old: &String, new: &String) -> Result<String, Error> {
    let file = File::open(input)?;
    let buffered = BufReader::new(file);
    let mut content = String::from("");
    buffered.lines().into_iter().filter(|it| {
        it.is_ok()
    }).map(|x| {
        x.unwrap()
    }).for_each(|x| {
        content.push_str(&x);
        content.push_str("\n");
    });
    let res = content[..content.len() - 1].to_owned();
    return Ok(res.replace(old, new))
}

fn write(input: &Path, res: &String) -> Result<(), Error> {
    let mut output = OpenOptions::new().write(true).open(input)?;
    write!(output, "{}", res)?;
    Ok(())
}