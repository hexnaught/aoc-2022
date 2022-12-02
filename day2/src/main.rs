use common;
use std::str::FromStr;

// A | X is rock
// B | Y is paper
// C | Z is scissors

// X is lose
// Y is draw
// Z is win

// Score per shape, Rock = 1, Paper = 2, Scissors = 3
// Score per game, Lost = 0, Draw = 3, Win = 6

enum RPSShape {
    Rock,
    Paper,
    Scissors
}

impl RPSShape {
    fn score(&self) -> i32 {
        return match self {
            RPSShape::Rock => 1,
            RPSShape::Paper => 2,
            RPSShape::Scissors => 3
        };
    }

    fn to_lose(&self) -> RPSShape {
        return match self {
            RPSShape::Rock => RPSShape::Scissors,
            RPSShape::Paper => RPSShape::Rock,
            RPSShape::Scissors => RPSShape::Paper
        };
    }

    fn to_win(&self) -> RPSShape {
        return match self {
            RPSShape::Rock => RPSShape::Paper,
            RPSShape::Paper => RPSShape::Scissors,
            RPSShape::Scissors => RPSShape::Rock
        };
    }

    fn to_draw(&self) -> RPSShape {
        return match self {
            RPSShape::Rock => RPSShape::Rock,
            RPSShape::Paper => RPSShape::Paper,
            RPSShape::Scissors => RPSShape::Scissors
        };
    }

    fn outcome_required(&self, result_needed: &str) -> RPSShape {
        return match result_needed {
            "X" => self.to_lose(),
            "Y" => self.to_draw(),
            "Z" => self.to_win(),
            _ => self.to_draw()
        }
    }

    fn versus(&self, other: &RPSShape) -> i32 {
        // return match (self, other) {
        //     (RPSShape::Rock, RPSShape::Scissors) => 6,
        //     (_, _) if self.score() > other.score() => 6,
        //     (_, _) if self.score() == other.score() => 3,
        //     (_, _) if self.score() < other.score() => 0,
        //     _ => 0 // realistically this would be an error thrown and handled?
        // }
        return match (self, other) {
            (RPSShape::Rock, RPSShape::Rock) => 3,
            (RPSShape::Rock, RPSShape::Paper) => 0,
            (RPSShape::Rock, RPSShape::Scissors) => 6,
            (RPSShape::Paper, RPSShape::Rock) => 6,
            (RPSShape::Paper, RPSShape::Paper) => 3,
            (RPSShape::Paper, RPSShape::Scissors) => 0,
            (RPSShape::Scissors, RPSShape::Rock) => 0,
            (RPSShape::Scissors, RPSShape::Paper) => 6,
            (RPSShape::Scissors, RPSShape::Scissors) => 3,
        };
    }
}

#[derive(Debug)]
struct ShapeScoreError;
impl FromStr for RPSShape {
    type Err = ShapeScoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "A" => Ok(RPSShape::Rock),
            "B" => Ok(RPSShape::Paper),
            "C" => Ok(RPSShape::Scissors),
            "X" => Ok(RPSShape::Rock),
            "Y" => Ok(RPSShape::Paper),
            "Z" => Ok(RPSShape::Scissors),
            _ => Err(ShapeScoreError)
        };
    }
}

fn main() {
    let input_path = common::get_filepath_arg().unwrap();
    let input_vec = match common::read_file_lines::<String>(input_path.as_str()) {
        Ok(contents) => contents,
        Err(error) => panic!("Problem reading the file contents: {:?}", error),
    };

    let mut total_score = 0;
    let mut rigged_score = 0;
    for round in input_vec {
        let round_shapes = round.split_once(" ").unwrap();

        let opponent_shape = RPSShape::from_str(round_shapes.0).expect("invalid shape thrown");
        let player_shape = RPSShape::from_str(round_shapes.1).expect("invalid shape thrown");

        // Part 1 Score
        let round_score = determine_round_score(&opponent_shape, &player_shape);
        total_score = total_score + round_score;

        // Part 2 Score
        let rigged_round_score = determine_round_score(
            &opponent_shape,
            &opponent_shape.outcome_required(round_shapes.1),
        );
        rigged_score = rigged_score + rigged_round_score;
    }

    println!("Part 1: {:?}", total_score);
    println!("Part 2: {:?}", rigged_score);
}

fn determine_round_score(opponent_shape: &RPSShape, player_shape: &RPSShape) -> i32 {
    let mut player_score = 0;

    player_score += player_shape.score();
    player_score += player_shape.versus(&opponent_shape);

    return player_score;
}
