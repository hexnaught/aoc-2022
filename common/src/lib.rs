use std::str::FromStr;

pub fn read_file_lines<T: FromStr>(file_name: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse())
        .collect()
}
