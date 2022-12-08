use std::fs;

fn main() {
    let file_path = "./src/input.txt";
    //let file_path = "./src/example.txt";
    
    let signal = fs::read_to_string(file_path).unwrap();

    part_1(&signal);
    part_2(&signal);
}

fn part_1(signal: &str) {
    for index in 0..signal.len() {
        let mut unique: Vec<char> = signal[index..index + 4].chars().collect();
        unique.sort();
        unique.dedup();

        if unique.len() == 4 {
            println!("{}", index + 4);
            break;
        }
    }
}

fn part_2(signal: &str) {
    for index in 0..signal.len() {
        let mut unique: Vec<char> = signal[index..index + 14].chars().collect();
        unique.sort();
        unique.dedup();

        if unique.len() == 14 {
            println!("{}", index + 14);
            break;
        }
    }
}
