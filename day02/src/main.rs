use std::io::BufRead;
use std::fs::File;
use std::io;

#[derive(PartialEq, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn beats(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }

    fn loses(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }

    fn shape_score(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn play_to(&self, result: &GameResult) -> Hand {
        match (self, result) {
            (Hand::Rock, GameResult::Win) => Hand::Rock.loses(),
            (Hand::Rock, GameResult::Lose) => Hand::Rock.beats(),
            (Hand::Rock, GameResult::Draw) => Hand::Rock,
            (Hand::Paper, GameResult::Win) => Hand::Paper.loses(),
            (Hand::Paper, GameResult::Lose) => Hand::Paper.beats(),
            (Hand::Paper, GameResult::Draw) => Hand::Paper,
            (Hand::Scissors, GameResult::Win) => Hand::Scissors.loses(),
            (Hand::Scissors, GameResult::Lose) => Hand::Scissors.beats(),
            (Hand::Scissors, GameResult::Draw) => Hand::Scissors
        }
    }
}

struct Game {
    other: Hand,
    result: GameResult,
}

impl Game {
    fn own(&self) -> Hand {
        self.other.play_to(&self.result)
    }

    fn score(&self) -> u32 {
        let shape_score = self.own().shape_score();

        let (own_beats, other_beats) = (self.own().beats(), self.other.beats());

        let match_score = match (own_beats, other_beats) {
            _ if own_beats == self.other => 6,
            _ if other_beats == self.own() => 0,
            _ => 3
        };

        shape_score + match_score
    }
}

fn main() {
    println!("Hello, world!");
    let reader = io::BufReader::new(File::open("input.txt").expect("Cannot open input.txt"));

    let mut tourney = vec![];
    for line in reader.lines() {
        let game_text = line.unwrap();
        let moves : Vec<&str> = game_text.split(" ").collect();
        let game = Game {
            other: parse_move(moves[0]),
            result: parse_desired_result(moves[1]),
        };

        tourney.push(game);
    }

    let mut score = 0;
    for game in tourney {
        println!("{}", game.score());
        score = score + game.score()
    }

    println!("{}", score)
}

fn parse_move(s :&str) -> Hand {
    match s {
        "A" => Hand::Rock,
        "B" => Hand::Paper,
        "C" => Hand::Scissors,
        _ => unreachable!()
    }
}

enum GameResult {
    Win,
    Lose,
    Draw
}

fn parse_desired_result(s : &str) -> GameResult {
    match s {
        "X" => GameResult::Lose,
        "Y" => GameResult::Draw,
        "Z" => GameResult::Win,
        _ => unreachable!()
    }
}
