use std::fs;
use std::ops::Index;
use std::collections::HashMap;

struct Sbeve {
    wins: String,
    ties: String,
    value: usize,
}

fn build_sbeve(key: &str, wins: &str, ties: &str, value: usize) -> (String, Sbeve) {
    (String::from(key),
    Sbeve {
        wins: String::from(wins),
        ties: String::from(ties),
        value
    })
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let mapping = HashMap::from([
        build_sbeve("X", "C", "A", 1),
        build_sbeve("Y", "A", "B", 2),
        build_sbeve("Z", "B", "C", 3),
    ]);

    let file_path = "./src/input.txt";
    //let file_path = "./src/example.txt";

    let strategy = fs::read_to_string(file_path).unwrap();
    let strategy = strategy.lines()
        .map(|line| { line.split(" ").collect::<Vec<&str>>() })
        .collect::<Vec<Vec<&str>>>();

    let mut score = 0;

    strategy.iter().for_each(|step| {
        let item_map = &mapping[step[1]];
        score += item_map.value;

        if item_map.wins == step[0] {
            score += 6;
        } else if item_map.ties == step[0] {
            score += 3;
        }
    });

    println!("{score}");
}

struct Sbove {
    Z: usize,
    Y: usize,
    X: usize,
}

impl Index<&'_ str> for Sbove {
    type Output = usize;
    fn index(&self, s: &str) -> &usize {
        match s {
            "X" => &self.X,
            "Y" => &self.Y,
            "Z" => &self.Z,
            _ => panic!("unknown field: {}", s),
        }
    }
}

fn build_sbove(key: &str, Z: usize, Y: usize, X: usize) -> (String, Sbove) {
    (String::from(key),
    Sbove { Z, Y, X })
}

fn part2() {
    let mapping = HashMap::from([
        build_sbove("A", 8, 4, 3),
        build_sbove("B", 9, 5, 1),
        build_sbove("C", 7, 6, 2),
    ]);

    let file_path = "./src/input.txt";
    //let file_path = "./src/example.txt";

    let strategy = fs::read_to_string(file_path).unwrap();
    let strategy = strategy.trim().split("\n")
        .map(|line| { line.split(" ").collect::<Vec<&str>>() })
        .collect::<Vec<Vec<&str>>>();

    let mut score = 0;

    strategy.iter().for_each(|step| {
        let item_map = &mapping[step[0]];
        score += item_map[step[1]];
    });

    println!("{score}");
}
