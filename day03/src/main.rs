use std::fs;

fn get_priority(item: char) -> usize {
    1 + ('a'..='z').chain('A'..='Z').collect::<String>().find(item).unwrap()
}

fn main() {
    let file_path = "./src/input.txt";
    //let file_path = "./src/example.txt";

    let bags = fs::read_to_string(file_path).unwrap();
    let bags: Vec<&str> = bags.trim().split("\n").collect();

    part_1(&bags);
    part_2(&bags);
}

fn part_1(bags: &Vec<&str>) {
    let mut sum = 0;

    for stuff in bags {
        let half_length = stuff.len() / 2;
        let pocket_1 = &stuff[0..half_length];
        for item in stuff[half_length..].chars() {
            if pocket_1.contains(item) { 
                sum += get_priority(item);
                break;
            }
        }
    }

    println!("{sum}");
}

fn part_2(bags: &Vec<&str>) {
    let mut sum = 0;

    for index in (0..bags.len()).step_by(3) {
        let one = bags[index];
        let two = bags[index + 1];
        let three = bags[index + 2];

        let mut one_plus_two = String::new();

        for item in one.chars() {
            if two.contains(item) { 
                one_plus_two.push(item);
            }
        }

        for item in three.chars() {
            if one_plus_two.contains(item) { 
                sum += get_priority(item);
                break;
            }
        }
    }

    println!("{sum}");
}
