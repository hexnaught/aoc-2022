use common;

fn main() {
    let input_path = common::get_filepath_arg().unwrap();
    let input_vec = match common::read_file_lines::<String>(input_path.as_str()) {
        Ok(contents) => contents,
        Err(error) => panic!("Problem reading the file contents: {:?}", error),
    };

    let p1_res = part_one(&input_vec);
    println!("Part 1: {:?}", p1_res);

    let p2_res = part_two(&input_vec);
    println!("Part 2: {:?}", p2_res);
}

fn part_one(input: &Vec<String>) -> usize {
    let mut total = 0;

    for sack in input {
        let (cmp_1, cmp_2) = split_rucksack(sack.as_str());
        let common = get_common_items(cmp_1, cmp_2);
        total = total + sum_item_priorities(common);
    }
    total
}

fn part_two(input: &Vec<String>) -> usize {
    let mut total = 0;

    for i in (0..input.len()).step_by(3) {
        let e1_e2_common = get_common_items(input[i].as_str(), input[i + 1].as_str())
            .into_iter()
            .collect::<String>();
        let e1e2_e3_common = get_common_items(e1_e2_common.as_str(), input[i + 2].as_str());

        total = total + sum_item_priorities(e1e2_e3_common);
    }

    total
}

fn sum_item_priorities(items: Vec<char>) -> usize {
    let r = items.iter().map(|item| get_priority(item)).sum::<usize>();
    r
}

fn get_common_items(cmp_1: &str, cmp_2: &str) -> Vec<char> {
    let mut common_items: Vec<char> = vec![];
    let mut cmp_arr: [usize; 26 * 2] = [0; 26 * 2];

    for item in cmp_1.chars() {
        cmp_arr[get_priority(&item) - 1] = 1;
    }

    for item in cmp_2.chars() {
        // TODO: This contains check is sub-optimal, hashmap/set or better solution in general
        if cmp_arr.get(get_priority(&item) - 1).unwrap_or(&0) == &(1 as usize)
            && !common_items.contains(&item)
        {
            common_items.push(item.clone());
        }
    }

    common_items
}

fn split_rucksack(sack: &str) -> (&str, &str) {
    sack.split_at(sack.len() / 2)
}

fn get_priority(item: &char) -> usize {
    // TODO: This could be statically defined for performance
    let alphabet: Vec<char> = ('a'..='z')
        .chain('A'..='Z')
        .into_iter()
        .collect::<Vec<char>>();
    alphabet.iter().position(|&x| x == *item).unwrap_or(0) + 1
}

#[cfg(test)]
mod tests {
    use crate::{get_common_items, get_priority, part_one, part_two, split_rucksack};

    #[test]
    fn example_split_sacks() {
        let inputs: Vec<&str> = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];
        let expected_results: Vec<(&str, &str)> = vec![
            ("vJrwpWtwJgWr", "hcsFMMfFFhFp"),
            ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"),
            ("PmmdzqPrV", "vPwwTWBwg"),
            ("wMqvLMZHhHMvwLH", "jbvcjnnSBnvTQFn"),
            ("ttgJtRGJ", "QctTZtZT"),
            ("CrZsJsPPZsGz", "wwsLwLmpwMDw"),
        ];

        for (idx, input) in inputs.iter().enumerate() {
            let expected = expected_results.get(idx).unwrap().to_owned();
            let actual = split_rucksack(input);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn example_item_priority() {
        let inputs: Vec<char> = vec!['a', 'm', 'z', 'A', 'M', 'Z'];
        let expected_results: Vec<usize> = vec![1, 13, 26, 27, 39, 52];

        for (idx, input) in inputs.iter().enumerate() {
            let expected = expected_results.get(idx).unwrap().to_owned();
            let actual = get_priority(input);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn can_get_common_items() {
        let inputs: Vec<(&str, &str)> = vec![
            ("abcDefGhijK", "ajKQWERTYZX"),
            ("abcd", "abcd"),
            ("abcd", "qwer"),
        ];
        let expected_results: Vec<Vec<char>> =
            vec![vec!['a', 'j', 'K'], vec!['a', 'b', 'c', 'd'], vec![]];

        for (idx, (left_cmp, right_cmp)) in inputs.iter().enumerate() {
            let expected = expected_results.get(idx).unwrap().to_owned();
            let actual = get_common_items(left_cmp, right_cmp);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn part_one_test() {
        let input: Vec<String> = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ];

        assert_eq!(157, part_one(&input))
    }

    #[test]
    fn part_two_test() {
        let input: Vec<String> = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ];

        assert_eq!(70, part_two(&input))
    }
}
