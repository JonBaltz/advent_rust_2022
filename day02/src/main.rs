use std::fs;

#[derive(PartialEq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn move_from_str(s: &str) -> Move {
    match s {
        "A" | "X" => Move::Rock,
        "B" | "Y" => Move::Paper,
        _ => Move::Scissors,
    }
}
fn winning_move(m: &Move) -> Move {
    match m {
        Move::Rock => Move::Paper,
        Move::Paper => Move::Scissors,
        Move::Scissors => Move::Rock,
    }
}

fn losing_move(m: &Move) -> Move {
    match m {
        Move::Rock => Move::Scissors,
        Move::Paper => Move::Rock,
        Move::Scissors => Move::Paper,
    }
}

fn move_from_round_end(enemy: &str, hero: &str) -> Move {
    let enemy_move = move_from_str(enemy);
    match hero {
        "X" => losing_move(&enemy_move),
        "Z" => winning_move(&enemy_move),
        _ => enemy_move,
    }
}

fn main() {
    let file_path = "./src/input.txt";
    //let file_path = "./src/example.txt";

    let guide = fs::read_to_string(file_path).unwrap();

    part_1(&guide);
    part_2(&guide);
}

fn part_1(guide: &str) {
    let guide = guide.trim().lines()
        .map(|line| {
            line.split(" ")
                .map(|s| { move_from_str(s) })
                .collect()
        }).collect::<Vec<Vec<Move>>>();

    let mut score = 0;

    for game in guide {
        score += match game[1] {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };

        if game[1] == winning_move(&game[0]) {
            score += 6;
        } else if game[0] == game[1] { score += 3; }
    }

    println!("{score}");
}

fn part_2(guide: &str) {
    let guide = guide.trim().lines()
        .map(|line| {
            line.split(" ").collect()
        }).collect::<Vec<Vec<&str>>>();

    let mut score = 0;

    for game in guide {
        score += match move_from_round_end(game[0], game[1]) {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };

        score += match game[1] {
            "Z" => 6,
            "Y" => 3,
            _ => 0,
        };
    }

    println!("{score}");
}
