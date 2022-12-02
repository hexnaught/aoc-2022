use std::fmt::Error;
use std::str::FromStr;
use std::env;

pub fn read_file_lines<T: FromStr>(file_name: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse())
        .collect()
}

pub fn get_filepath_arg() -> Result<String, Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 as usize {
        eprintln!("invalid number of arguments");
        return Err(Error);
    }
    Ok(args[1].clone())
}
