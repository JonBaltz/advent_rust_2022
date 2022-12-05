use std::fs;

fn inside_other(one: &Vec<usize>, two: &Vec<usize>) -> bool {
    one[0] >= two[0] && two[1] >= one[1] ||
    two[0] >= one[0] && one[1] >= two[1]
}

fn overlaps(one: &Vec<usize>, two: &Vec<usize>) -> bool {
    one[1] >= two[0] && two[1] >= one[0]
}

fn main() {
    let file_path = "./src/input.txt";
    //let file_path = "./src/example.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    let pairs: Vec<Vec<Vec<usize>>> = contents.trim().lines()
        .map(|line| {
            line.split(",")
                .map(|half| {
                    half.split("-")
                        .map(|digit| { digit.parse::<usize>().unwrap() }).collect()
                }).collect()
        }).collect();

    part_1(&pairs);
    part_2(&pairs);
}

fn part_1(pairs: &Vec<Vec<Vec<usize>>>) {
    let count = pairs.iter().fold(0, |acc, pair| {
        acc + if inside_other(&pair[0], &pair[1]) { 1 } else { 0 }
    });

    println!("{count}");
}

fn part_2(pairs: &Vec<Vec<Vec<usize>>>) {
    let count = pairs.iter().fold(0, |acc, pair| {
        acc + if overlaps(&pair[0], &pair[1]) { 1 } else { 0 }
    });

    println!("{count}");
}
