use std::fs;

fn main() {
    let file_path = "./src/input.txt";
    //let file_path = "./src/example.txt";
    let content = fs::read_to_string(file_path).unwrap();

    let moves: Vec<Vec<&str>> = content.lines()
        .map(|line| line.split(" ").collect())
        .collect();

    part_1(&moves);
    part_2(&moves);
}

fn simulate(moves: &Vec<Vec<&str>>, mut rope: Vec<(i32, i32)>) {
    let length = rope.len();
    let mut tail_moves: Vec<String> = vec![];

    moves.iter().for_each(|m| {
        let move_length: usize = m[1].parse().unwrap();
        for _ in 0..move_length {
            match m[0] {
                "R" => rope[0].0 += 1,
                "L" => rope[0].0 -= 1,
                "U" => rope[0].1 += 1,
                "D" => rope[0].1 -= 1,
                _ => panic!("bad move"),
            }

            for rope_index in 0..length - 1 {
                let lead = rope[rope_index];
                let mut follow = &mut rope[rope_index + 1];

                if (lead.0 - follow.0).abs() > 1 {
                    if lead.0 > follow.0 { 
                        follow.0 += 1;
                    } else {
                        follow.0 -= 1;
                    }
                    if lead.1 > follow.1 {
                        follow.1 += 1;
                    } else if lead.1 < follow.1 {
                        follow.1 -= 1;
                    }
                } else if (lead.1 - follow.1).abs() > 1 {
                    if lead.1 > follow.1 { 
                        follow.1 += 1;
                    } else {
                        follow.1 -= 1;
                    }
                    if lead.0 > follow.0 {
                        follow.0 += 1;
                    } else if lead.0 < follow.0 {
                        follow.0 -= 1;
                    }
                }
            }

            tail_moves.push(format!("{},{}", rope[length - 1].0, rope[length - 1].1));
        }
    });

    tail_moves.sort();
    tail_moves.dedup();
    let count = tail_moves.len();

    println!("{count}");
}

fn part_1(moves: &Vec<Vec<&str>>) {
    let rope: Vec<(i32, i32)> = vec![(0, 0); 2];

    simulate(moves, rope);
}

fn part_2(moves: &Vec<Vec<&str>>) {
    let rope: Vec<(i32, i32)> = vec![(0, 0); 10];

    simulate(moves, rope);
}
