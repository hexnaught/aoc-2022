use std::collections::VecDeque;

const INPUT: &str = include_str!("../../inputs/d11-p1.txt");

fn main() {
    println!("{}", INPUT);

    let p1_res = part_one(INPUT);
    println!("Part 1: {:?}", p1_res);

    let p2_res = part_two(INPUT);
    println!("Part 2: {:?}", p2_res);
}

fn part_one(input: &str) -> u64 {
    let mut monkeys: Vec<Monkey> = vec![];

    for monkey in input.split("\n\n") {
        monkeys.push(Monkey::new(monkey, 3));
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let inspected_item = &monkeys[i].inspect(0);
                let target = &monkeys[i].get_throw_target(&inspected_item);
                monkeys[*target].items.push_back(*inspected_item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));
    monkeys[0].inspected * monkeys[1].inspected
}

fn part_two(input: &str) -> u64 {
    let mut monkeys: Vec<Monkey> = vec![];

    for monkey in input.split("\n\n") {
        monkeys.push(Monkey::new(monkey, 1));
    }

    let modulo: u64 = monkeys.iter().map(|m| m.target_cond).product();
    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let inspected_item = &monkeys[i].inspect(modulo);
                let target = &monkeys[i].get_throw_target(&inspected_item);
                monkeys[*target].items.push_back(*inspected_item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));
    monkeys[0].inspected * monkeys[1].inspected
}

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Add(u64),
    Sub(u64),
    Multi(u64),
    MultiSelf,
}

#[derive(Debug, Eq, PartialEq)]
struct Monkey {
    inspected: u64,
    items: VecDeque<u64>,
    op: Operation,
    target_cond: u64,
    target_true: usize,
    target_false: usize,
    worry_div_by: usize,
}

impl Monkey {
    pub fn new(monkey_descriptor: &str, worry_div_by: usize) -> Self {
        let mut lines = monkey_descriptor.lines();

        lines.next().unwrap();

        let items: VecDeque<u64> = lines
            .next()
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .into_iter()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        let op_str = lines
            .next()
            .unwrap()
            .strip_prefix("  Operation: new = old ")
            .unwrap();
        let op = match op_str.chars().next().unwrap() {
            '+' => Operation::Add(op_str.strip_prefix("+ ").unwrap().parse::<u64>().unwrap()),
            '-' => Operation::Sub(op_str.strip_prefix("+ ").unwrap().parse::<u64>().unwrap()),
            '*' => match op_str.ends_with(" old") {
                true => Operation::MultiSelf,
                false => {
                    Operation::Multi(op_str.strip_prefix("* ").unwrap().parse::<u64>().unwrap())
                }
            },
            _ => unreachable!(),
        };

        println!("{}, {:?}", op_str, op);

        let target_cond: u64 = lines
            .next()
            .unwrap()
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap();

        let target_true: usize = lines
            .next()
            .unwrap()
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();
        let target_false: usize = lines
            .next()
            .unwrap()
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        Monkey {
            inspected: 0,
            items,
            op,
            target_cond,
            target_true,
            target_false,
            worry_div_by,
        }
    }

    fn inspect(&mut self, modulo: u64) -> u64 {
        let item = self.items.pop_front().unwrap();
        let new_item = match self.op {
            Operation::Add(n) => item + n,
            Operation::Sub(n) => item - n,
            Operation::Multi(n) => item * n,
            Operation::MultiSelf => item * item,
        };

        self.inspected = self.inspected + 1;

        if self.worry_div_by == 1 {
            new_item % modulo
        } else {
            new_item / self.worry_div_by as u64
        }
    }

    fn get_throw_target(&self, item: &u64) -> usize {
        if item % self.target_cond == 0 {
            self.target_true
        } else {
            self.target_false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    #[test]
    fn example_part_one() {
        let input: &str = "Monkey 0
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
        let expected = 10605;
        let result = part_one(&input);

        assert_eq!(expected, result);
    }

    #[test]
    fn example_part_two() {
        let input: &str = "Monkey 0
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
        let expected = 2713310158;
        let result = part_two(&input);

        assert_eq!(expected, result);
    }
}
