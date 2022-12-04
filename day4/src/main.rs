use common;

fn main() {
    let input_path = common::get_filepath_arg().unwrap();
    let input_vec = match common::read_file_lines::<String>(input_path.as_str()) {
        Ok(contents) => contents,
        Err(error) => panic!("Problem reading the file contents: {:?}", error),
    };

    let p1_result = part_one(&input_vec);
    println!("P1 Result: {:?}", p1_result);

    let p2_result = part_two(&input_vec);
    println!("P2 Result: {:?}", p2_result);
}

fn part_one(input: &Vec<String>) -> usize {
    let mut contained_pairs = 0;
    for pair in input.iter() {
        let left_right_split: Vec<&str> = pair.split(',').collect();
        let left_min_max: Vec<&str> = left_right_split[0].split('-').collect();
        let right_min_max: Vec<&str> = left_right_split[1].split('-').collect();

        let left_min: usize = left_min_max[0].parse().unwrap();
        let left_max: usize = left_min_max[1].parse().unwrap();
        let right_min: usize = right_min_max[0].parse().unwrap();
        let right_max: usize = right_min_max[1].parse().unwrap();

        if (left_min >= right_min && left_max <= right_max)
            || (right_min >= left_min && right_max <= left_max)
        {
            contained_pairs = contained_pairs + 1;
        }
    }

    contained_pairs
}

fn part_two(input: &Vec<String>) -> usize {
    let mut overlap_pairs = 0;
    for pair in input.iter() {
        let left_right_split: Vec<&str> = pair.split(',').collect();
        let left_min_max: Vec<&str> = left_right_split[0].split('-').collect();
        let right_min_max: Vec<&str> = left_right_split[1].split('-').collect();

        let left_min: usize = left_min_max[0].parse().unwrap();
        let left_max: usize = left_min_max[1].parse().unwrap();
        let right_min: usize = right_min_max[0].parse().unwrap();
        let right_max: usize = right_min_max[1].parse().unwrap();

        if (left_max >= right_min && left_min <= right_max)
            || (right_max >= left_min && right_min <= left_max)
        {
            overlap_pairs = overlap_pairs + 1;
        }
    }

    overlap_pairs
}

#[cfg(test)]
mod tests {
    use crate::part_two;

    #[test]
    fn part_2_example_test() {
        let inputs: Vec<String> = vec![
            "2-4,6-8".to_string(),
            "2-3,4-5".to_string(),
            "5-7,7-9".to_string(),
            "2-8,3-7".to_string(),
            "6-6,4-6".to_string(),
            "2-6,4-8".to_string(),
        ];

        assert_eq!(4, part_two(&inputs));
    }

    #[test]
    fn part_2_extended_test() {
        let inputs: Vec<String> = vec![
            "2-4,6-8".to_string(),
            "2-3,4-5".to_string(),
            "5-7,7-9".to_string(),
            "2-8,3-7".to_string(),
            "6-6,4-6".to_string(),
            "2-6,4-8".to_string(),
            "1-1,4-4".to_string(),
            "1-2,2-3".to_string(),
            "1-5,2-3".to_string(),
            "2-3,1-5".to_string(),
            "8-9,7-8".to_string(),
            "1-2,11-12".to_string(),
            "3-99,1-2".to_string(),
        ];

        assert_eq!(8, part_two(&inputs));
    }
}
