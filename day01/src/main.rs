use std::fs::File;
use std::io::{self, BufRead};

// Elf can carry Calories
struct Elf {
    food: Vec<i32>
}

impl Elf {
    fn total_calories(&self) -> i32 {
        self.food.iter().sum()
    }
    
    fn add_food(&mut self, calories: i32) {
        self.food.push(calories);
    }
}

fn new_elf() -> Elf {
    Elf { food: vec![] }
}

struct Expedition {
    elfs: Vec<Elf>
}

fn new_expedition() -> Expedition {
    Expedition { elfs: vec![] }
}

impl Expedition {
    fn highest_calories(&self) -> i32 {
        self.elfs
            .iter()
            .map(|e| e.total_calories())
            .max()
            .unwrap()
    }

    fn top_three_sum(&self) -> i32 {
        let mut calories : Vec<i32> = self.elfs.iter().map(|e| e.total_calories()).collect();

        calories.sort_by(|a, b| b.cmp(a));
        calories.truncate(3);

        calories.iter().sum()
    }

    fn add_elf(&mut self, elf: Elf) {
        self.elfs.push(elf)
    }
}

fn main() {
    let reader = io::BufReader::new(File::open("input.txt").expect("Cannot open input.txt"));

    let mut elf = new_elf();
    let mut expedition = new_expedition();

    for line in reader.lines() {
        let unwrapped_line = line.unwrap();
        if unwrapped_line.is_empty() {
            expedition.add_elf(elf);
            elf = new_elf();
            continue;
        } 

        let calories: i32 = unwrapped_line.parse().unwrap();
        elf.add_food(calories);
    }

    println!("{}", expedition.highest_calories());
    println!("{}", expedition.top_three_sum());
}
