use std::fs;

fn main() {
    let file_path = "./src/input.txt";
    //let file_path = "./src/example.txt";

    let elves: Vec<Vec<usize>> = fs::read_to_string(file_path).unwrap()
        .trim().split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|item| {
                    item.parse().unwrap()
                }).collect()
        }).collect();

    part_1(&elves);
    part_2(&elves);
}

fn part_1(elves: &Vec<Vec<usize>>) {
    let max = elves.iter().fold(0, |max, elf| {
        let sum = elf.iter().sum();
        if sum > max { sum } else { max }
    });

    println!("{max}");
}

fn part_2(elves: &Vec<Vec<usize>>) {
    let mut elf_sums: Vec<usize> = elves.iter().map(|elf| {
        elf.iter().sum()
    }).collect();

    elf_sums.sort_by(|a, b| b.cmp(a));

    let mut max_three_elves = 0;
    for elf_dex in 0..3 {
        max_three_elves += elf_sums[elf_dex];
    }

    println!("{max_three_elves}");
}
