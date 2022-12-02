use common;

fn main() {
    let input_path = common::get_filepath_arg().unwrap();
    let input_vec = match common::read_file_lines::<String>(input_path.as_str()) {
        Ok(contents) => contents,
        Err(error) => panic!("Problem reading the file contents: {:?}", error),
    };

    let mut index_total_vec: Vec<i32> = vec![];

    let mut elf_index = 0;
    let mut total_for_elf = 0;
    for calorie in input_vec.iter() {
        if calorie.is_empty() {
            index_total_vec.push(total_for_elf);
            elf_index = elf_index + 1;
            total_for_elf = 0;
            continue;
        }

        let v: i32 = calorie.parse().unwrap_or(0);
        total_for_elf = total_for_elf + v;
    }

    index_total_vec.sort();

    let top_three_elves_total = index_total_vec.get(index_total_vec.len() - 1).unwrap_or(&0)
        + index_total_vec.get(index_total_vec.len() - 2).unwrap_or(&0)
        + index_total_vec.get(index_total_vec.len() - 3).unwrap_or(&0);

    println!("{:?}", top_three_elves_total)
}
