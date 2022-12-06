use std::fs;

fn get_top_items(stacks: Vec<Vec<&char>>) -> String {
    stacks.iter().map(|stack| {
        *stack[stack.len() - 1]
    }).collect()
}


fn main() {
    let file_path = "./src/input.txt";
    //let file_path = "./src/example.txt";

    let contents = fs::read_to_string(file_path).unwrap();
    let input: Vec<&str> = contents.split("\n\n").collect();

    part_1(&input);
    part_2(&input);
}

fn part_1(input: &Vec<&str>) {
    let one: Vec<&str> = input[0].lines().collect();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); (one[0].len() + 1) / 4];

    one.iter().for_each(|line| {
        for (index, digit) in line.chars().enumerate() {
            if digit.is_uppercase() {
                stacks[(index + 2) / 4].push(digit);
            }
        }
    });
    let mut stacks: Vec<Vec<&char>> = stacks.iter().map(|stack| {
        stack.iter().rev().collect()
    }).collect();

    let steps: Vec<Vec<usize>> = input[1].lines()
        .map(|step| {
            step.split(" ")
                .filter_map(|word| { word.parse::<usize>().ok() })
                .collect()
        }).collect();

    steps.iter().for_each(|step| {
        for _ in 0..step[0] {
            let temp = stacks[step[1] as usize - 1].pop().unwrap();
            stacks[step[2] as usize - 1].push(temp);
        }
    });

    println!("{}", get_top_items(stacks));
}

fn part_2(input: &Vec<&str>) {
    let one: Vec<&str> = input[0].lines().collect();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); (one[0].len() + 1) / 4];

    one.iter().for_each(|line| {
        for (index, digit) in line.chars().enumerate() {
            if digit.is_uppercase() {
                stacks[(index + 2) / 4].push(digit);
            }
        }
    });
    let mut stacks: Vec<Vec<&char>> = stacks.iter().map(|stack| {
        stack.iter().rev().collect()
    }).collect();

    let steps: Vec<Vec<usize>> = input[1].lines()
        .map(|step| {
            step.split(" ")
                .filter_map(|word| { word.parse::<usize>().ok() })
                .collect()
        }).collect();

    steps.iter().for_each(|step| {
        let from = &mut stacks[step[1] as usize - 1];
        let tail = from.split_off(from.len() - step[0]);
        stacks[step[2] as usize - 1].extend(tail);
    });

    println!("{}", get_top_items(stacks));
}
