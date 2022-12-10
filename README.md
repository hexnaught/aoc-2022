# Advent of Code 2022

## What is Advent of Code?

Advent of Code is an 'Advent Calendar of programming challenges', where by a challenge is unlocked each day through December as an input/output puzzle.

See more at https://adventofcode.com/

## Language Choice

Will be attempting to do this in Rust, as it is a language I want to learn but haven't really dove in to much.

A lot of the code here is going to be sub-optimal and not performant... I may go back and optimize things but I am using this as an excuse to explore the Rust language, features, standard library and way of doing things. I may start using TDD and exploring how to structure projects as I go further in this Repo.

## Day Solution Scaffold


### Create solution binary project in workspace
Create binary with cargo

```sh
$ cargo new --bin day0
```

### Add the project to the workspace and import common lib

Edit the Workspace `Cargo.toml` (in root) to add the project (`day0`) to the workspace members.

Add the common lib as a dependency for the binary project in `day0/Cargo.toml`.

```toml
[dependencies]
common = { path = "../common" }
```

### `main.rs` file contents to get started

```rust
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

fn part_one(input: &Vec<String>) {}

fn part_two(input: &Vec<String>) {}
```

### Run the solution

```bash
$ cargo run --bin day0 inputs/solution-input-file.txt
```
